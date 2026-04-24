# OminiConnect

**A self-hosted API gateway that lets AI agents use connected services via OAuth.**

OminiConnect is a **Rust workspace** centered on **`omini_connect/portal`**: a web UI and Axum API that store OAuth tokens, issue API keys, and **proxy** requests to vendor APIs with tokens attached. For **integration catalog and Nango-backed flows** (Option B), the same portal can call a **self-hosted [Nango](https://github.com/NangoHQ/nango)** instance over HTTP using **`NANGO_BASE_URL`** and **`NANGO_SECRET_KEY`** — you **use** upstream Nango as a submodule; a small **optional patch** (`scripts/nango_omini_connect_local.patch`) only smooths local split-origin dev (dashboard vs API ports) and signup without SMTP until upstream or your deployment layout removes the need.

## How it works

```
AI agent / client
        │  Authorization: Bearer <OminiConnect API key>
        ▼
┌───────────────────────────────────────┐
│  OminiConnect portal                  │
│  (Rust + React, repo-root workspace)  │
│  • OAuth connect / token vault        │
│  • API keys, sessions, proxy routes   │
│  • Optional: Nango HTTP client        │
└───────────────┬───────────────────────┘
                │  When Nango is configured:
                │  NANGO_BASE_URL + NANGO_SECRET_KEY
                ▼
        ┌───────────────┐         ┌─────────────────┐
        │  Nango (opt.) │ ──────► │  Provider APIs  │
        │  self-hosted  │         │  (LinkedIn, …)  │
        └───────────────┘         └─────────────────┘
```

## Features

- **OAuth2 management**: Connect platforms (LinkedIn, Facebook, Feishu, DingTalk, WeChat Work, X, and more).
- **Token persistence** and **auto-refresh** where supported.
- **API key auth** for agents: Bearer token to the portal; portal injects stored OAuth tokens on proxy calls.
- **Passthrough proxy**: `POST/GET /api/proxy/{platform}/{path}` to native APIs.
- **Nango (Option B)**: Optional integration with self-hosted Nango for catalog / flows; see **Developing with Nango** below.

## Repository layout

```
OminiConnect/                    # Cargo workspace root (`cargo build`, `cargo run -p …`)
├── omini_connect/
│   ├── portal/                  # Main UI + API (primary product crate: omini-connect-portal)
│   ├── oauth_vault/             # Token storage / refresh helpers
│   ├── connectors/              # Optional MCP-style connectors (Feishu, Maton, …)
│   ├── sdk/, skills/, …         # Supporting crates
│   └── …
├── third_party/
│   └── nango/                   # Git submodule → NangoHQ/nango (not vendored source in Git)
├── scripts/                     # Nango setup, dev wrappers, secret sync, local patch
│   ├── ensure_nango.sh          # Submodule + patch + npm ci + ts-build
│   ├── dev_omini_connect_nango_native.sh
│   ├── dev_with_nango.sh        # Docker Compose Nango + portal
│   ├── repair_nango_submodule.sh
│   ├── snapshot_nango_patch.sh
│   ├── sync_nango_secret_to_omini_env.sh
│   └── nango_omini_connect_local.patch
├── docs/                        # e.g. omini_connect_nango_option_b.md, nango_upstream_strategy.md
├── panda/                       # Forked Panda crates (proxy / WASM plugins workspace members)
├── dashboard/                   # Additional workspace member
├── Makefile                     # `make dev`, `make dev-nango-docker`
├── .env.example                 # Portal + Nango env (loaded from repo root for portal)
└── .gitmodules                  # Submodule URL + pinned commit for third_party/nango
```

## Quick start

### Prerequisites

- **Rust** (stable), **cargo**
- For **Nango + Postgres dev**: **Node.js 22+**, **npm**, **PostgreSQL** locally
- For **Docker Nango**: **Docker** (see `make dev-nango-docker`)

### A — Full stack: OminiConnect + Nango (native Postgres, recommended for Nango features)

```bash
git clone --recurse-submodules <your-repo-url>
cd OminiConnect

cp .env.example .env
# Use a Postgres URL for the portal if you want Postgres (see .env.example comments).
# Create two DBs on the same Postgres if you like: one for Nango, one for the portal.

chmod +x scripts/ensure_nango.sh
./scripts/ensure_nango.sh

make dev
# Same as: ./scripts/dev_omini_connect_nango_native.sh
# Starts Nango (API :3003, dashboard :3000, Connect UI :3009) then `cargo run -p omini-connect-portal`.
```

First time, **`make dev`** may take several minutes while Nango runs **`npm ci`** / **`npm run ts-build`**. **`NANGO_SECRET_KEY`** in repo-root **`.env`** can be filled automatically when empty: **`./scripts/sync_nango_secret_to_omini_env.sh`** (see `.env.example`).

Open the **OminiConnect** UI at **`http://localhost:9000`** (or **`PORTAL_BASE_URL`**). Use the **Nango dashboard** at **`http://localhost:3000`** when developing with the native script.

### B — Docker Nango + portal

```bash
git clone --recurse-submodules <your-repo-url>
cd OminiConnect
cp .env.example .env
./scripts/ensure_nango.sh

make dev-nango-docker
# Same as: ./scripts/dev_with_nango.sh
```

### C — Portal only (no Nango submodule work)

You can run the portal without checking out Nango if you do not need Nango-backed features:

```bash
git clone <your-repo-url>   # submodules optional
cd OminiConnect
cp .env.example .env
# Leave NANGO_* unset or point NANGO_BASE_URL only if you have Nango elsewhere.

cargo run -p omini-connect-portal
```

Run from the **repository root** so workspace members and **`.env`** paths resolve correctly (the portal loads **`.env`** from the repo root per `.env.example`).

### After clone without `--recurse-submodules`

```bash
git submodule update --init --recursive
./scripts/ensure_nango.sh
```

## Supported platforms

| Platform | Type | Status | Notes |
|----------|------|--------|-------|
| LinkedIn | OAuth2 | ✅ | Posting, user info |
| Facebook | OAuth2 | ✅ | Page access tokens (App Review for broad posting) |
| X / Twitter | OAuth2 | ✅ | PKCE; credit-based API |
| Feishu / Lark | OAuth2 | ✅ | Custom apps (not Enterprise Internal) |
| DingTalk | OAuth2 | ✅ | |
| WeChat Work | OAuth2 | ✅ | |
| Maton.ai | API Key | ✅ | Agent connector |
| QQ Enterprise Mail | API Key | ✅ | |

### Platform notes

- **Facebook**: `pages_manage_posts` / `pages_read_engagement` often need [App Review](https://developers.facebook.com/docs/app-review) in production.
- **X**: PKCE; monthly limits apply.
- **Feishu**: Use **Custom App** type for user OAuth.

## Environment variables

The portal reads **repo-root `.env`** (see `.env.example` for full comments).

| Variable | Typical value | Description |
|----------|---------------|-------------|
| `PORTAL_BASE_URL` | `http://localhost:9000` | Public base URL for OAuth callbacks and links |
| `DATABASE_URL` | *(optional)* | Omit for default SQLite under `omini_connect/portal/`; or Postgres for the **portal** DB (separate from Nango’s DB) |
| `NANGO_BASE_URL` | `http://localhost:3003` | Nango **HTTP API** base (native dev) |
| `NANGO_SECRET_KEY` | *(from Nango UI or sync script)* | Nango **environment** API secret (Bearer), not per-user OAuth secrets |

**Nango’s own** configuration lives in **`third_party/nango/.env`** (created by `dev_omini_connect_nango_native.sh` on first run, or maintained by you). **`sync_nango_secret_to_omini_env.sh`** copies the dev secret into repo-root **`.env`** when **`NANGO_SECRET_KEY`** is empty.

## Developing with Nango

### Design (Option B)

- **OminiConnect** remains the product surface (UI, API keys, proxy). **Nango** is an **optional dependency**: same machine or cluster, reached over **HTTPS/HTTP** with **`NANGO_SECRET_KEY`**.
- **Two databases** in typical Postgres dev: one for Nango (`NANGO_DATABASE_URL` in `third_party/nango/.env`), one for the portal (`DATABASE_URL` in repo-root `.env`). See `.env.example`.
- **Submodule, not a fork in-tree**: `third_party/nango` tracks an upstream commit via **`.gitmodules`**. **`./scripts/ensure_nango.sh`** checks out the submodule, applies **`scripts/nango_omini_connect_local.patch`** when it still applies, installs deps, and builds TypeScript. Treat the patch as **local dev ergonomics** (CORS across `:3000` / `:3003`, optional email-verification skip); prefer **upstream fixes**, **Docker single-origin**, or **reverse proxy** long term if you want zero patch.

### Scripts (from repo root)

| Script | Purpose |
|--------|---------|
| `scripts/ensure_nango.sh` | Submodule init, patch, `npm ci` if needed, `ts-build` if needed (`NANGO_SKIP_*`, `NANGO_FORCE_BUILD` — see script header) |
| `scripts/dev_omini_connect_nango_native.sh` | Postgres + Nango from source + `cargo run -p omini-connect-portal` |
| `scripts/dev_with_nango.sh` | `docker-compose` in submodule + portal (`--nango-only` supported) |
| `scripts/repair_nango_submodule.sh` | Re-clone submodule if `.git` was removed; restores `.env` backup; runs `ensure_nango.sh` |
| `scripts/snapshot_nango_patch.sh` | Writes `nango_omini_connect_local.patch` from `git diff` in submodule; `--reset` cleans submodule + re-applies patch |
| `scripts/sync_nango_secret_to_omini_env.sh` | Fills **`NANGO_SECRET_KEY`** in repo-root `.env` from local Nango DB |
| `scripts/bootstrap_nango_submodule.sh` | Alias for **`ensure_nango.sh`** |

### What the local patch does

`scripts/nango_omini_connect_local.patch` adjusts **local** Nango dashboard + API split (Vite **3000**, API **3003**): CORS on loopback, optional **`NANGO_DEV_SKIP_EMAIL_VERIFICATION`** path for signup without SMTP, and clearer signup **403** handling in the webapp. Regenerate after editing inside `third_party/nango`:

```bash
cd third_party/nango && git diff > ../scripts/nango_omini_connect_local.patch
```

Or: **`./scripts/snapshot_nango_patch.sh`** (use **`--reset`** so the submodule is not left “dirty” for Git — see script).

### Git: what to commit

- **Commit** in **OminiConnect**: the patch file, `scripts/*`, `.gitmodules`, **README**, **`.env.example`** — not **`third_party/nango/.env`**, not **`node_modules`**, not **`scripts/.nango_omini_repair_backup/`** (gitignored).
- **Do not** commit ad-hoc edits **inside** the submodule as the main story; snapshot them into **`nango_omini_connect_local.patch`** or use your **fork** of Nango and bump the submodule pointer.

### Further reading

- `docs/try_managed_nango_flow.md` — **hands-on**: popup Nango Connect from OminiConnect, then finalize
- `docs/omini_connect_nango_option_b.md` — Option B architecture
- `docs/nango_upstream_strategy.md` — upstream / submodule strategy

## API reference

### Proxy

```
POST /api/proxy/{platform}/{path}
GET  /api/proxy/{platform}/{path}
```

**Auth:** `Authorization: Bearer <OminiConnect API key>`

The proxy validates the key, loads the stored OAuth token for that platform, forwards to the vendor API, and returns the response.

### Examples

```bash
curl -X POST http://localhost:9000/api/proxy/linkedin/v2/userinfo \
  -H "Authorization: Bearer YOUR_KEY"

curl -X POST http://localhost:9000/api/proxy/facebook/me?fields=id,name,email \
  -H "Authorization: Bearer YOUR_KEY"

curl -X POST http://localhost:9000/api/proxy/feishu/open-apis/contact/v3/users/me \
  -H "Authorization: Bearer YOUR_KEY"
```

## Security notes

- OAuth tokens and API keys are stored hashed; client secrets are not returned from the API.
- Session cookies: `HttpOnly`, `Secure` when on HTTPS, `SameSite` as configured by the portal.

## License

Private — All rights reserved
