# Kong (edge) handshake with Panda

This document is the **contract** between an upstream L7 gateway (Kong, NGINX, Envoy, cloud LB, etc.) and Panda when you run **Panda behind the edge** (coexistence / “best neighbors”).

## Trust boundary

- **Untrusted:** Internet clients and anything before your edge.
- **Trusted hop:** The single network hop from **edge → Panda** where you control routing and header injection (private VPC, mesh, or locked listener).

Panda **must not** trust identity headers from clients. It only trusts them when **attestation succeeds** on the edge→Panda hop.

## Environment

| Variable | Required | Purpose |
|----------|----------|---------|
| `PANDA_TRUSTED_GATEWAY_SECRET` | When using attestation | Shared secret; must match what the edge sends in the attestation header (see below). |

## YAML: `trusted_gateway` (optional)

In `panda.yaml`:

| Field | Required | Purpose |
|-------|----------|---------|
| `attestation_header` | Recommended | Header name the edge sets so Panda knows the hop is trusted. |
| `subject_header` | Optional | End-user subject (e.g. OIDC `sub` after edge auth). |
| `tenant_header` | Optional | Tenant identifier. |
| `scopes_header` | Optional | Space- or comma-separated scopes. |

If `attestation_header` is unset or `PANDA_TRUSTED_GATEWAY_SECRET` is unset, **no hop is trusted** and identity headers are stripped before upstream (see behavior below).

## Attestation semantics (exact)

Implementation: `crates/panda-proxy/src/shared/gateway.rs` (`attestation_equals`, `apply_trusted_gateway`).

1. The edge sets **`attestation_header`** to a **string value** (single header value, not multi-line).
2. Panda compares that value to the string in **`PANDA_TRUSTED_GATEWAY_SECRET`** without leaking timing via naive string compare: it computes **HMAC-SHA256(key, header_value)** and **HMAC-SHA256(key, secret)** with `key = secret.as_bytes()`, then compares those digests with `constant_time_eq`.
3. For trust to succeed, the header value must be **byte-for-byte identical** to `PANDA_TRUSTED_GATEWAY_SECRET`. This is a **shared-secret** check on a private hop (not a JWT signature).
4. If either side is longer than **16 KiB**, attestation fails.
5. After processing, Panda **removes** the attestation header from the upstream-bound request.

**Operational note:** Treat `PANDA_TRUSTED_GATEWAY_SECRET` like a password: rotate via config rollout, restrict who can read Panda’s env, and prefer **mTLS or private networking** on edge→Panda so the secret is not exposed on the public internet.

## Identity headers (subject / tenant / scopes)

When `trusted_hop == true`:

- Panda reads `subject_header`, `tenant_header`, `scopes_header` (if configured) and exposes them into internal `RequestContext` (TPM keys, audit, etc.).
- Those headers are **left on the request** for upstream (unless other middleware changes them).

When `trusted_hop == false`:

- Panda **removes** `subject_header`, `tenant_header`, and `scopes_header` so clients cannot spoof identity.

## Correlation and tracing

Implementation: `ensure_correlation_id` in `gateway.rs`.

- **Primary:** `observability.correlation_header` (default `x-request-id`) if present and non-empty.
- **Else:** W3C `traceparent` → extract **32-hex trace id** and set `correlation_header` to that value.
- **Else:** Generate a UUID.

Downstream responses echo the correlation id on the same header name.

**Recommendation for Kong:** Forward or generate `traceparent` / `x-request-id` so logs align across Kong and Panda.

## Kong recipe (conceptual)

Goal: **route only AI traffic to Panda**, **strip client spoofed identity headers**, **inject attestation + identity** on the Kong→Panda hop.

### 1) Route match

- Match paths that should go to Panda (examples: `/v1/chat/completions`, `/v1/embeddings`, or your internal prefix).
- Point the upstream to Panda’s listen address (internal service).

### 2) Strip untrusted headers from the client

On the **client → Kong** request, remove headers that Panda should not trust unless attested, e.g.:

- `X-User-Id`, `X-Tenant-Id`, `X-User-Scopes` (use the exact names you configure in Panda `trusted_gateway`).

Use Kong’s **Request Transformer** (or equivalent) **remove** rules on those header names.

### 3) Inject trusted headers on Kong → Panda

After authentication (OIDC/JWT plugin at Kong) or from Kong’s consumer context, **set**:

- `attestation_header` = `PANDA_TRUSTED_GATEWAY_SECRET` (same value Panda has in env).
- `subject_header` = authenticated user id (OIDC `sub` or your corporate id).
- **Optional:** `tenant_header`, `scopes_header` from your IdP or Kong consumer.

**Important:** Kong must not forward client-supplied values for those identity headers without overwriting them.

### 4) Panda config

Align names in `panda.yaml` with what Kong sets:

```yaml
trusted_gateway:
  attestation_header: "X-Panda-Internal"
  subject_header: "X-User-Id"
  tenant_header: "X-Tenant-Id"
  scopes_header: "X-User-Scopes"
```

Exact plugin names and YAML snippets depend on Kong version and edition; keep the **header contract** above as the source of truth.

## See also

- [Integration & evolution](./integration_and_evolution.md) — positioning and migration phases.
- [Deployment](./deployment.md#standalone-no-kong) — no Kong / no `trusted_gateway`.
