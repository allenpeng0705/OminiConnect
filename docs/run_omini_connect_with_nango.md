# Run and test OminiConnect with Nango

This guide describes the **native** dev stack: Nango from `third_party/nango` + the OminiConnect portal, with PostgreSQL for Nango (and optionally for the portal).

For variable semantics and deployment notes, see **`.env.example`** in the repo root.

---

## What you are starting

| Component | Role | Typical URL |
|-----------|------|----------------|
| **Nango** (Node, from submodule) | OAuth hub, Connect UI, dashboard | API **http://localhost:3003**, Connect UI **http://localhost:3009**, dashboard **http://localhost:3000** |
| **OminiConnect portal** (Rust) | Operator UI, proxy, API keys | **http://localhost:9000** (see `PORTAL_BASE_URL` in `.env`) |

**Data:**

- **Nango** requires **PostgreSQL** (its own database, e.g. `nango`).
- **Portal** can use **SQLite** (default file under `omini_connect/portal/`) or a **second** Postgres database (e.g. `omini_connect_portal`) on the same server.

---

## 1. Prerequisites

| Tool | Notes |
|------|--------|
| **Rust** | `cargo` on `PATH` |
| **Node.js** | **22+** (see `third_party/nango/.nvmrc`; use nvm/fnm if needed) |
| **npm** | With Node |
| **PostgreSQL** | Listening on **127.0.0.1:5432** (or set URLs accordingly) |
| **OpenSSL** | Used if the dev script generates `third_party/nango/.env` (encryption key) |
| **curl** | Used by `scripts/dev_omini_connect_nango_native.sh` for Nango `/health` |
| **`pg_isready`** (optional) | Dev script uses it when available |

---

## 2. Clone and initialize the Nango submodule

```bash
cd /path/to/OminiConnect
git submodule update --init --recursive
```

You should have `third_party/nango/packages/server` and the rest of the Nango tree. If init fails, fix git/network access and retry.

---

## 3. Prepare PostgreSQL

Nango needs its own DB. The portal should use a **different** database name if you use Postgres for it too.

Example (adjust `-U postgres` / host if your install differs):

```bash
createuser nango 2>/dev/null || true
psql -U postgres -c "ALTER USER nango WITH PASSWORD 'nango';" 2>/dev/null || true
createdb -O nango nango 2>/dev/null || true
createdb -O nango omini_connect_portal 2>/dev/null || true
```

- **`nango`** — Nango application data.
- **`omini_connect_portal`** — OminiConnect portal only (optional if you use SQLite for the portal).

---

## 4. Repo-root `.env`

```bash
cp .env.example .env
```

Set at least:

1. **`PORTAL_BASE_URL=http://localhost:9000`**  
   Must match where you open the UI; the portal binds **port 9000** by default.

2. **Portal database** — choose one:
   - **Postgres (closer to production):**  
     `DATABASE_URL=postgres://nango:nango@127.0.0.1:5432/omini_connect_portal`
   - **SQLite (minimal):** leave **`DATABASE_URL` unset** (or empty); the portal uses `omini_connect/portal/portal.db` by default.

