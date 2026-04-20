#!/usr/bin/env python3
"""
Generate traffic to simulate all empty Grafana dashboard panels
"""

import os
import sys
import time
import json
import requests
import subprocess


def run_command(cmd):
    """Run a command and return the output"""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        return result.stdout, result.stderr, result.returncode
    except Exception as e:
        return "", str(e), 1


def start_panda(config_file):
    """Start Panda with the specified configuration"""
    print(f"Starting panda with {config_file}...")
    # Kill any existing panda process
    run_command("pkill -f panda || true")
    time.sleep(2)
    # Start panda in the background
    env = os.environ.copy()
    env["PANDA_ADMIN_STATUS_SECRET"] = "123456"
    cmd = f"cargo run --bin panda {config_file} > /dev/null 2>&1 &"
    run_command(cmd)
    time.sleep(5)


def generate_semantic_cache_traffic():
    """Generate semantic cache traffic"""
    print("\n2. Generating AI Gateway - Semantic Cache traffic...")
    
    # First generate some unique requests to populate the cache
    for i in range(5):
        try:
            response = requests.post(
                "http://localhost:8080/v1/chat/completions",
                headers={
                    "Content-Type": "application/json",
                    "x-panda-admin-secret": "123456"
                },
                json={
                    "model": "gpt-3.5-turbo",
                    "messages": [{"role": "user", "content": "What is the capital of France?"}]
                },
                timeout=5
            )
            print("M", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.1)
    
    # Now generate identical requests to get cache hits
    for i in range(10):
        try:
            response = requests.post(
                "http://localhost:8080/v1/chat/completions",
                headers={
                    "Content-Type": "application/json",
                    "x-panda-admin-secret": "123456"
                },
                json={
                    "model": "gpt-3.5-turbo",
                    "messages": [{"role": "user", "content": "What is the capital of France?"}]
                },
                timeout=5
            )
            print("H", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.1)
    print()


def generate_mcp_tool_cache_traffic():
    """Generate MCP tool cache traffic"""
    print("\n3. Generating MCP Tool Cache traffic...")
    
    # Use a fixed session ID
    session_id = f"test-session-{int(time.time())}"
    
    # Generate tool calls to populate cache
    print("Populating tool cache...")
    for i in range(5):
        try:
            response = requests.post(
                "http://localhost:8080/mcp",
                headers={
                    "Content-Type": "application/json",
                    "Accept": "application/json, text/event-stream",
                    "Mcp-Session-Id": session_id
                },
                json={
                    "jsonrpc": "2.0",
                    "id": i + 1,
                    "method": "tools/call",
                    "params": {
                        "name": "mcp_corpapi_fetch",
                        "arguments": {"query": "test-query"}
                    }
                },
                timeout=5
            )
            print("S", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.2)
    print()
    
    # Generate identical tool calls to get cache hits
    print("Generating cache hits...")
    for i in range(10):
        try:
            response = requests.post(
                "http://localhost:8080/mcp",
                headers={
                    "Content-Type": "application/json",
                    "Accept": "application/json, text/event-stream",
                    "Mcp-Session-Id": session_id
                },
                json={
                    "jsonrpc": "2.0",
                    "id": i + 10,
                    "method": "tools/call",
                    "params": {
                        "name": "mcp_corpapi_fetch",
                        "arguments": {"query": "test-query"}
                    }
                },
                timeout=5
            )
            print("H", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.1)
    print()
    
    # Generate different tool calls to get cache misses
    print("Generating cache misses...")
    for i in range(5):
        try:
            response = requests.post(
                "http://localhost:8080/mcp",
                headers={
                    "Content-Type": "application/json",
                    "Accept": "application/json, text/event-stream",
                    "Mcp-Session-Id": session_id
                },
                json={
                    "jsonrpc": "2.0",
                    "id": i + 20,
                    "method": "tools/call",
                    "params": {
                        "name": "mcp_corpapi_fetch",
                        "arguments": {"query": f"test-query-{i}"}
                    }
                },
                timeout=5
            )
            print("M", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.2)
    print()


def generate_mcp_governance_traffic():
    """Generate MCP agent governance traffic"""
    print("\n4. Generating MCP Agent Governance events...")
    
    intents = ["data_read", "data_write", "general"]
    intent_tools = {
        "data_read": ["mcp_corpapi_fetch", "mcp_corp_from_a", "mcp_inventory_health"],
        "data_write": ["mcp_corp_from_b"],
        "general": ["mcp_edge_hi"]
    }
    
    for i in range(20):
        intent = intents[i % len(intents)]
        tools = intent_tools[intent]
        tool_name = tools[i % len(tools)]
        
        try:
            response = requests.post(
                "http://localhost:8080/mcp",
                headers={
                    "Content-Type": "application/json",
                    "Accept": "application/json, text/event-stream",
                    "Mcp-Session-Id": f"governance-session-{i}"
                },
                json={
                    "jsonrpc": "2.0",
                    "id": i,
                    "method": "tools/call",
                    "params": {
                        "name": tool_name,
                        "arguments": {"query": f"test-{i}"}
                    }
                },
                timeout=5
            )
            print(".", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.2)
    print()


