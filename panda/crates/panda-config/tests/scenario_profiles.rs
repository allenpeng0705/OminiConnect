//! Scenario YAML profiles aligned with `docs/panda_scenarios_summary.md` and `docs/panda_use_cases.md`.
//! Ensures representative configs parse and `effective_*` helpers behave as documented.

use panda_config::PandaConfig;

#[test]
fn scenario_a_ai_gateway_only_no_mcp() {
    let cfg = PandaConfig::from_yaml_str(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:5023'
routes:
  - path_prefix: /v1/chat
    backend_base: 'https://api.openai.com/v1'
mcp:
  enabled: false
"#,
    )
    .unwrap();
    assert!(!cfg.mcp.enabled);
    assert_eq!(
        cfg.effective_backend_base("/v1/chat/completions"),
        "https://api.openai.com/v1"
    );
}

#[test]
fn scenario_b_mcp_and_egress_no_routes() {
    let cfg = PandaConfig::from_yaml_str(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:5023'
api_gateway:
  ingress:
    enabled: true
  egress:
    enabled: true
    corporate:
      default_base: 'http://127.0.0.1:18081'
    allowlist:
      allow_hosts: ['127.0.0.1:18081']
      allow_path_prefixes: ['/allowed']
mcp:
  enabled: true
  advertise_tools: true
  servers:
    - name: corpapi
      enabled: true
      http_tool:
        path: /allowed/p
        method: GET
        tool_name: t
"#,
    )
    .unwrap();
    assert!(cfg.mcp.enabled);
    assert!(cfg.api_gateway.egress.enabled);
}

#[test]
fn scenario_c_routes_mcp_advertise_global_false_route_true() {
    let cfg = PandaConfig::from_yaml_str(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:5023'
routes:
  - path_prefix: /v1/chat
    backend_base: 'http://127.0.0.1:5023'
    mcp_advertise_tools: true
mcp:
  enabled: true
  advertise_tools: false
  servers:
    - name: a
"#,
    )
    .unwrap();
    assert!(!cfg.effective_mcp_advertise_tools_for_path("/v1/embeddings"));
    assert!(cfg.effective_mcp_advertise_tools_for_path("/v1/chat/completions"));
}

#[test]
fn scenario_e_longest_prefix_embeddings_uses_default_backend() {
    let cfg = PandaConfig::from_yaml_str(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:9'
routes:
  - path_prefix: /v1/chat
    backend_base: 'http://127.0.0.1:7'
mcp:
  enabled: false
"#,
    )
    .unwrap();
    assert_eq!(
        cfg.effective_backend_base("/v1/chat/completions"),
        "http://127.0.0.1:7"
    );
    assert_eq!(
        cfg.effective_backend_base("/v1/embeddings"),
        "http://127.0.0.1:9"
    );
    assert_eq!(cfg.effective_adapter_provider("/v1/embeddings"), "openai");
}

#[test]
fn scenario_f_per_route_adapter_type_overrides_global() {
    let cfg = PandaConfig::from_yaml_str(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
adapter:
  provider: openai
routes:
  - path_prefix: /v1/chat
    backend_base: 'http://127.0.0.1:2'
    type: anthropic
mcp:
  enabled: false
"#,
    )
    .unwrap();
    assert_eq!(cfg.effective_adapter_provider("/v1/chat/foo"), "anthropic");
    assert_eq!(cfg.effective_adapter_provider("/v1/embeddings"), "openai");
}

