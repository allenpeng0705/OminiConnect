# MCP gateway + API gateway — next version backlog

**Purpose:** Track **what is not finished** in the combined MCP gateway and built-in API gateway, for **release planning**. Canonical status vs `main` stays in [`gateway_design_completion.md`](./gateway_design_completion.md); this file is the **prioritized remainder** only.

**Scope:** In-repo product gaps (code + operator docs). Security sign-off and environment-specific SLOs may live partly outside the repo.

---

## 1. MCP transport and protocol

| ID | Item | Notes |
|----|------|--------|
| **D-open** | **MCP streamable HTTP (full semantics)** | Today: minimal SSE wrap + in-memory session IDs. **Next:** session lifecycle, resumption, multi-chunk streams — align with client expectations beyond one JSON-RPC envelope per POST. |
| **D-open** | **JSON-RPC batch** on ingress MCP | Not implemented on `backend: mcp` POST path. |

---

## 2. API gateway — ingress

| ID | Item | Notes |
|----|------|--------|
| **F2** | **Ingress load / SLO evidence** | **In repo:** [`runbooks/ingress_gateway_slo.md`](./runbooks/ingress_gateway_slo.md) + [`grafana/ingress_gateway_slo.json`](./grafana/ingress_gateway_slo.json). **Per GA:** attach measured p99 / 429 rates (ticket or release notes). |
| **Catalog** | **Per-route auth** on ingress rows | **`auth`** on ingress rows shipped; remaining catalog gaps (split listen, ACME) vs [`panda_api_gateway_features.md`](./panda_api_gateway_features.md). |
| **G2-open** | *(optional)* **Ingress RL dashboards** | Counters shipped; operators still need to **import** dashboard / wire alerts in **their** Prometheus. |
| **Catalog** | **Ingress TLS** | Split listen (admin vs public), ACME, advanced cipher policy — beyond top-level `tls` on `listen`. |

---

## 3. API gateway — egress

| ID | Item | Notes |
|----|------|--------|
| **G3** | **Cluster / per-route egress caps + cipher policy** | **Shipped** (see [`gateway_backlog_progress.md`](./gateway_backlog_progress.md) **G3**): optional **`egress.rate_limit.redis`**, **`per_route`** caps with **`route_label`**, **`egress.tls.cipher_suites`** + **`min_protocol_version`**, **`panda_egress_rps_total`**. **Residual:** tune alerts per environment; exotic multi-cluster topologies may still need extra design. |

---

## 4. Control plane (dynamic config)

| ID | Item | Notes |
|----|------|--------|
| **E partial** | **Multi-tenant RBAC / row-level namespacing** | Control plane exists; **full** tenant isolation and role model still open per plan. |
| **E** | **MySQL / Azure SQL stores** | Postgres (and file/sqlite/memory) path exists; other SQL backends not implemented. |
| **E partial** | **OIDC + Redis keys for control plane** | **Shipped (baseline):** console OIDC session for CP when enabled; **`oidc_read_only_roles`** (read-only vs RW); Redis API keys with **`scopes`** + optional **`tenant_id`** for scoped mutations. **Open:** full multi-tenant RBAC UI, richer role matrix — [`control_plane_evolution.md`](./control_plane_evolution.md). |
| **E4-open** | **Redis-backed issuance/revocation UI** (if product needs it) | HTTP **`POST/DELETE …/v1/api_keys`** exists when Redis configured; a **productized** operator UI is still optional. |

---

## 5. Hardening and release hygiene

| ID | Item | Notes |
|----|------|--------|
| **F3** | **Security review sign-off** | **In repo:** [`security_review_gate.md`](./security_review_gate.md) (formal gate). **Per GA:** complete checklist + sign-off block in your ticket system. |

---

## 6. Developer portal

| ID | Item | Notes |
|----|------|--------|
| **H2** | **Portal accuracy** | Ongoing: keep `/portal`, OpenAPI slice, tools catalog aligned with shipped behavior. |
| **H3** | **API keys + ingress wiring** | **Optional** product slice: self-service issuance/revocation and **using** those keys on ingress — not required for “baseline portal” complete. |

---

## 7. Cross-cutting

| ID | Item | Notes |
|----|------|--------|
| **Correlation** | **`route_id` (and friends) everywhere** | Partial today; completion matrix calls full plumbing **partial**. |
| **Per-identity limits** | **RPS / concurrency by caller** | Design gap beyond prefix-based ingress RL. |

---

## Suggested ordering (non-binding)

1. **F2 + F3 evidence** — fill SLO evidence table + security sign-off when targeting a **named GA** (see runbooks).  
2. **Dashboards / alerts** — import [`grafana/ingress_gateway_slo.json`](./grafana/ingress_gateway_slo.json) and egress metrics; tune thresholds per environment (**G2/G3** counters already exist).  
3. **Streamable MCP (full)** when a concrete client or spec milestone requires it.  
4. **H3** only if product commits to API-key-based access on ingress.  
5. **E** depth (RBAC, extra stores) when control-plane becomes customer-facing.

---

## Related

- [`gateway_design_completion.md`](./gateway_design_completion.md) — phase table vs code  
- [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md) — phase IDs (D–H)  
- [`panda_api_gateway_features.md`](./panda_api_gateway_features.md) — feature catalog  
