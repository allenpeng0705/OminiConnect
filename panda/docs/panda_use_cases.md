# Panda Use Cases

This document outlines all possible usage scenarios for Panda, including detailed configuration examples for each scenario.

## Overview

Panda is a unified gateway that combines three main functions:
1. **API Gateway** (both ingress and egress)
2. **MCP (Model Context Protocol) Gateway** for tool orchestration
3. **AI Gateway** for OpenAI-compatible LLM traffic

You can use Panda in various combinations of these components based on your needs.

---

## Scenario 1: AI Gateway Only

### Description
Use Panda as a governed OpenAI-compatible proxy for LLM traffic with features like token budgeting, semantic cache, routing, and failover.

### Use Case
- When you only need to proxy chat completions, embeddings, or other LLM requests
- When you don't need tool orchestration via MCP

### Configuration Example (`panda.yaml`)

```yaml
# AI Gateway Only Configuration

# Basic server settings
listen: 0.0.0.0:8080

# Default backend for LLM traffic
default_backend: https://api.openai.com/v1

# Routes for different LLM services
routes:
  # Internal LLM route
  - path_prefix: /v1/internal-chat
    backend_base: http://internal-llm:8000/v1
    # Rate limits per route
    rps_limit: 10
    rpm_limit: 100
  # OpenAI route
  - path_prefix: /v1/chat
    backend_base: https://api.openai.com/v1
    # Token budget per minute
    tpm_limit: 10000

# MCP disabled
mcp:
  enabled: false

# API Gateway ingress (optional, but recommended for routing control)
api_gateway:
  ingress:
    enabled: true
    routes:
      # Route all AI traffic to the AI backend
      - path_prefix: /v1
        backend: ai
        methods: [POST, GET]
      # Health check route
      - path_prefix: /health
        backend: ops
        methods: [GET]

# Token-based budgeting (TPM)
tpm:
  enforce_budget: true
  budget_tokens_per_minute: 10000

# Semantic cache (optional)
semantic_cache:
  enabled: true
  backend: memory
  similarity_threshold: 0.8

# Observability
observability:
  prometheus:
    enabled: true
  opentelemetry:
    enabled: false
```

### Key Notes
- `mcp.enabled: false` disables MCP functionality
- `default_backend` or `routes` define the LLM upstream
- `tpm` settings control token usage limits
- `semantic_cache` reduces latency and costs by caching similar requests

---

## Scenario 2: MCP Gateway Only

### Description
Use Panda as an MCP gateway for tool orchestration, without LLM traffic.

### Use Case
- Integration tests, scripts, or headless automation
- Direct JSON-RPC `POST /mcp` calls for tool execution
- Tools hitting REST backends or remote MCP servers

### Configuration Example (`panda.yaml`)

```yaml
# MCP Gateway Only Configuration

# Basic server settings
listen: 0.0.0.0:8080

# API Gateway ingress (required for MCP JSON-RPC)
api_gateway:
  ingress:
    enabled: true
    routes:
      # Route /mcp to MCP backend
      - path_prefix: /mcp
        backend: mcp
        methods: [POST]
      # Health check route
      - path_prefix: /health
        backend: ops
        methods: [GET]
  # Egress for REST tools (optional)
  egress:
    corporate:
      default_base: http://internal-api:8080
      allowlist:
        - internal-api:8080
        - api.example.com:443

# MCP configuration
mcp:
  enabled: true
  # Tool servers
  servers:
    # HTTP tool server (REST)
    - name: corpapi
      type: http_tool
      config:
        base_url: /api
        headers:
          Authorization: "Bearer ${CORP_API_KEY}"
    # Stdio MCP server
    - name: localtools
      type: stdio
      config:
        command: ["/path/to/tool-server", "--port", "8081"]
    # Remote MCP server
    - name: remotetools
      type: remote_mcp
      config:
        remote_mcp_url: http://remote-mcp:8082/mcp

# Observability
observability:
  prometheus:
    enabled: true
```

### Key Notes
- `api_gateway.ingress.enabled: true` is required to route `/mcp` to the MCP backend
- `mcp.servers` defines tool servers (stdio, remote MCP, or HTTP tools)
- `api_gateway.egress` is optional, used for REST tool calls to corporate backends

