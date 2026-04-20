# Security review gate (F3)

**Purpose:** Structured **formal security review** before calling the **MCP + API gateway** surface **GA-ready** for an environment. Pen tests, customer security questionnaires, and **signed approvals** may live **outside** the repo; this file is the **in-repo** checklist, scope statement, and sign-off template.

**Related:** [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md) §3.6, [`gateway_design_completion.md`](./gateway_design_completion.md), [`runbooks/ingress_gateway_slo.md`](./runbooks/ingress_gateway_slo.md) (F2 evidence complements F3).

---

## 1. Review charter (fill per engagement)

| Field | Notes |
|-------|--------|
| **Scope** | Panda binary version; **ingress / MCP / egress / control plane / semantic cache / Wasm plugins** — strike what is out of scope. |
| **Threat model inputs** | Link or attach: [`threat_model_semantic_cache.md`](./threat_model_semantic_cache.md); egress SSRF assumptions; identity (JWT, ops secrets, control-plane OIDC / API keys). |
| **Residual risk** | Document **accepted** risks (e.g. process-local egress caps, streamable MCP session limits) and **mitigations** (rate limits, allowlists, monitoring). |
| **Evidence** | `cargo audit` / org scanner output; config review PR; optional external pen-test ID. |

---

## 2. Threat areas (minimum)

| Area | Questions |
|------|-----------|
| **Ingress** | Can unauthenticated callers reach **`/mcp`** or **`/v1`** when they should not? JWT / ops secrets / trusted gateway attestation tested? Per-route **`auth`** (`inherit` / `required` / `optional`) matches policy? |
| **Egress** | SSRF: only **allowlisted** hosts/paths? mTLS and CA trust **as configured**? Retry behavior does not amplify abuse? |
| **Control plane** | Admin routes require **`observability.admin_auth_header`** + secret (or allowed alternatives); **read-only** env secrets and **read-only OIDC roles** cannot mutate; **Redis API keys** with **`scopes`** / **`tenant_id`** cannot escape tenant scope on ingress mutations. |
| **Semantic cache** | Cache poisoning and privacy: [`threat_model_semantic_cache.md`](./threat_model_semantic_cache.md); who can influence keys and stored completions? |
| **MCP & tools** | `tool_routes`, `http_tool`, **remote MCP** URLs reviewed for production data paths; HITL and tool-cache policies match risk tolerance. |
| **Secrets** | No API keys in YAML; env-based references documented; logs do not print `Authorization` or raw tokens. |
| **TLS** | Min version / cert reload behavior matches policy (`api_gateway.egress.tls`, listener TLS). |
| **Dependencies** | `cargo audit` (or org equivalent); track RUSTSEC advisories for **tokio / hyper / rustls** stack. |

---

## 3. Checklist (copy for each release)

**Aligned with [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md) §3.6**, extended for shipped controls.

- [ ] **Egress SSRF** — host + path allowlist enforced when egress enabled (`api_gateway.egress`).
- [ ] **Secrets** — not logged; debug / trace paths reviewed for header redaction.
- [ ] **Ingress rate limits** — `routes[].rate_limit`, `api_gateway.ingress.routes[].rate_limit`, optional **`ingress.rate_limit_redis`**; **Prometheus** counters **`panda_gateway_rps_*`** and **`panda_gateway_ingress_rps_total`** reviewed for monitoring (see F2 runbook).
- [ ] **mTLS egress** — client cert + optional `extra_ca_pem` when required; integration coverage in `api_gateway/egress.rs`.
- [ ] **Ingress surface** — default table and every custom `path_prefix` intentional; MCP and AI paths match exposure intent.
- [ ] **Control plane** — store backend (memory / file / sqlite / postgres) appropriate; dynamic routes and **import** paths reviewed.
- [ ] **Rate limits** — ingress RL, egress RL, TPM set for expected load.
- [ ] **Dependencies** — `cargo audit` or org equivalent run; critical findings triaged.
- [ ] **Logs** — grep logging paths for accidental secret emission.

---

## 4. Formal sign-off block

Use for **internal GA**, **customer go-live**, or **SOC / procurement** packages. External signatures stay in your ticket system; paste references below.

| Field | Value |
|-------|--------|
| **Release / tag** | |
| **Date** | |
| **Review type** | e.g. internal design review / peer review / external pen test summary |
| **Reviewer(s)** | Name, role (e.g. security champion, SRE, customer CISO delegate) |
| **Scope summary** | One paragraph: what was reviewed |
| **Open findings** | IDs and severity, or “none” |
| **External references** | Pen-test report ID, ticket URL, questionnaire ID (optional) |
| **Notes** | |

---

## 5. Status

- **In-repo gate:** Checklist §3 complete **for your GA bar** (internal or customer).
- **F3 tracking:** Mark progress in [`gateway_backlog_progress.md`](./gateway_backlog_progress.md) if your program links F3 to a specific release.

Formal **legal sign-off** is out of scope for this repository; this document provides **traceability** that a structured review occurred.
