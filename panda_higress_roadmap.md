# Panda Gateway: Unique AI + MCP + API Gateway Roadmap

## North star

**Panda is not “another Higress.”** It is a **distinct product**: a **Rust unified gateway** where **AI traffic, MCP tool orchestration, and API ingress/egress** share **one runtime, one configuration model, and one governance story**—optimized for **simplicity, AI-era operations, and agent workloads**, not for replicating **Istio + Envoy + Kubernetes ingress controller** at hyperscale.

[Alibaba Higress](https://github.com/alibaba/higress) is a valuable **reference** (Wasm plugins, MCP hosting patterns, console UX expectations, enterprise packaging). We **learn selectively**; we **do not** chase parity with its control plane, plugin mass, or ingress-controller positioning.

---

## Executive summary

| | **Higress** | **Panda** |
|---|-------------|-----------|
| **Center of gravity** | Cloud-native **ingress / microservice gateway** extended with AI and MCP (Envoy + Istio ecosystem). | **AI + MCP + API** as a **single first-class product** in one binary and one YAML surface. |
| **Best at** | K8s Ingress/Gateway API, huge Wasm catalog, Nacos/Dubbo-style discovery, Alibaba-scale ingress story. | **Unified** LLM + tools + routes, **low-cardinality** observability, **token/cost governance**, stream-first AI paths, **minimal ops footprint**. |
| **Relationship** | Benchmark and lesson source. | **Different category emphasis**—overlap in “AI gateway” wording only; buyers and architecture differ. |

**Strategic takeaway:** Compete on **clarity of purpose** and **depth on AI/MCP/governance**, not on **being a smaller Higress**. Roadmap items below are **Panda-first**; Higress appears only where it informs UX or packaging patterns.

---

## How Panda can be much better than Higress (differentiation)

These are **intentional advantages**—what we optimize for. They are not claims that Panda wins on every dimension (Higress wins on K8s ingress replacement, plugin ecosystem size, and long Alibaba production lineage).

### 1. One product, one mental model

- **Panda:** **API gateway + MCP host + AI gateway** in **one process**, **one `panda.yaml`**, one set of policies (auth, limits, audit) across chat, tools, and REST—no mesh, no CRD sprawl for the core story.
- **Higress:** Powerful but **heavy** stack (Envoy + Istio-style control plane); AI/MCP are **extensions** on a **platform** whose primary identity is still **cloud-native ingress and microservices**.

**Better for:** Teams that want a **dedicated AI+MCP gateway** without operating a **service-mesh-class** control plane.

### 2. AI-era governance as the hero feature

- **Panda:** **Token budgets**, **semantic cache**, **low-cardinality metrics**, compliance-oriented logging, **model failover** semantics documented (e.g. `/ready`)—**cost, fairness, and audit** as first-class.
- **Higress:** Strong policy and plugins; enterprise story often tied to **ingress scale** and **Wasm** breadth.

**Better for:** Buyers who prioritize **LLM spend control**, **observability that does not explode cardinality**, and **auditability of tool and model usage** over raw ingress feature count.

### 3. MCP-native depth (not “MCP as a plugin category”)

- **Panda:** MCP **in the core**—stdio/HTTP/remote tools, multi-round loops, tool-result cache, registry examples (`panda-mcp-registry/`), streamable MCP paths in tests—**same gateway** as AI routes.
- **Higress:** MCP **hosted through plugins** and tooling (e.g. openapi-to-mcp); excellent, but **orchestration model** is still **platform + extension**.

**Better for:** **Agent platforms** and product teams that treat **tools and models as one system** to govern and observe.

### 4. Radical operational simplicity

- **Panda:** **Single Rust binary**, Docker/K8s as **deployment options**, not a prerequisite for a coherent story.
- **Higress:** **Production-grade** and **K8s-centric**; all-in-one Docker exists, but the **gravitational pull** is **cluster ingress and enterprise K8s**.

**Better for:** **Edge**, **VM**, **single-team** deployments, and **fast dev/prod parity** without a platform team.

### 5. Smaller attack surface and clear ownership

- **Panda:** **Focused codebase** (Rust), fewer moving parts than **Envoy + control plane + CRD ecosystem**—clearer **CVE ownership** and upgrade path *for this product scope*.
- **Higress:** Inherits **Envoy/Istio** velocity and ecosystem; different tradeoff.

**Better for:** Organizations that value **supply-chain clarity** on a **bounded** gateway product.

### 6. Streaming and long-lived AI connections as native constraints

- **Panda:** **Stream-first** design, SSE timeouts, documented failover—**AI workloads** shape the core, not only Wasm filter capabilities.
- **Higress:** Strong streaming/SSE story on Envoy; Panda differentiates by **end-to-end semantics** documented in **one** stack (config + `/ready` + tests).

**Better for:** Teams that need **predictable** streaming behavior **without** debugging across mesh + gateway layers for **their** feature set.

---

## Positioning snapshot (honest)

| Dimension | Higress (reference) | Panda (unique AI + MCP + API gateway) |
|-----------|----------------------|----------------------------------------|
| Core runtime | Envoy + Istio-style control plane | Rust `panda-server`, unified proxy + MCP runtime |
| Config | K8s CRDs / Ingress / Gateway API + console | Unified YAML; control plane REST where needed |
| Primary win | **K8s ingress + microservices + Wasm mass** | **Unified AI + MCP + API** + **governance** + **simplicity** |
| MCP | Plugins + openapi-to-mcp ecosystem | **Core MCP host** + orchestration + registry patterns |
| “Better” claim | Scale, plugin count, ingress migration | **One config**, **cost/audit**, **MCP depth**, **ops simplicity** |

---

## What to learn from Higress (selective, non-cloning)

Use Higress as a **UX and packaging lab**, not a **feature checklist**.

1. **Operator expectations** — Console flows, auth method breadth, ACME, Helm/mirrors: **learn patterns**, implement **Panda-shaped** equivalents.
2. **Plugin discipline** — Versioned Wasm, hub/catalog **ideas**; Panda uses **PDK + curated quality** over **plugin count**.
3. **OpenAPI → MCP** — Study [openapi-to-mcpserver](https://github.com/higress-group/openapi-to-mcpserver); **integrate or wrap**, do not rewrite without cause.
4. **Docs and demos** — Quickstarts and **MCP hosting** narratives; **reframe** around Panda’s **single-binary** story.
5. **Related repos (reference only):** [higress-console](https://github.com/higress-group/higress-console), [higress-standalone](https://github.com/higress-group/higress-standalone), [wasm-go](https://github.com/higress-group/wasm-go).

**Explicit non-goals (avoid becoming Higress-like):**

- Do **not** treat **Ingress API / Gateway API controller parity** as default success—only if the product **explicitly** chooses the **K8s ingress replacement** wedge.
- Do **not** optimize roadmap for **Wasm plugin count** vs Higress; optimize for **Panda plugin contract**, **safety**, and **AI/MCP relevance**.
- Do **not** rebuild **Istio + Envoy**; **Rust unified core** is the differentiator.

---

## Gaps to close (Panda’s bar—not Higress parity)

**Already strong:** Rust performance narrative, streaming/SSE, MCP + AI in one gateway, low-cardinality metrics, Wasm/PDK, JWT and compliance logging, control plane test coverage (`docs/testing_scenarios_and_data_flows.md`).

| Area | Why it matters for **Panda’s** story | Notes |
|------|--------------------------------------|--------|
| **Product narrative** | Sell **unique** value, not “catch Higress” | Website, README, one **comparison** page: Panda vs generic ingress-first AI gateways |
| **Console + RBAC** | Enterprise adoption without YAML-only ops | Tied to **unified** AI/MCP/API views—not generic ingress UI |
| **Curated plugin line** | Trust and **AI-relevant** extensions | Small set, great docs—**quality** as differentiator |
| **OpenAPI → MCP** | Faster time-to-tool for API backends | Aligns with **MCP-native** positioning |
| **Security depth** | Table stakes for edge | HMAC, Basic, OIDC depth, ACME as **incremental** |
| **Optional SD** | Only if buyers need **registry-backed** upstreams | Not default; **does not define** Panda |

---

## Roadmap phases (Panda-first outcomes)

Phases prioritize **differentiation** and **enterprise readiness** on **our** architecture. Timelines are indicative.

### Phase 1 — Sharpen the unique story + ship operational clarity (0–3 months)

| Priority | Initiative | Outcome |
|----------|------------|---------|
| P0 | **Positioning & docs** — Single doc: what Panda is/is not; **“not another Higress”** explicit; comparison table | Buyers self-select; contributors understand scope |
| P0 | **Enterprise ops doc** — Control plane, reload, HA, SSE/failover (`/ready`), fleet snapshot | Admins succeed without reading source |
| P0 | **Security breadth** — Auth matrix (HMAC, Basic, OIDC where needed) | Enterprise checklist without cloning Higress’s plugin list |
| P1 | **PDK + curated plugins** — Semver, CI, minimal signing; **AI/MCP-relevant** examples | Quality and safety over count |
| P1 | **OpenAPI → MCP** — Tooling or integration path | Faster MCP adoption for REST backends |
| P2 | **Distribution** — Multi-region images, Helm values aligned with `deploy/` | Predictable installs |

### Phase 2 — Unified management plane (3–6 months)

| Priority | Initiative | Outcome |
|----------|------------|---------|
| P0 | **Console** — One UI for **routes, models, MCP servers, budgets, keys** (not generic ingress-only) | **Differentiated** operator experience |
| P1 | **Durable config + audit** — Source of truth for console; change audit, dry-run | Enterprise change management |
| P1 | **Fast config apply** — Measured targets where applicable | Documented SLO for route/policy updates |
| P2 | **MCP lifecycle** — Health, metrics, clear model: local vs remote vs Wasm-adjacent | **MCP-native** operations story |
| P2 | **Service discovery** — **Optional** (e.g. one registry) | Only if product commits; not default |

### Phase 3 — Hardening at the edge (6–12 months)

| Priority | Initiative | Outcome |
|----------|------------|---------|
| P1 | **WAF / L7 rules** — Pragmatic coverage; Wasm where it fits | Security gateway narrative without Envoy dependency |
| P1 | **ACME / cert lifecycle** | Hands-off TLS |
| P2 | **Multi-tenant isolation** — Budgets, audit, namespaces per tenant | SaaS and large orgs |
| P2 | **Community** — CONTRIBUTING, plugin templates, public roadmap | Ecosystem fits **Panda** model |

---

## Design principles (carry through all work)

1. **Unified policy** — Same auth, limits, and audit hooks for **API, MCP tools, and LLM routes** wherever possible.
2. **Low-cardinality observability by default** — Do not trade **AI observability** for metric explosion.
3. **MCP and AI in the core** — Extensions **amplify** the core story; they do not **replace** it.
4. **Simplicity is a feature** — Resist **mesh-class** complexity unless the product **explicitly** chooses that wedge.
5. **Learn from Higress; ship Panda** — Borrow **patterns**; never **merge roadmaps**.

---

## Success metrics (Panda-specific)

| Category | Example target | Notes |
|----------|----------------|-------|
| **Differentiation** | Published **positioning** + **comparison** (honest) | Sales and OSS clarity |
| **Unified ops** | Time to **first** governed AI+MCP+API path (minutes, not days) | Demo and onboarding |
| **Governance** | Documented **budget + audit** stories for LLM + tools | Enterprise AI buyers |
| **Performance** | P95 overhead vs baseline (`benchmarks/`) | Scoped to **Panda** feature set |
| **Reliability** | Uptime SLO in **reference** deployment | Claim only what you measure |
| **Ecosystem** | **Curated** plugins with CI | Quality, not Higress-scale count |

---

## Risks

- **Feature parity trap** — Chasing Higress **plugin count** or **ingress** features **dilutes** the unique story.
- **Scope creep** — Full **Gateway API ingress controller** is a **different product**; enter only deliberately.
- **Messaging drift** — If marketing says “like Higress,” **reset** to **Panda’s** north star (this document).

---

## Suggested next steps

1. **Ship positioning** — README + site: **one paragraph** + comparison: *unique AI + MCP + API gateway* vs ingress-first gateways.
2. **Owner matrix** — Phase 1 table with owners and quarter targets.
3. **Spikes in parallel** — Console MVP (**unified** AI/MCP/API); OpenAPI→MCP evaluation; one **security** item (OIDC depth or ACME).

---

## References

- Higress: https://github.com/alibaba/higress  
- Panda: `README.md`  
- Higress (local): `../higress/README.md`  
- Data flows / tests: `docs/testing_scenarios_and_data_flows.md`

---

## Document history

- **Updated:** Reframed around **unique AI + MCP + API gateway**; added **“much better than Higress”** as **differentiation**, not universal superiority; selective learning from Higress; **non-goals** to avoid cloning; roadmap tied to **Panda-first** outcomes.
