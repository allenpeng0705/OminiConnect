#!/bin/bash
# Generate traffic to simulate all empty Grafana dashboard panels

echo "Generating traffic to simulate all metrics..."
echo "================================================="

# 1. MCP Gateway - Agent Governance
echo "\n1. Generating MCP Agent Governance events..."

# Use the test configuration file
echo "Using test configuration file: panda.test.yaml"

# Restart panda with test configuration
echo "Restarting panda with test configuration..."
pkill -f panda || true
sleep 2
export PANDA_ADMIN_STATUS_SECRET="123456"
cargo run --bin panda panda.test.yaml > /dev/null 2>&1 &
sleep 5

# 2. AI Gateway - Semantic Cache Hit Rate
echo "\n2. Generating AI Gateway - Semantic Cache traffic..."

# First generate some unique requests to populate the cache
for i in {1..5}; do
  curl -X POST "http://localhost:8080/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "x-panda-admin-secret: 123456" \
    -d '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "What is the capital of France?"}]}' > /dev/null 2>&1
  echo -n "M"
  sleep 0.1
done

# Now generate identical requests to get cache hits
for i in {1..10}; do
  curl -X POST "http://localhost:8080/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "x-panda-admin-secret: 123456" \
    -d '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "What is the capital of France?"}]}' > /dev/null 2>&1
  echo -n "H"
  sleep 0.1
done
echo ""

# 3. MCP Gateway - Tool Cache Hit Rate and Operations
echo "\n3. Generating MCP Tool Cache traffic..."

# Use a fixed session ID
SID="test-session-$(date +%s)"

# Generate tool calls to populate cache
echo "Populating tool cache..."
for i in {1..5}; do
  curl -X POST "http://localhost:8080/mcp" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json, text/event-stream" \
    -H "Mcp-Session-Id: $SID" \
    -d '{"jsonrpc":"2.0","id":'"$((i+1))"',"method":"tools/call","params":{"name":"mcp_corpapi_fetch","arguments":{"query":"test-query"}}}' > /dev/null 2>&1
  echo -n "S"
  sleep 0.2
done

# Generate identical tool calls to get cache hits
echo "\nGenerating cache hits..."
for i in {1..10}; do
  curl -X POST "http://localhost:8080/mcp" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json, text/event-stream" \
    -H "Mcp-Session-Id: $SID" \
    -d '{"jsonrpc":"2.0","id":'"$((i+10))"',"method":"tools/call","params":{"name":"mcp_corpapi_fetch","arguments":{"query":"test-query"}}}' > /dev/null 2>&1
  echo -n "H"
  sleep 0.1
done

# Generate different tool calls to get cache misses
echo "\nGenerating cache misses..."
for i in {1..5}; do
  curl -X POST "http://localhost:8080/mcp" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json, text/event-stream" \
    -H "Mcp-Session-Id: $SID" \
    -d '{"jsonrpc":"2.0","id":'"$((i+20))"',"method":"tools/call","params":{"name":"mcp_corpapi_fetch","arguments":{"query":"test-query-'"$i"'"}}}' > /dev/null 2>&1
  echo -n "M"
  sleep 0.2
done
echo ""

# 4. MCP Gateway - Agent Governance
echo "\n4. Generating MCP Agent Governance events..."

