#!/usr/bin/env python3
"""
Deephaven Enterprise (Core+) connection probe.

Goal: connect to a Deephaven Enterprise server, reach the Persistent Query that
holds a live table, open the table, and print snapshots so you can see it tick.

It is deliberately verbose: it prints every step, the auth/session objects'
available methods, and the list of Persistent Queries — so its output doubles as
the reference for porting the Enterprise auth+worker-discovery flow to the Rust
client.

Enterprise authentication is NOT raw PSK/Basic against the worker. You log in to
the *auth server* (username/password), which issues a token; the controller then
routes you to the PQ's worker, which trusts that token. This is what the
`deephaven_enterprise.SessionManager` does.

Install the client (from your Enterprise server's wheel, not public PyPI):
    pip install --extra-index-url https://<server>/iris/coreplus/pip deephaven-coreplus-client
    # (exact URL varies; your Deephaven/infra team can confirm)

Usage (PowerShell):
    $env:DH_USER="<username>"; $env:DH_PASSWORD="<password>"
    python enterprise_client.py --connection-url https://<enterprise-host>:<web-port>/iris/connection.json --table <table>

Or pass --host and let it build the connection.json URL:
    python enterprise_client.py --host <enterprise-host> --web-port <web-port> --table <table>
"""

import argparse
import os
import sys
import time


def log(msg):
    print(f"[probe] {msg}", flush=True)


def show_api(label, obj):
    """Print the public methods/attributes of an object (API discovery)."""
    members = [m for m in dir(obj) if not m.startswith("_")]
    log(f"{label} ({type(obj).__module__}.{type(obj).__name__}) members: {members}")


def dump_token_like(obj, label):
    """Print any attributes that look like a token/cookie/credential (handoff)."""
    for name in dir(obj):
        if name.startswith("_"):
            continue
        if any(k in name.lower() for k in ("token", "cookie", "cred")):
            try:
                val = getattr(obj, name)
                if callable(val):
                    log(f"  {label}.{name}() -> callable (not invoked)")
                else:
                    log(f"  {label}.{name} = {val!r}")
            except Exception as e:
                log(f"  {label}.{name} access error: {e}")


def deep_scan(obj, label, depth=0, max_depth=3, seen=None):
    """Walk the session object graph and surface anything that looks like an
    auth token or a worker host/port — the values the Rust client needs.
    Recurses only into Deephaven/pydeephaven objects to stay bounded."""
    if seen is None:
        seen = set()
    if id(obj) in seen or depth > max_depth:
        return
    seen.add(id(obj))
    for name in dir(obj):
        if name.startswith("__"):
            continue
        lname = name.lower()
        try:
            val = getattr(obj, name)
        except Exception:
            continue
        if callable(val):
            continue
        if isinstance(val, (str, bytes, int)):
            if any(k in lname for k in ("token", "cookie", "cred")):
                log(f"  TOKEN  {label}.{name} = {val!r}")
            elif any(k in lname for k in ("host", "port", "target", "uri", "url")):
                log(f"  ADDR   {label}.{name} = {val!r}")
        mod = getattr(type(val), "__module__", "") or ""
        if (mod.startswith("pydeephaven") or mod.startswith("deephaven")) and depth < max_depth:
            deep_scan(val, f"{label}.{name}", depth + 1, max_depth, seen)


