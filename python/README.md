# Deephaven Enterprise Python probe

`enterprise_client.py` connects to a Deephaven **Enterprise (Core+)** server the
supported way — authenticate to the auth server, find the Persistent Query that
holds your table, connect to its worker, and print snapshots so you can see the
table tick.

It's intentionally verbose: it prints the auth/session objects' methods and the
Persistent Query list, so its output is also the reference for porting the
Enterprise auth + worker-discovery flow into the Rust client.

## Why this (and not raw PSK/Basic to the worker)

Enterprise workers don't accept raw PSK or username/password directly — that's
why connecting straight to the worker port (e.g. 24005) returns
`Authentication details invalid`. You authenticate to the **auth server**, which
issues a token the workers trust. `deephaven_enterprise.SessionManager` does this.

## Install

The Enterprise client wheel comes from your server, not public PyPI:

```bash
python -m venv .venv && . .venv/Scripts/activate    # Windows PowerShell: .venv\Scripts\Activate.ps1
pip install --extra-index-url https://nspra00a0005.wellsfargo.com:8123/iris/coreplus/pip deephaven-coreplus-client
```

(The exact index URL varies by deployment — your Deephaven/infra team can
confirm it. It's usually under `…/iris/…` on the web port.)

## Run

```powershell
$env:DH_USER="u674256"
$env:DH_PASSWORD="your-password"
python enterprise_client.py --host nspra00a0005.wellsfargo.com --web-port 8123 --table executions
```

- If it lists Persistent Queries, re-run with `--pq "<the PQ name that holds executions>"`.
- `--seconds 0` prints a single snapshot; otherwise it polls every `--interval` seconds.
- `--connection-url` lets you pass the full `…/iris/connection.json` URL directly.

## What to send back

Paste the `[probe]` output — especially:
- the `SessionManager`/`Session`/`ControllerClient` member lists,
- the Persistent Query list,
- and whether `open_table` succeeded.

That tells me exactly which calls the Rust client must make (auth-server login →
token → controller → worker) to support Enterprise natively.