3. **Nango HTTP from the portal:**  
   ```env
   NANGO_BASE_URL=http://localhost:3003
   ```
   **`NANGO_SECRET_KEY`:** leave empty for first run if you want **`make dev`** to auto-fill it from Nango’s DB (see [Nango API secret](#7-nango-api-secret-nango_secret_key)). You can also paste a key from the Nango UI.

See **`.env.example`** for full comments (K8s, multiple instances, etc.).

---

## 5. Nango’s `third_party/nango/.env`

Path: **`third_party/nango/.env`**.

- On **first** run, **`scripts/dev_omini_connect_nango_native.sh`** may **create** this file with `NANGO_DATABASE_URL` / `RECORDS_DATABASE_URL` pointing at **`127.0.0.1`** — correct for **native** Postgres.
- If you previously used **Docker** Nango, an existing `.env` might reference host **`nango-db`**. That host does not resolve on the host machine for native dev. For native dev, use **`127.0.0.1`** for DB URLs, for example:
  - `NANGO_DATABASE_URL=postgresql://nango:nango@127.0.0.1:5432/nango`
  - `RECORDS_DATABASE_URL=postgresql://nango:nango@127.0.0.1:5432/nango`
  - `NANGO_DB_HOST=127.0.0.1`, `NANGO_DB_PORT=5432`, etc.

Keep **`NANGO_ENCRYPTION_KEY`** stable for a given Nango database; changing it without a fresh DB breaks decryption.

---

## 6. Build the portal frontend

Required for a working SPA (run after clone or when frontend changes):

```bash
cd omini_connect/portal/frontend
npm ci
npm run build
cd ../../..
```

---

## 7. Nango API secret (`NANGO_SECRET_KEY`)

- Nango stores a **UUID-shaped environment API secret** in its Postgres (encrypted with `NANGO_ENCRYPTION_KEY`). There is **no universal fixed “magic number”** for all installs.
- **Without the Nango UI:** from repo root:
  ```bash
  ./scripts/sync_nango_secret_to_omini_env.sh
  ```
  This reads Nango’s DB using **`third_party/nango/.env`** and writes **`NANGO_SECRET_KEY`** into repo-root **`.env`**. If there are no Nango environments yet, it can seed a minimal dev account.
- **CI / scripts (stdout only, no `.env` write):**
  ```bash
  ./scripts/sync_nango_secret_to_omini_env.sh --print
  ```
- **`make dev`** runs the sync script automatically when **`NANGO_SECRET_KEY`** is empty in `.env` (after Nango is healthy).
- **Manual:** Nango dashboard → **Environment → API keys** (not provider client secrets).

---

## 8. (Optional) Compile the portal

```bash
cargo build -p omini-connect-portal
```

---

## 9. Start Nango + OminiConnect (`make dev`)

From **repository root**:

```bash
make dev
```

This runs **`scripts/dev_omini_connect_nango_native.sh`**, which roughly:

1. Verifies Node, npm, cargo (and optionally `pg_isready` against `NANGO_DATABASE_URL`, default `postgresql://nango:nango@127.0.0.1:5432/nango`).
2. Ensures **`third_party/nango/.env`** exists (creates it if missing).
3. Under **`third_party/nango`**: `npm ci` if needed, **`npm run ts-build`**, then **`npm run dev:watch:web`** (Nango API + webapp + Connect UI) in the background.
4. Waits for **`http://localhost:3003/health`**.
5. If repo **`.env`** exists and **`NANGO_SECRET_KEY`** is empty, runs **`./scripts/sync_nango_secret_to_omini_env.sh`**.
6. **`source .env`**, then **`cargo run -p omini-connect-portal`** (foreground).

**First run** can take several minutes (Nango `npm ci` + build). Later runs are faster.

**Ports:** keep **9000**, **3003**, **3000**, **3009** free or adjust configs.

**Stop:** **Ctrl+C** in that terminal; the script stops background Nango processes.

---

## 10. What to open

| URL | Purpose |
|-----|--------|
| **http://localhost:9000** | OminiConnect — login, connectors, API keys |
| **http://localhost:3003** | Nango HTTP API |
| **http://localhost:3000** | Nango dashboard (Vite) — integrations, API keys |
| **http://localhost:3009** | Nango Connect UI |

**Default portal login** (after seed): **`admin` / `admin`** — change for anything beyond local dev.

---

## 11. How to test

1. Open **http://localhost:9000** → log in as **`admin` / `admin`**.
2. For **Nango-backed connectors** (`engine=nango`), configure the **integration / provider** in the **Nango dashboard** so `NANGO_BASE_URL` accepts connect sessions and OAuth app settings match Nango’s expectations.
3. If Nango calls fail with missing auth, confirm **`NANGO_SECRET_KEY`** in `.env` and restart the portal (or re-run **`make dev`**).

### Integration library (“Open library”)

The UI loads **`GET /api/nango/providers`**, which calls Nango’s **`GET /providers`** with your **`NANGO_SECRET_KEY`**. If that returns **401** (wrong or stale secret), the portal falls back to Nango’s public **`GET /providers.json`** so you can still browse templates. Fix **`NANGO_SECRET_KEY`** (e.g. `./scripts/sync_nango_secret_to_omini_env.sh`) for full API behavior. If you see **401** on the API itself, sign in to the portal again (session cookie).

---

## 12. Portal only (no Nango)

```bash
cargo run -p omini-connect-portal
```

Ensure repo-root **`.env`** is loaded (portal reads **`../../.env`** from the crate). Nango-specific features need **`NANGO_BASE_URL`** and **`NANGO_SECRET_KEY`**.

---

## 13. Docker-based Nango (optional)

```bash
make dev-nango-docker        # OminiConnect + Nango via compose helper
make dev-nango-docker-up     # Nango only
```

Align **`NANGO_BASE_URL`** and **`third_party/nango/.env`** / secrets with the compose-published host and ports.

---

## 14. `make dev` pipeline (summary)

```text
Postgres (nango + optional omini_connect_portal)
    → third_party/nango/.env (create or reuse)
    → npm ci (if needed) + npm run ts-build + npm run dev:watch:web
    → wait :3003/health
    → optional sync NANGO_SECRET_KEY into repo .env
    → source .env + cargo run -p omini-connect-portal (:9000)
```

The portal loads **repo-root `.env`** at startup (`dotenvy` from `omini_connect/portal`), then connects to its DB, runs migrations, seeds admin, serves API + SPA.

---

## 15. Checklist

- [ ] `git submodule update --init --recursive`
- [ ] Postgres: database **`nango`** (and **`omini_connect_portal`** if not using SQLite for portal)
- [ ] `cp .env.example .env` — **`PORTAL_BASE_URL`**, **`DATABASE_URL`** if using Postgres for portal
- [ ] **`third_party/nango/.env`** — DB on **`127.0.0.1`** for native dev (not `nango-db`)
- [ ] `cd omini_connect/portal/frontend && npm ci && npm run build`
- [ ] From repo root: **`make dev`**
- [ ] Browser: **http://localhost:9000** — **`admin` / `admin`**

---

## Related files

| File | Purpose |
|------|--------|
| **`.env.example`** | All env vars and comments |
| **`Makefile`** | `make dev`, Docker helpers |
| **`scripts/dev_omini_connect_nango_native.sh`** | Native Nango + portal orchestration |
| **`scripts/sync_nango_secret_to_omini_env.sh`** | DB → `NANGO_SECRET_KEY` (and `--print`) |
| **`scripts/omini_connect_sync_repo_env.ts`** | Implementation (copied under Nango for `tsx`) |