# Generate MCP traffic with different intent types
for i in {1..20}; do
  # Different intent types
  INTENTS=($"data_read" $"data_write" $"general")
  INTENT=${INTENTS[$RANDOM % ${#INTENTS[@]}]}
  
  # Choose tool based on intent
  case $INTENT in
    "data_read")
      TOOLS=($"mcp_corpapi_fetch" $"mcp_corp_from_a" $"mcp_inventory_health")
      ;;
    "data_write")
      TOOLS=($"mcp_corp_from_b")
      ;;
    "general")
      TOOLS=($"mcp_edge_hi")
      ;;
  esac
  
  TOOL_NAME=${TOOLS[$RANDOM % ${#TOOLS[@]}]}
  
  # Generate MCP JSON-RPC request
  curl -X POST "http://localhost:8080/mcp" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json, text/event-stream" \
    -H "Mcp-Session-Id: governance-session-$i" \
    -d '{"jsonrpc":"2.0","id":'"$i"',"method":"tools/call","params":{"name":"'"$TOOL_NAME"'","arguments":{"query":"test-'"$i"'"}}}' > /dev/null 2>&1
  
  echo -n "."
  sleep 0.2
done
echo ""

# 5. API Gateway - Auth Failures by Reason
echo "\n5. Generating API Gateway - Auth Failures..."

# Generate requests without admin secret
for i in {1..15}; do
  curl -X GET "http://localhost:8080/metrics" > /dev/null 2>&1
  echo -n "F"
  sleep 0.1
done

# Generate requests with invalid admin secret
for i in {1..10}; do
  curl -X GET "http://localhost:8080/metrics" \
    -H "x-panda-admin-secret: invalid" > /dev/null 2>&1
  echo -n "I"
  sleep 0.1
done
echo ""

# 6. API Gateway - Rate Limit Exceeded
echo "\n6. Generating API Gateway - Rate Limit Exceeded events..."

# Send requests faster than the rate limit (10 RPS for /metrics)
echo "Sending requests to exceed rate limit..."
for i in {1..30}; do
  curl -X GET "http://localhost:8080/metrics" \
    -H "x-panda-admin-secret: 123456" > /dev/null 2>&1
  echo -n "R"
  # No sleep - send as fast as possible
done
echo ""

# 7. API Gateway - Ingress Requests by Path
echo "\n7. Generating API Gateway - Ingress Requests..."

# Generate traffic to different ingress paths
PATHS=($"/v1/chat/completions" $"/mcp" $"/health" $"/metrics")

for i in {1..50}; do
  PATH=${PATHS[$RANDOM % ${#PATHS[@]}]}
  
  if [[ "$PATH" == "/v1/chat/completions" ]]; then
    # Chat completion request
    curl -X POST "http://localhost:8080$PATH" \
      -H "Content-Type: application/json" \
      -H "x-panda-admin-secret: 123456" \
      -d '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Hello"}]}' > /dev/null 2>&1
  elif [[ "$PATH" == "/mcp" ]]; then
    # MCP request
    curl -X POST "http://localhost:8080$PATH" \
      -H "Content-Type: application/json" \
      -H "Accept: application/json, text/event-stream" \
      -H "Mcp-Session-Id: ingress-session-$i" \
      -d '{"jsonrpc":"2.0","id":1,"method":"ping","params":{}}' > /dev/null 2>&1
  else
    # GET request
    curl -X GET "http://localhost:8080$PATH" \
      -H "x-panda-admin-secret: 123456" > /dev/null 2>&1
  fi
  
  echo -n "."
  sleep 0.05
done
echo ""

# 8. AI Gateway - Model Failover Retries
echo "\n8. Generating AI Gateway - Model Failover events..."

# Use the failover test configuration file
echo "Using failover test configuration file: panda.test.failover.yaml"

# Restart panda with failover configuration
echo "Restarting panda with failover configuration..."
pkill -f panda || true
sleep 2
export PANDA_ADMIN_STATUS_SECRET="123456"
cargo run --bin panda panda.test.failover.yaml > /dev/null 2>&1 &
sleep 5

# Generate requests to trigger failover
echo "Generating requests to trigger model failover..."
for i in {1..15}; do
  curl -X POST "http://localhost:8080/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "x-panda-admin-secret: 123456" \
    -d '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Hello"}]}' > /dev/null 2>&1
  echo -n "F"
  sleep 0.3
done
echo ""

# 9. TPM Rejects by Bucket Class
echo "\n9. Generating TPM Rejects..."

# Create a TPM test configuration
cat > panda.test.tpm.yaml << 'EOF'
listen: 127.0.0.1:8080
default_backend: "http://127.0.0.1:5023"

routes:
  - path_prefix: /v1/chat
    backend_base: "http://127.0.0.1:5023"
    type: openai
    mcp_advertise_tools: true
    rate_limit:
      rps: 300
    tpm_limit: 100  # Low TPM limit to trigger rejects
    semantic_cache: true

  - path_prefix: /v1
    backend_base: "http://127.0.0.1:5023"
    type: openai
    mcp_advertise_tools: false
    rate_limit:
      rps: 200

  - path_prefix: /mcp
    backend: mcp
    backend_base: "http://127.0.0.1:5023"

control_plane:
  enabled: false
  path_prefix: /ops/control
  store:
    kind: memory

api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: /health
        backend: ops
      - path_prefix: /metrics
        backend: ops
        rate_limit:
          rps: 10
      - path_prefix: /v1
        backend: ai
        rate_limit:
          rps: 50
      - path_prefix: /mcp
        backend: mcp
        rate_limit:
          rps: 50
      - path_prefix: /test/deny
        backend: deny
  egress:
    enabled: true
    timeout_ms: 5000
    pool_idle_timeout_ms: 0
    corporate:
      default_base: "http://127.0.0.1:18081"
    allowlist:
      allow_hosts: ["127.0.0.1:18081"]
      allow_path_prefixes: ["/allowed", "/corp", "/api", "/v1"]

mcp:
  enabled: true
  advertise_tools: false
  fail_open: true
  tool_timeout_ms: 30000
  tool_cache:
    enabled: true
    backend: memory
    default_ttl_seconds: 300
    max_value_bytes: 65536
    allow:
      - server: corpapi
        tool: fetch
        ttl_seconds: 60
      - server: corp
        tool: from_a
        ttl_seconds: 30
  proof_of_intent_mode: "audit"
  intent_tool_policies:
    - intent: "data_read"
      allowed_tools:
        - "mcp_corpapi_fetch"
        - "mcp_corp_from_a"
        - "mcp_inventory_health"
    - intent: "data_write"
      allowed_tools:
        - "mcp_corp_from_b"
    - intent: "general"
      allowed_tools:
        - "mcp_edge_hi"
  servers:
    - name: corpapi
      enabled: true
      http_tool:
        path: /allowed/toolpath
        method: GET
        tool_name: fetch
    - name: corp
      enabled: true
      http_tools:
        - path: /corp/service-a
          method: GET
          tool_name: from_a
        - path: /corp/service-b
          method: GET
          tool_name: from_b
    - name: inventory
      enabled: true
      http_tool:
        path: /v1/status
        method: GET
        tool_name: health
    - name: edge
      enabled: true
      http_tool:
        path: /api/hi
        method: GET
        tool_name: hi

observability:
  prometheus:
    enabled: true

tpm:
  enabled: true
  buckets:
    - class: "tenant"
      name: "default"
      budget_tokens_per_minute: 50

semantic_cache:
  enabled: true
EOF

echo "Restarting panda with TPM configuration..."
pkill -f panda || true
sleep 2
export PANDA_ADMIN_STATUS_SECRET="123456"
cargo run --bin panda panda.test.tpm.yaml > /dev/null 2>&1 &
sleep 5

# Generate requests to exceed TPM limit
echo "Generating requests to exceed TPM limit..."
for i in {1..20}; do
  # Long prompt to use more tokens
  curl -X POST "http://localhost:8080/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "x-panda-admin-secret: 123456" \
    -d '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Hello "'"$(printf 'x%.0s' {1..1000})"'"}]}' > /dev/null 2>&1
  echo -n "T"
  sleep 0.2
done
echo ""

# Restore original panda.yaml configuration
echo "\nRestoring original panda configuration..."
pkill -f panda || true
sleep 2
export PANDA_ADMIN_STATUS_SECRET="123456"
cargo run --bin panda panda.yaml > /dev/null 2>&1 &
sleep 5

# Clean up test files
rm -f panda.test.tpm.yaml 2>/dev/null

echo "\n================================================="
echo "All metrics should now be populated!"
echo "Check the Grafana dashboard to verify."
