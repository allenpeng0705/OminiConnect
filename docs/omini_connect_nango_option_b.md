# OminiConnect + Nango (Option B)

## Goal

Use **OminiConnect Portal as the only user-facing control plane** while reusing Nango as connector infrastructure for overlapping global providers.

- Tenant operators only use OminiConnect portal.
- Nango portal is internal/ops-only (or disabled in production tenant workflows).
- OminiConnect remains the product surface (branding, API, MCP, policy, SDK).

## Product Decision

### Control plane

- **OminiConnect portal owns all connector lifecycle**:
  - create/update connector configuration
  - connect OAuth
  - reconnect
  - test and health checks
  - disconnect
  - audit history

### Execution plane

- **Nango-backed connectors** for global APIs where Nango already has robust support.
- **OminiConnect native connectors** for China-first providers and custom gaps.

## Connector Engine Model

Add engine routing metadata per connector:

- `engine = "nango" | "omini_connect_native"`
- `auth_mode = "oauth2" | "api_key" | "oauth2_cc" | ...` (normalized)
- `provider_key` (e.g. `linkedin`, `facebook`, `twitter-v2`, `feishu`)
- `connection_ref` (engine-specific connection ID/reference)

Suggested ownership:

- Nango-backed: `linkedin`, `facebook`, `twitter-v2` (X), plus other global SaaS.
- OminiConnect native: `feishu`, `dingtalk`, `wechatwork` (and any China-specific providers not in Nango).

## API Contract (OminiConnect-facing)

All tenant clients and agents continue using OminiConnect APIs.

### Connector lifecycle

- `POST /api/connectors` create/update connector config (with `engine`)
- `GET /api/connectors`
- `GET /api/connectors/:platform/status`
- `POST /api/connectors/:platform/test`
- `DELETE /api/connectors/:platform`

### OAuth

- `GET /oauth/:platform` starts OAuth in OminiConnect
- `GET /oauth/:platform/callback` ends OAuth in OminiConnect

Routing behavior:

- For `engine=omini_connect_native`: current OminiConnect OAuth vault flow.
- For `engine=nango`: OminiConnect redirects/bridges to Nango auth flow and stores Nango `connection_id` back into OminiConnect.

### Proxy and tool execution

- `POST|GET /api/proxy/:platform/*path` remains stable.
- Internally dispatch by engine:
  - `omini_connect_native` -> OminiConnect token vault + native upstream.
  - `nango` -> Nango proxy/function endpoint + mapped connection ID.

## Internal Adapter Boundary

Introduce an engine adapter interface in Rust:

- `ConnectorExecutor` trait:
  - `connect_start(...)`
  - `connect_callback(...)`
  - `test_connection(...)`
  - `proxy_request(...)`
  - `disconnect(...)`

Implementations:

- `OminiConnectNativeExecutor`
- `NangoExecutor`

Benefits:

- No API break for portal/frontend.
- Easy per-provider migration and rollback.
- Clear place for policy enforcement and telemetry.

## Data Model Changes

Existing table `connectors` needs extension:

- `engine` (TEXT, default `omini_connect_native`)
- `provider_key` (TEXT, default `platform`)
- `connection_ref` (TEXT, nullable)
- `last_health_at` (TEXT, nullable)
- `last_health_status` (TEXT, nullable)
- `last_error` (TEXT, nullable)

Do not remove legacy columns now. Migrate incrementally.

## Migration Strategy

### Phase 1 (no behavior break)

1. Add schema fields and adapter abstraction.
2. Keep all providers on `omini_connect_native` by default.
3. Add `NangoExecutor` behind feature flag.

### Phase 2 (overlap migration)

1. Migrate `linkedin`, `facebook`, `x/twitter-v2` to `engine=nango`.
2. Keep legacy OminiConnect implementations as fallback.
3. Add parity test cases for auth, proxy, refresh, and failure behavior.

### Phase 3 (hardened production)

1. Circuit breakers and retry policy per engine.
2. Unified error normalization in OminiConnect responses.
3. Unified observability (request IDs and engine tags).

### Phase 4 (Maton/Composio gap fill)

1. Stable semantic tool schemas in OminiConnect.
2. MCP-first discovery and invocation.
3. OminiConnect SDK (TS/Python) backed by OminiConnect endpoints, not raw Nango.

## Deletion Policy

Do **not** delete existing OminiConnect connectors immediately.

Only remove legacy connector paths when all are true:

1. Nango-backed parity tests pass.
2. 2+ weeks stable in staging/production.
3. No unresolved provider-specific regressions.

## Open Questions

1. Should OminiConnect expose provider-level engine choice in UI, or keep it internal?
2. For enterprise self-host, do we bundle Nango as a managed sidecar or separate deployment?
3. Should China deployments allow disabling Nango-backed connectors entirely?

## Recommended Defaults

- Tenant-facing UI: one OminiConnect portal.
- Engine selection: internal default map by provider.
- Nango portal: internal admin/debug only.
