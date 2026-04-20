# Testing All Panda Use Cases

This guide explains how to systematically test all Panda use cases using the existing testing framework. Based on the use cases defined in `panda_use_cases.md`, we'll cover configuration validation and end-to-end integration testing.

## Overview

Panda has five main use cases:
1. **AI Gateway Only**
2. **MCP Gateway Only**
3. **AI Gateway + MCP Gateway**
4. **Full Stack (All Components)**
5. **Pure API Gateway**

Each use case should be tested at two levels:
1. **Configuration validation** (in `panda-config/tests/scenario_profiles.rs`)
2. **End-to-end integration** (in `panda-proxy/src/tests/gateway_workflow.rs` or similar)

## Testing Approach

### 1. Configuration Validation Tests

These tests verify that YAML configurations parse correctly and that `effective_*` helpers behave as expected.

**Location:** `crates/panda-config/tests/scenario_profiles.rs`

### 2. Integration Tests

These tests verify the complete data flow through Panda's `dispatch` function.

**Location:** `crates/panda-proxy/src/tests/gateway_workflow.rs` (and other test modules)

## Test Matrix

Create a matrix of all configuration options and their possible states:

| Component | Setting | States |
|-----------|---------|--------|
| **MCP** | `mcp.enabled` | `true`, `false` |
| | `mcp.advertise_tools` | `true`, `false` |
| | `mcp.servers` | None, stdio, http_tool, http_tools, remote_mcp |
| **API Gateway Ingress** | `api_gateway.ingress.enabled` | `true`, `false` |
| | `api_gateway.ingress.routes` | None, minimal, full |
| **API Gateway Egress** | `api_gateway.egress.enabled` | `true`, `false` |
| **AI Gateway** | `default_backend` | Set, not set |
| | `routes` | None, with mcp_advertise_tools |
| **JWT** | `identity.require_jwt` | `true`, `false` |
| **TPM** | `tpm.enforce_budget` | `true`, `false` |
| **Semantic Cache** | `semantic_cache.enabled` | `true`, `false` |

## Extending Existing Tests

### Step 1: Add Configuration Scenario Tests

Update `crates/panda-config/tests/scenario_profiles.rs` to include all use cases:

```rust
#[test]
fn scenario_pure_api_gateway() {
    let cfg = PandaConfig::from_yaml_str(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:8080'
routes:
  - path_prefix: /api/users
    backend_base: 'http://user-service:8081'
  - path_prefix: /api/products
    backend_base: 'http://product-service:8082'
api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: /api
        backend: ai
        methods: [GET, POST, PUT, DELETE]
mcp:
  enabled: false
"#,
    )
    .unwrap();
    assert!(!cfg.mcp.enabled);
    assert!(cfg.api_gateway.ingress.enabled);
    assert_eq!(
        cfg.effective_backend_base("/api/users/123"),
        "http://user-service:8081"
    );
}

#[test]
fn scenario_mcp_gateway_only() {
    let cfg = PandaConfig::from_yaml_str(
        r#"listen: '127.0.0.1:0'
api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: /mcp
        backend: mcp
        methods: [POST]
  egress:
    enabled: true
    corporate:
      default_base: 'http://internal-api:8080'
    allowlist:
      allow_hosts: ['internal-api:8080']
      allow_path_prefixes: ['/api']
mcp:
  enabled: true
  advertise_tools: false
  servers:
    - name: corpapi
      enabled: true
      http_tool:
        path: /api/data
        method: GET
        tool_name: fetch
"#,
    )
    .unwrap();
    assert!(cfg.mcp.enabled);
    assert!(cfg.api_gateway.ingress.enabled);
    assert!(cfg.api_gateway.egress.enabled);
}

#[test]
fn scenario_full_stack_all_components() {
    let cfg = PandaConfig::from_yaml_str(
        r#"listen: '127.0.0.1:0'
default_backend: 'https://api.openai.com/v1'
routes:
  - path_prefix: /v1/chat
    backend_base: 'https://api.openai.com/v1'
    mcp_advertise_tools: true
  - path_prefix: /v1/embeddings
    backend_base: 'https://api.openai.com/v1'
    mcp_advertise_tools: false
api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: /mcp
        backend: mcp
        methods: [POST]
      - path_prefix: /v1
        backend: ai
        methods: [POST, GET]
  egress:
    enabled: true
    corporate:
      default_base: 'http://internal-api:8080'
    allowlist:
      allow_hosts: ['internal-api:8080']
      allow_path_prefixes: ['/api']
mcp:
  enabled: true
  advertise_tools: false
  servers:
    - name: corpapi
      enabled: true
      http_tool:
        path: /api/data
        method: GET
        tool_name: fetch
tpm:
  enforce_budget: true
  budget_tokens_per_minute: 10000
semantic_cache:
  enabled: true
  backend: memory
"#,
    )
    .unwrap();
    assert!(cfg.mcp.enabled);
    assert!(cfg.api_gateway.ingress.enabled);
    assert!(cfg.api_gateway.egress.enabled);
    assert!(cfg.tpm.enforce_budget);
    assert!(cfg.semantic_cache.enabled);
    assert!(cfg.effective_mcp_advertise_tools_for_path("/v1/chat/completions"));
    assert!(!cfg.effective_mcp_advertise_tools_for_path("/v1/embeddings"));
}
```

### Step 2: Add Integration Tests

Create integration tests for each use case in `crates/panda-proxy/src/tests/gateway_workflow.rs` or a new test module.

**For Pure API Gateway:**