#[test]
fn scenario_d_repo_root_panda_yaml_parses() {
    let yaml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../panda.yaml"));
    let cfg = PandaConfig::from_yaml_str(yaml).expect("repo panda.yaml must parse");
    assert_eq!(cfg.listen.trim(), "127.0.0.1:8080");
    assert_eq!(cfg.default_backend.trim(), "http://127.0.0.1:5023");
    assert_eq!(cfg.observability.correlation_header, "x-request-id");
    assert_eq!(cfg.observability.admin_auth_header, "x-panda-admin-secret");
    assert!(!cfg.control_plane.enabled);
    assert!(cfg.mcp.enabled);
    assert!(cfg.api_gateway.ingress.enabled);
    assert!(cfg.api_gateway.egress.enabled);
    assert!(!cfg.mcp.advertise_tools);
    assert!(!cfg.effective_mcp_advertise_tools_for_path("/v1/embeddings"));
    assert!(cfg.effective_mcp_advertise_tools_for_path("/v1/chat/completions"));
    assert_eq!(
        cfg.effective_backend_base("/v1/chat/completions"),
        "http://127.0.0.1:5023"
    );
    assert_eq!(
        cfg.effective_backend_base("/v1/embeddings"),
        "http://127.0.0.1:5023"
    );
    let enabled_servers: Vec<_> = cfg
        .mcp
        .servers
        .iter()
        .filter(|s| s.enabled)
        .map(|s| s.name.as_str())
        .collect();
    assert!(
        enabled_servers.contains(&"corpapi")
            && enabled_servers.contains(&"corp")
            && enabled_servers.contains(&"inventory")
            && enabled_servers.contains(&"edge"),
        "expected four REST MCP servers: {enabled_servers:?}"
    );
}

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
      - path_prefix: /health
        backend: ops
        methods: [GET]
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
    assert_eq!(
        cfg.effective_backend_base("/api/products/456"),
        "http://product-service:8082"
    );
}

#[test]
fn scenario_mcp_gateway_only() {
    let cfg = PandaConfig::from_yaml_str(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:5023'
api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: /mcp
        backend: mcp
        methods: [POST]
      - path_prefix: /health
        backend: ops
        methods: [GET]
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
    - name: crm
      enabled: true
      remote_mcp_url: 'http://crm-mcp:8082/mcp'
"#,
    )
    .unwrap();
    assert!(cfg.mcp.enabled);
    assert!(cfg.api_gateway.ingress.enabled);
    assert!(cfg.api_gateway.egress.enabled);
    let enabled_servers: Vec<_> = cfg
        .mcp
        .servers
        .iter()
        .filter(|s| s.enabled)
        .map(|s| s.name.as_str())
        .collect();
    assert!(
        enabled_servers.contains(&"corpapi") && enabled_servers.contains(&"crm")
    );
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
      - path_prefix: /health
        backend: ops
        methods: [GET]
      - path_prefix: /metrics
        backend: ops
        methods: [GET]
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
    - name: crm
      enabled: true
      remote_mcp_url: 'http://crm-mcp:8082/mcp'
  intent_tool_policies:
    - intent: data_read
      allowed_tools:
        - mcp_corpapi_fetch
        - mcp_crm_search
    - intent: data_write
      allowed_tools:
        - mcp_crm_update
  proof_of_intent_mode: enforce
tpm:
  enforce_budget: true
  budget_tokens_per_minute: 10000
semantic_cache:
  enabled: true
  backend: memory
identity:
  require_jwt: true
  jwt_jwks_url: 'https://auth.example.com/.well-known/jwks.json'
  jwt_audience: 'panda-api'
"#,
    )
    .unwrap();
    assert!(cfg.mcp.enabled);
    assert!(cfg.api_gateway.ingress.enabled);
    assert!(cfg.api_gateway.egress.enabled);
    assert!(cfg.tpm.enforce_budget);
    assert!(cfg.semantic_cache.enabled);
    assert!(cfg.identity.require_jwt);
    assert!(cfg.effective_mcp_advertise_tools_for_path("/v1/chat/completions"));
    assert!(!cfg.effective_mcp_advertise_tools_for_path("/v1/embeddings"));
    let enabled_servers: Vec<_> = cfg
        .mcp
        .servers
        .iter()
        .filter(|s| s.enabled)
        .map(|s| s.name.as_str())
        .collect();
    assert!(
        enabled_servers.contains(&"corpapi") && enabled_servers.contains(&"crm")
    );
}
