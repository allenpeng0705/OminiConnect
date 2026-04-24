# Try the managed (Nango) connector flow

OminiConnect stays the **surface**; **Nango** runs OAuth and holds tokens. End users never need the Nango *operator* dashboard for this path (operators still use it to **enable** integrations in the environment).

## Prerequisites

- Nango running with API reachable at `NANGO_BASE_URL` (e.g. `http://localhost:3003`).
- Repo-root `.env`: `NANGO_BASE_URL`, `NANGO_SECRET_KEY` (see `.env.example`; use `./scripts/sync_nango_secret_to_omini_env.sh` if needed).
- In **Nango** (dashboard): create an **environment**, enable at least one **integration** (e.g. GitHub) so it appears under Environment → Integrations.

## Steps (happy path)

1. **Start** OminiConnect + Nango (e.g. `./scripts/dev_omini_connect_nango_native.sh` or your stack).
2. **Sign in** to the OminiConnect portal (`PORTAL_BASE_URL`, default `http://localhost:9000`).
3. **Add a managed connector**
   - Dashboard → **Add from library** (or `/connectors/add-managed`).
   - Choose a **connector id** (slug used in `/api/proxy/{slug}/…`).
   - Choose the **integration unique key** that matches what you enabled in Nango (often same as provider id, e.g. `github`).
4. **Connect OAuth (in the OminiConnect portal)**
   - Dashboard → **Connect (managed)** on that card, or open **Configure** → **Connect in OminiConnect**. Both go to `/connectors/{slug}/connect`.
   - The portal loads **Nango Connect** inside the page (iframe, with **Open sign-in in new tab** if a provider blocks framing). The address bar stays on OminiConnect.
5. **Finalize** (store `connection_id` in OminiConnect)
   - On that same page, click **Save connection**, or open `/oauth/{your-connector-slug}/nango-finalize` from the connector’s **Configure** page.
   - You should be redirected home; status becomes **Connected**.
6. **Call through OminiConnect**  
   Use your OminiConnect API key and `POST /api/proxy/{slug}/…` — the portal forwards to Nango for `engine=nango` connectors.

## API used by the in-portal connect page

- `POST /api/nango/connect-session` with JSON `{ "platform": "<connector-slug>" }` (session cookie required).  
  Returns `{ "connect_url": "…" }` — Nango Connect URL; the hub page embeds it (or opens it in a new tab).

## Troubleshooting

- **Empty iframe**: use **Open sign-in in new tab** on `/connectors/{slug}/connect`, or visit `GET /oauth/{platform}` for a full-page Nango Connect redirect if you prefer that escape hatch.
- **Finalize says no connection**: complete OAuth in Connect first; `end_user` id is stable per connector (`omini_connect_portal_{platform}`).
- **502 on connect-session**: Nango down, wrong secret, or integration key not enabled in Nango.

For architecture context, see `docs/omini_connect_nango_option_b.md`.
