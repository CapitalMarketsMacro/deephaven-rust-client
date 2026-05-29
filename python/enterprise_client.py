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
    $env:DH_USER="u674256"; $env:DH_PASSWORD="your-password"
    python enterprise_client.py --connection-url https://nspra00a0005.wellsfargo.com:8123/iris/connection.json --table executions

Or pass --host and let it build the connection.json URL:
    python enterprise_client.py --host nspra00a0005.wellsfargo.com --web-port 8123 --table executions
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


def main():
    p = argparse.ArgumentParser(description="Deephaven Enterprise connection probe")
    p.add_argument("--host", help="Server host, e.g. nspra00a0005.wellsfargo.com")
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

    # --- Open the table and print snapshots -------------------------------
    log(f"Opening table '{args.table}'...")
    table = session.open_table(args.table)
    log(f"Table opened. Columns: {[(c.name, c.type) for c in table.columns]}")

    def snapshot_rows():
        arrow = table.to_arrow()
        return arrow.num_rows, arrow

    if args.seconds <= 0:
        rows, arrow = snapshot_rows()
        log(f"Snapshot: {rows} rows")
        print(arrow.slice(max(0, rows - 5)).to_pandas())
        return

    deadline = time.time() + args.seconds
    while time.time() < deadline:
        rows, arrow = snapshot_rows()
        tail = arrow.slice(max(0, rows - 1)).to_pylist()
        log(f"rows={rows}  last={tail}")
        time.sleep(args.interval)

    log("Done.")


if __name__ == "__main__":
    main()