---

## Scenario 3: AI Gateway + MCP Gateway

### Description
Use Panda as an AI gateway with MCP tool orchestration, where the LLM chooses tools and Panda executes them.

### Use Case
- Assistant flows with tool integration
- Chatbots that can access external services
- AI agents that require tool use

### Configuration Example (`panda.yaml`)

```yaml
# AI Gateway + MCP Gateway Configuration

# Basic server settings
listen: 0.0.0.0:8080

# Default backend for LLM traffic
default_backend: https://api.openai.com/v1

# Routes with MCP tool advertisement
routes:
  - path_prefix: /v1/chat
    backend_base: https://api.openai.com/v1
    # Enable MCP tool injection for chat
    mcp_advertise_tools: true
    tpm_limit: 10000
  - path_prefix: /v1/embeddings
    backend_base: https://api.openai.com/v1
    # Disable MCP tool injection for embeddings
    mcp_advertise_tools: false

# API Gateway ingress
api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: /v1
        backend: ai
        methods: [POST, GET]
      - path_prefix: /health
        backend: ops
        methods: [GET]
  # Egress for REST tools
  egress:
    corporate:
      default_base: http://internal-api:8080
      allowlist:
        - internal-api:8080

# MCP configuration
mcp:
  enabled: true
  # Global tool advertisement (overridden by routes)
  advertise_tools: false
  # Tool servers
  servers:
    - name: weather
      type: http_tool
      config:
        base_url: /weather
    - name: database
      type: stdio
      config:
        command: ["/path/to/db-tool-server"]
  # Intent gating (optional)
  intent_tool_policies:
    data_read:
      - mcp_weather_get
      - mcp_database_query
    data_write:
      - mcp_database_update
  proof_of_intent_mode: enforce

# Token-based budgeting
tpm:
  enforce_budget: true
  budget_tokens_per_minute: 10000

# Semantic cache
semantic_cache:
  enabled: true
  backend: memory

# Observability
observability:
  prometheus:
    enabled: true
  opentelemetry:
    enabled: true
    service_name: panda-gateway
```

### Key Notes
- `mcp.enabled: true` enables MCP functionality
- `mcp_advertise_tools: true` on routes enables tool injection for chat requests
- `intent_tool_policies` and `proof_of_intent_mode` control tool access based on user intent
- `api_gateway.egress` is used for REST tool calls

---

## Scenario 4: Full Stack (All Components)

### Description
Use Panda with all components: API Gateway (ingress and egress), MCP Gateway, and AI Gateway.

### Use Case
- Enterprise deployments
- One Panda process serving both chat and MCP tool calls
- Governed access to both LLMs and corporate services

### Configuration Example (`panda.yaml`)