def main():
    p = argparse.ArgumentParser(description="Deephaven Enterprise connection probe")
    p.add_argument("--host", help="Server host, e.g. dh-enterprise.example.com")
    p.add_argument("--web-port", default="8123", help="Web/Envoy port (for connection.json). Default 8123")
    p.add_argument("--connection-url", help="Full connection.json URL (overrides --host/--web-port)")
    p.add_argument("--user", default=os.environ.get("DH_USER"), help="Username (or DH_USER env)")
    p.add_argument("--password", default=os.environ.get("DH_PASSWORD"), help="Password (or DH_PASSWORD env)")
    p.add_argument("--table", default="executions", help="Table name to open (default: executions)")
    p.add_argument("--pq", help="Persistent Query name that holds the table (optional; auto-listed if omitted)")
    p.add_argument("--seconds", type=int, default=20, help="How long to poll snapshots (0 = one snapshot)")
    p.add_argument("--interval", type=float, default=2.0, help="Seconds between snapshots")
    args = p.parse_args()

    if args.connection_url:
        connection_url = args.connection_url
    elif args.host:
        connection_url = f"https://{args.host}:{args.web_port}/iris/connection.json"
    else:
        log("ERROR: provide --connection-url or --host")
        sys.exit(2)

    if not args.user or not args.password:
        log("ERROR: provide --user/--password (or DH_USER/DH_PASSWORD env)")
        sys.exit(2)

    log(f"connection.json URL : {connection_url}")
    log(f"user                : {args.user}")
    log(f"table               : {args.table}")

    # --- Import the Enterprise client -------------------------------------
    try:
        from deephaven_enterprise.client.session_manager import SessionManager
    except Exception as e:
        log(f"Could not import deephaven_enterprise: {e}")
        log("Install the Core+ client wheel from your Enterprise server (see the")
        log("docstring at the top of this file), then re-run.")
        sys.exit(1)

    # --- Authenticate to the auth server ----------------------------------
    log("Creating SessionManager (downloads connection.json, finds auth server)...")
    sm = SessionManager(connection_url)
    show_api("SessionManager", sm)

    log("Authenticating with username/password to the auth server...")
    # Method name has been stable as `password`, but print API above in case it
    # differs in your version (e.g. `private_key`, `saml`, etc.).
    sm.password(args.user, args.password)
    log("Authenticated. (The auth server issued a token the workers will trust.)")

    # --- Find the Persistent Query and connect to its worker --------------
    pq_name = args.pq
    if pq_name is None:
        log("Listing Persistent Queries to find the one holding the table...")
        try:
            # Controller client lists PQ configs. API name may vary by version.
            controller = sm.controller_client if hasattr(sm, "controller_client") else None
            if controller is None and hasattr(sm, "get_controller_client"):
                controller = sm.get_controller_client()
            if controller is not None:
                show_api("ControllerClient", controller)
                # Common: map() / get_serializable_configurations() of PQ infos
                infos = None
                for meth in ("map", "get_serializable_configurations", "get"):
                    if hasattr(controller, meth):
                        try:
                            infos = getattr(controller, meth)()
                            break
                        except Exception as ie:
                            log(f"controller.{meth}() failed: {ie}")
                log(f"Persistent Queries: {infos}")
        except Exception as e:
            log(f"Could not list Persistent Queries: {e}")
        log("If a PQ name above holds your table, re-run with --pq <name>.")

    log("Connecting to the Persistent Query worker...")
    session = None
    # Try the common connection methods; print which one worked.
    for meth, kwargs in (
        ("connect_to_persistent_query", {"name": pq_name} if pq_name else {}),
        ("connect_to_new_worker", {}),
    ):
        if pq_name is None and meth == "connect_to_persistent_query":
            continue
        if hasattr(sm, meth):
            try:
                log(f"Trying SessionManager.{meth}({kwargs})...")
                session = getattr(sm, meth)(**kwargs)
                log(f"Connected via {meth}.")
                break
            except Exception as e:
                log(f"{meth} failed: {e}")

    if session is None:
        log("Could not establish a worker session. Inspect the SessionManager")
        log("members printed above and adjust (e.g. pass --pq <name>).")
        sys.exit(1)

    show_api("Session", session)

    # --- Handoff info for the Rust client ---------------------------------
    # We want: the real worker host:port, the CA cert, and the auth token the
    # worker accepts — so the Rust client can connect to the worker directly.
    log("=== Rust handoff: worker address + auth token ===")
    log("Lines tagged TOKEN are candidate DH_AUTH values; ADDR lines are the")
    log("worker host/port. Use them with the Rust `ticking` example (see below).")
    deep_scan(sm, "sm")
    deep_scan(session, "session")
    log("Rust command (run promptly — the token is session-scoped and expires):")
    log('  $env:DH_AUTH="<the TOKEN value above>"; $env:DH_TLS_INSECURE="1"; $env:DH_SECONDS=20')
    log('  cargo run --example ticking -- https://<ADDR host>:<ADDR port> ' + args.table)

    # --- Open the table and print snapshots -------------------------------
    log(f"Opening table '{args.table}'...")
    table = session.open_table(args.table)
    show_api("Table", table)

    def take_snapshot():
        # This Core+ table exposes an Arrow snapshot; fall back to the session's
        # barrage snapshot if the method name differs.
        if hasattr(table, "to_arrow"):
            return table.to_arrow()
        if hasattr(session, "barrage_snapshot"):
            return session.barrage_snapshot(table).to_arrow()
        if hasattr(session, "snapshot"):
            return session.snapshot(table).to_arrow()
        raise RuntimeError("No snapshot method found; see the Table members above")

    arrow = take_snapshot()
    log(f"Columns: {[(f.name, str(f.type)) for f in arrow.schema]}")
    log(f"Initial rows: {arrow.num_rows}")

    if args.seconds <= 0:
        n = arrow.num_rows
        print(arrow.slice(max(0, n - 5)).to_pandas())
        return

    deadline = time.time() + args.seconds
    while time.time() < deadline:
        arrow = take_snapshot()
        n = arrow.num_rows
        tail = arrow.slice(max(0, n - 1)).to_pylist()
        log(f"rows={n}  last={tail}")
        time.sleep(args.interval)

    log("Done.")


if __name__ == "__main__":
    main()