```rust
#[tokio::test]
async fn workflow_pure_api_gateway_routes_to_backends() {
    let mock1 = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let mock1_addr = mock1.local_addr().unwrap();
    let mock2 = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let mock2_addr = mock2.local_addr().unwrap();

    let mock1_task = tokio::spawn(async move {
        let (mut sock, _) = mock1.accept().await.unwrap();
        let mut buf = vec![0u8; 4096];
        let n = sock.read(&mut buf).await.unwrap();
        let req = std::str::from_utf8(&buf[..n]).unwrap();
        assert!(req.contains("/api/users"));
        sock.write_all(b"HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 13\r\n\r\n{\"user\":\"ok\"}").await.unwrap();
    });

    let mock2_task = tokio::spawn(async move {
        let (mut sock, _) = mock2.accept().await.unwrap();
        let mut buf = vec![0u8; 4096];
        let n = sock.read(&mut buf).await.unwrap();
        let req = std::str::from_utf8(&buf[..n]).unwrap();
        assert!(req.contains("/api/products"));
        sock.write_all(b"HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 17\r\n\r\n{\"product\":\"ok\"}").await.unwrap();
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routes:
  - path_prefix: /api/users
    backend_base: 'http://{mock1_addr}'
  - path_prefix: /api/products
    backend_base: 'http://{mock2_addr}'
api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: /api
        backend: ai
        methods: [GET]
mcp:
  enabled: false
tpm:
  enforce_budget: false
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).unwrap());
    let ingress = IngressRouter::try_new(&cfg.api_gateway.ingress).unwrap();
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    let state = Arc::new(state);
    let (addr, server) = spawn_dispatch_one_accept(state).await;

    let resp1 = raw_get(addr, "/api/users").await;
    assert!(resp1.contains("200 OK"), "{resp1}");
    assert!(resp1.contains("user"), "{resp1}");

    let (addr2, server2) = spawn_dispatch_one_accept(Arc::clone(&state)).await;
    let resp2 = raw_get(addr2, "/api/products").await;
    assert!(resp2.contains("200 OK"), "{resp2}");
    assert!(resp2.contains("product"), "{resp2}");

    server.await.ok();
    server2.await.ok();
    mock1_task.await.ok();
    mock2_task.await.ok();
}
```

### Step 3: Test Dispatch Branches

Verify all dispatch branches in `crates/panda-proxy/src/tests/dispatch_branches.rs`:

- Control plane enabled/disabled
- Ingress enabled/disabled
- Various backend types

### Step 4: Use Pairwise Testing

For complex combinations, use pairwise testing to cover all interactions without exhaustive testing:

**Example combinations to test:**
- Ingress on/off × MCP on/off
- Egress on/off × HTTP tools on/off
- JWT on/off × TPM on/off
- Semantic cache on/off × TPM on/off

### Step 5: Add Tests to Documentation

Update `docs/testing_scenarios_and_data_flows.md` to document all new tests.

## Running the Tests

```bash
# Run configuration tests
cargo test -p panda-config

# Run integration tests
cargo test -p panda-proxy tests::gateway_workflow -- --nocapture

# Run all proxy tests
cargo test -p panda-proxy

# Run specific use case tests
cargo test -p panda-config scenario_pure_api_gateway -- --nocapture
cargo test -p panda-config scenario_full_stack_all_components -- --nocapture
```

## Checklist for New Use Case Tests

When adding a new use case, ensure you:

1. [ ] Add configuration parsing test to `scenario_profiles.rs`
2. [ ] Test all `effective_*` helper functions
3. [ ] Add integration test to `gateway_workflow.rs`
4. [ ] Test both success and error paths
5. [ ] Test all relevant dispatch branches
6. [ ] Update `testing_scenarios_and_data_flows.md`
7. [ ] Add mocks for external dependencies (LLM, corporate APIs, etc.)
8. [ ] Verify observability (metrics, logging)
9. [ ] Test edge cases (empty config, invalid config, etc.)

## Test Data Flow for Each Use Case

### Pure API Gateway
- **Flow:** `TCP → ING → FWD → backend`
- **What to test:** Path-based routing, method filtering, rate limiting

### AI Gateway Only
- **Flow:** `TCP → ING (optional) → FWD → LLM`
- **What to test:** TPM, semantic cache, model failover, streaming

### MCP Gateway Only
- **Flow:** `TCP → ING → MCP↑ → EG → backend`
- **What to test:** JSON-RPC, tool execution, egress allowlisting

### AI Gateway + MCP Gateway
- **Flow:** `TCP → ING → FWD → LLM → MCP → EG → backend → FWD → LLM`
- **What to test:** Tool injection, multi-round loops, intent gating

### Full Stack
- **Flow:** Multiple flows depending on path
- **What to test:** All of the above + interactions between components

## Mock Strategy

Use these mocks for integration tests:

- **LLM Backend:** Minimal HTTP server that returns OpenAI-compatible responses
- **Corporate API:** Minimal HTTP server for REST tools
- **Remote MCP:** JSON-RPC server over HTTP
- **Stdio MCP:** Python mock (`mcp_mock_stdio.py`)

## Continuous Integration

Ensure all tests run in CI:

```bash
# Fast configuration tests
cargo test -p panda-config

# Integration tests (may require network or Python)
cargo test -p panda-proxy

# End-to-end smoke test (optional)
./scripts/gateway_mcp_e2e_smoke.sh
```

## Summary

By following this approach, you can systematically test all Panda use cases:
1. Start with configuration validation tests
2. Add integration tests for end-to-end flows
3. Use pairwise testing for complex combinations
4. Maintain comprehensive documentation
5. Run tests in CI for every change

This ensures that all use cases remain functional as the codebase evolves.