def generate_auth_failures():
    """Generate API gateway auth failures"""
    print("\n5. Generating API Gateway - Auth Failures...")
    
    # Generate requests without admin secret
    for i in range(15):
        try:
            response = requests.get("http://localhost:8080/metrics", timeout=2)
            print("F", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.1)
    
    # Generate requests with invalid admin secret
    for i in range(10):
        try:
            response = requests.get(
                "http://localhost:8080/metrics",
                headers={"x-panda-admin-secret": "invalid"},
                timeout=2
            )
            print("I", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.1)
    print()


def generate_rate_limit_exceeded():
    """Generate API gateway rate limit exceeded events"""
    print("\n6. Generating API Gateway - Rate Limit Exceeded events...")
    print("Sending requests to exceed rate limit...")
    
    # Send requests faster than the rate limit (10 RPS for /metrics)
    for i in range(30):
        try:
            response = requests.get(
                "http://localhost:8080/metrics",
                headers={"x-panda-admin-secret": "123456"},
                timeout=2
            )
            print("R", end="")
        except Exception as e:
            print("E", end="")
        # No sleep - send as fast as possible
    print()


def generate_ingress_requests():
    """Generate API gateway ingress requests"""
    print("\n7. Generating API Gateway - Ingress Requests...")
    
    paths = ["/v1/chat/completions", "/mcp", "/health", "/metrics"]
    
    for i in range(50):
        path = paths[i % len(paths)]
        
        try:
            if path == "/v1/chat/completions":
                # Chat completion request
                response = requests.post(
                    f"http://localhost:8080{path}",
                    headers={
                        "Content-Type": "application/json",
                        "x-panda-admin-secret": "123456"
                    },
                    json={
                        "model": "gpt-3.5-turbo",
                        "messages": [{"role": "user", "content": "Hello"}]
                    },
                    timeout=5
                )
            elif path == "/mcp":
                # MCP request
                response = requests.post(
                    f"http://localhost:8080{path}",
                    headers={
                        "Content-Type": "application/json",
                        "Accept": "application/json, text/event-stream",
                        "Mcp-Session-Id": f"ingress-session-{i}"
                    },
                    json={
                        "jsonrpc": "2.0",
                        "id": 1,
                        "method": "ping",
                        "params": {}
                    },
                    timeout=5
                )
            else:
                # GET request
                response = requests.get(
                    f"http://localhost:8080{path}",
                    headers={"x-panda-admin-secret": "123456"},
                    timeout=2
                )
            print(".", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.05)
    print()


def generate_model_failover():
    """Generate AI gateway model failover events"""
    print("\n8. Generating AI Gateway - Model Failover events...")
    
    # Start panda with failover configuration
    start_panda("panda.test.failover.yaml")
    
    # Generate requests to trigger failover
    print("Generating requests to trigger model failover...")
    for i in range(15):
        try:
            response = requests.post(
                "http://localhost:8080/v1/chat/completions",
                headers={
                    "Content-Type": "application/json",
                    "x-panda-admin-secret": "123456"
                },
                json={
                    "model": "gpt-3.5-turbo",
                    "messages": [{"role": "user", "content": "Hello"}]
                },
                timeout=5
            )
            print("F", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.3)
    print()


def generate_tpm_rejects():
    """Generate TPM rejects"""
    print("\n9. Generating TPM Rejects...")
    
    # Create TPM test configuration
    tpm_config = """
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
"""
    
    # Write the TPM config file
    with open("panda.test.tpm.yaml", "w") as f:
        f.write(tpm_config)
    
    # Start panda with TPM configuration
    start_panda("panda.test.tpm.yaml")
    
    # Generate requests to exceed TPM limit
    print("Generating requests to exceed TPM limit...")
    for i in range(20):
        # Long prompt to use more tokens
        long_prompt = "Hello " + "x" * 1000
        try:
            response = requests.post(
                "http://localhost:8080/v1/chat/completions",
                headers={
                    "Content-Type": "application/json",
                    "x-panda-admin-secret": "123456"
                },
                json={
                    "model": "gpt-3.5-turbo",
                    "messages": [{"role": "user", "content": long_prompt}]
                },
                timeout=5
            )
            print("T", end="")
        except Exception as e:
            print("E", end="")
        time.sleep(0.2)
    print()
    
    # Clean up
    try:
        os.remove("panda.test.tpm.yaml")
    except:
        pass


def main():
    """Main function"""
    print("Generating traffic to simulate all metrics...")
    print("=================================================")
    
    # 1. Start panda with test configuration
    print("\n1. Starting panda with test configuration...")
    start_panda("panda.test.yaml")
    
    # 2. Generate semantic cache traffic
    generate_semantic_cache_traffic()
    
    # 3. Generate MCP tool cache traffic
    generate_mcp_tool_cache_traffic()
    
    # 4. Generate MCP governance traffic
    generate_mcp_governance_traffic()
    
    # 5. Generate auth failures
    generate_auth_failures()
    
    # 6. Generate rate limit exceeded events
    generate_rate_limit_exceeded()
    
    # 7. Generate ingress requests
    generate_ingress_requests()
    
    # 8. Generate model failover events
    generate_model_failover()
    
    # 9. Generate TPM rejects
    generate_tpm_rejects()
    
    # 10. Restore original configuration
    print("\nRestoring original panda configuration...")
    start_panda("panda.yaml")
    
    print("\n=================================================")
    print("All metrics should now be populated!")
    print("Check the Grafana dashboard to verify.")


if __name__ == "__main__":
    main()