```yaml
# Full Stack Configuration

# Basic server settings
listen: 0.0.0.0:8080

# Default backend for LLM traffic
default_backend: https://api.openai.com/v1

# Routes with MCP tool advertisement
routes:
  - path_prefix: /v1/chat
    backend_base: https://api.openai.com/v1
    mcp_advertise_tools: true
    tpm_limit: 10000
  - path_prefix: /v1/embeddings
    backend_base: https://api.openai.com/v1
    mcp_advertise_tools: false
  - path_prefix: /v1/internal
    backend_base: http://internal-llm:8000/v1
    mcp_advertise_tools: false

# API Gateway ingress
api_gateway:
  ingress:
    enabled: true
    routes:
      # Route /mcp to MCP backend
      - path_prefix: /mcp
        backend: mcp
        methods: [POST]
      # Route /v1 to AI backend
      - path_prefix: /v1
        backend: ai
        methods: [POST, GET]
      # Health check route
      - path_prefix: /health
        backend: ops
        methods: [GET]
      # Metrics route
      - path_prefix: /metrics
        backend: ops
        methods: [GET]
  # Egress for REST tools
  egress:
    corporate:
      default_base: http://internal-api:8080
      allowlist:
        - internal-api:8080
        - api.example.com:443
      # mTLS for corporate services
      mtls:
        enabled: false

# MCP configuration
mcp:
  enabled: true
  advertise_tools: false
  servers:
    - name: weather
      type: http_tool
      config:
        base_url: /weather
    - name: database
      type: stdio
      config:
        command: ["/path/to/db-tool-server"]
    - name: crm
      type: remote_mcp
      config:
        remote_mcp_url: http://crm-mcp:8082/mcp
  intent_tool_policies:
    data_read:
      - mcp_weather_get
      - mcp_database_query
      - mcp_crm_search
    data_write:
      - mcp_database_update
      - mcp_crm_update
  proof_of_intent_mode: enforce

# Token-based budgeting
tpm:
  enforce_budget: true
  budget_tokens_per_minute: 10000
  # Redis for shared counters (optional)
  redis_url: redis://redis:6379

# Semantic cache
semantic_cache:
  enabled: true
  backend: redis
  redis_url: redis://redis:6379
  similarity_threshold: 0.8

# Security
identity:
  require_jwt: true
  jwt_jwks_url: https://auth.example.com/.well-known/jwks.json
  jwt_audience: panda-api

# Observability
observability:
  prometheus:
    enabled: true
  opentelemetry:
    enabled: true
    service_name: panda-gateway
    trace_sampling_ratio: 0.2

# Developer console (optional)
developer_console:
  enabled: false
```

### Key Notes
- All components are enabled and configured
- `api_gateway.ingress` routes both `/mcp` and `/v1` paths
- `api_gateway.egress` governs access to corporate services
- `mcp` configures multiple tool servers
- `tpm` and `semantic_cache` use Redis for shared state
- `identity` enables JWT authentication

---

## Scenario 5: Pure API Gateway

### Description
Use Panda purely as an API gateway for traditional REST traffic, without MCP or AI gateway features.

### Use Case
- Traditional REST API proxying
- Service mesh integration
- Edge gateway for TLS termination and auth

### Configuration Example (`panda.yaml`)

```yaml
# Pure API Gateway Configuration

# Basic server settings
listen: 0.0.0.0:8080

# Default backend for catch-all proxying
default_backend: http://backend-services:8080

# Routes for path-based routing
routes:
  # User service
  - path_prefix: /api/users
    backend_base: http://user-service:8081
    methods: [GET, POST, PUT, DELETE]
    rps_limit: 100
  # Product service
  - path_prefix: /api/products
    backend_base: http://product-service:8082
    methods: [GET, POST]
    rps_limit: 200
  # Order service
  - path_prefix: /api/orders
    backend_base: http://order-service:8083
    methods: [GET, POST]
    rps_limit: 150

# API Gateway ingress (enabled for routing control)
api_gateway:
  ingress:
    enabled: true
    routes:
      # Route API paths to AI backend (which acts as generic HTTP proxy)
      - path_prefix: /api
        backend: ai
        methods: [GET, POST, PUT, DELETE]
      # Health check route
      - path_prefix: /health
        backend: ops
        methods: [GET]
      # Metrics route
      - path_prefix: /metrics
        backend: ops
        methods: [GET]

# MCP disabled
mcp:
  enabled: false

# Security
identity:
  require_jwt: true
  jwt_jwks_url: https://auth.example.com/.well-known/jwks.json
  jwt_audience: api-gateway

# Observability
observability:
  prometheus:
    enabled: true
  opentelemetry:
    enabled: true
    service_name: api-gateway

# TLS configuration (optional)
tls:
  enabled: false
  # cert_pem: /path/to/cert.pem
  # key_pem: /path/to/key.pem
```

### Key Notes
- `mcp.enabled: false` disables MCP functionality
- `routes` define path-based routing to different backend services
- `api_gateway.ingress` enables routing control
- `identity` enables JWT authentication for API access
- `default_backend` acts as a catch-all for unmatched paths (when ingress is disabled)

---

## Conclusion

Panda's flexible architecture allows it to be used in a variety of scenarios, from simple LLM proxying to full-featured enterprise deployments with tool orchestration. By configuring the appropriate components, you can tailor Panda to your specific needs while maintaining a unified deployment model.