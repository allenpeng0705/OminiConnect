# MCP gateway — Phase 1 (minimal YAML)

**Purpose:** Onboarding skeleton for **MCP + API gateway** without advanced options. Same content was previously referenced from [`architecture_two_pillars.md`](./architecture_two_pillars.md) and [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md).

**Canonical deep dive:** [`mcp_gateway_phase1` content lives in `panda.example.yaml` comments](https://github.com/search?q=repo%3A+panda.example.yaml+mcp+) — search the repo for the **`# --- MCP gateway (Phase 1)`** block in [`panda.example.yaml`](../panda.example.yaml).

## Minimal steps

1. Set **`mcp.enabled: true`** and at least one **`mcp.servers[]`** entry (stdio `command` or **`http_tool`** with **`api_gateway.egress`** enabled per example).
2. Set **`mcp.advertise_tools: true`** if tools should appear on **`POST /v1/chat/completions`**.
3. For **HTTP MCP ingress** (`POST /mcp` JSON-RPC): set **`api_gateway.ingress.enabled: true`** (built-in **`/mcp` → backend: mcp** when `ingress.routes` is empty).

See also: [`mcp_gateway_reference_designs.md`](./mcp_gateway_reference_designs.md), [`testing_mcp_api_gateway.md`](./testing_mcp_api_gateway.md).
