# Panda Unified Gateway - Grafana Dashboards & Recording Rules

This directory contains Grafana dashboards and Prometheus recording rules for monitoring the Panda Unified Gateway (API + MCP + AI).

## Contents

### 1. Recording Rules

**File:** `recording_rules.agent_fleet.yaml`

Prometheus recording rules that pre-compute common queries for better dashboard performance and alerting.

#### Rule Groups

**panda_unified_gateway_rates** (30s interval):
- `panda:tpm_rejects:rate5m` - TPM rejection rate by bucket_class
- `panda:mcp_rounds_exceeded:rate1h` - MCP tool rounds exceeded rate
- `panda:mcp_tool_cache:hit_rate5m` - MCP tool cache hit rate
- `panda:semantic_cache:hit_rate5m` - AI semantic cache hit rate
- `panda:api_gateway:ingress_rate5m` - API Gateway ingress request rate
- `panda:api_gateway:egress_rate5m` - API Gateway egress request rate
- `panda:api_gateway:auth_failures:rate5m` - API Gateway auth failure rate
- `panda:model_failover:midstream_retries:rate1h` - Model failover retry rate
- `panda:ops_auth:deny_ratio5m` - Ops endpoint auth denial ratio

**panda_unified_gateway_alerts** (60s interval):
- `PandaHighTPMRejectionRate` - High TPM rejection rate warning
- `PandaMCPRoundsCapExceeded` - MCP rounds cap exceeded frequently
- `PandaLowToolCacheHitRate` - Low tool cache hit rate
- `PandaAPIGatewayHighAuthFailures` - High API Gateway auth failures
- `PandaHighOpsAuthDenialRatio` - High ops endpoint auth denial ratio
- `PandaModelFailoverMidstreamRetries` - Model failover midstream retries elevated

### 2. Grafana Dashboard

**File:** `panda_agent_fleet.json`

Unified dashboard for monitoring all three gateway functions.

#### Dashboard Sections

**1. Unified Gateway Overview**
- Request rate across all gateways
- Ops auth denial ratio
- TPM rejects by bucket_class

**2. AI Gateway**
- Semantic cache hit rate
- Semantic cache operations (hits, misses, stores)

**3. MCP Gateway**
- Tool cache hit rate
- Tool cache operations (hits, misses, stores, bypasses)
- Agent governance (rounds exceeded, tools filtered, intent denied)

**4. API Gateway**
- Ingress requests by path
- Auth failures by reason
- Egress requests by host
- Rate limit exceeded

## Installation

### 1. Load Recording Rules into Prometheus

Add the recording rules to your Prometheus configuration:

```yaml
# prometheus.yml
rule_files:
  - /path/to/panda/grafana/recording_rules.agent_fleet.yaml
```

Restart Prometheus to load the rules:

```bash
# Check configuration
promtool check config prometheus.yml

# Restart Prometheus
systemctl restart prometheus
# or
docker restart prometheus
```

Verify rules are loaded:

```bash
# Check Prometheus UI at http://prometheus:9090/rules
# Or use the API
curl http://prometheus:9090/api/v1/rules
```

### 2. Import Dashboard into Grafana

**Option A: Via Grafana UI**

1. Open Grafana UI
2. Navigate to Dashboards → Import
3. Upload the `panda_agent_fleet.json` file
4. Select your Prometheus datasource
5. Click Import

**Option B: Via Grafana API**

```bash
# Set your Grafana API key and URL
GRAFANA_URL="http://grafana:3000"
GRAFANA_API_KEY="your-api-key"

# Import the dashboard
curl -X POST \
  -H "Authorization: Bearer $GRAFANA_API_KEY" \
  -H "Content-Type: application/json" \
  -d @panda_agent_fleet.json \
  $GRAFANA_URL/api/dashboards/db
```

**Option C: Via Kubernetes ConfigMap**

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: grafana-dashboard-panda
  labels:
    grafana_dashboard: "1"
data:
  panda-unified-gateway.json: |
    <paste dashboard JSON content here>
```

### 3. Configure Prometheus Datasource

Ensure your Grafana has a Prometheus datasource configured:

1. Navigate to Configuration → Data Sources
2. Add Prometheus datasource
3. Set URL to your Prometheus instance (e.g., `http://prometheus:9090`)
4. Click Save & Test

## Usage

### Viewing the Dashboard

1. Open Grafana UI
2. Navigate to Dashboards
3. Find "Panda Unified Gateway" dashboard
4. Select time range (default: last 1 hour)
5. Set refresh interval (default: 10s)

### Key Metrics to Monitor

#### Unified Gateway
- **Request Rate**: Overall traffic across all gateways
- **Ops Auth Denial Ratio**: Should be low (<10%)
- **TPM Rejects**: Token budget rejections by bucket_class

#### AI Gateway
- **Semantic Cache Hit Rate**: Should be >15% for cost savings
- **Cache Operations**: Monitor hit/miss/store patterns

#### MCP Gateway
- **Tool Cache Hit Rate**: Should be >20% for deterministic tools
- **Agent Governance**: Watch for rounds exceeded and intent denials
- **Tool Cache Bypasses**: Check reasons for cache bypasses

#### API Gateway
- **Ingress Requests**: Traffic by path prefix
- **Auth Failures**: Monitor authentication issues
- **Egress Requests**: Backend service traffic
- **Rate Limit Exceeded**: Should be minimal

### Alerting

The recording rules include alert definitions. To enable:

1. Load the recording rules into Prometheus
2. Configure Alertmanager to receive alerts
3. Adjust alert thresholds based on your environment

Example Alertmanager configuration:

```yaml
route:
  receiver: 'panda-alerts'
  match:
    gateway: unified

receivers:
  - name: 'panda-alerts'
    slack_configs:
      - channel: '#panda-alerts'
        send_resolved: true
```

## Customization

### Adding Custom Panels

1. Edit the dashboard in Grafana UI
2. Add new panel with your PromQL query
3. Save dashboard
4. Export JSON to update `panda_agent_fleet.json`

### Modifying Recording Rules

1. Edit `recording_rules.agent_fleet.yaml`
2. Reload Prometheus configuration:
   ```bash
   curl -X POST http://prometheus:9090/-/reload
   ```

### Creating New Alerts

1. Add alert rule to `panda_unified_gateway_alerts` group
2. Set appropriate severity and thresholds
3. Configure Alertmanager routing

## Troubleshooting

### No Data in Dashboard

1. Verify Prometheus is scraping Panda metrics:
   ```bash
   curl http://prometheus:9090/api/v1/targets
   ```
2. Check Panda is exposing metrics at `/metrics` endpoint
3. Verify datasource configuration in Grafana

### Recording Rules Not Working

1. Check Prometheus logs for rule evaluation errors
2. Verify metric names match Panda exports
3. Check rule syntax with `promtool check rules`

### Alerts Not Firing

1. Verify Alertmanager is configured
2. Check alert rule conditions
3. Review Alertmanager logs

## Additional Resources

- [Panda Documentation](../docs/)
- [Prometheus Recording Rules](https://prometheus.io/docs/prometheus/latest/configuration/recording_rules/)
- [Grafana Dashboard Best Practices](https://grafana.com/docs/grafana/latest/dashboards/)
- [Panda Runbooks](../docs/runbooks/)

## Support

For issues with:
- **Panda Gateway**: Check [docs/runbooks/agent_fleet_oncall.md](../docs/runbooks/agent_fleet_oncall.md)
- **Prometheus**: Check Prometheus documentation and logs
- **Grafana**: Check Grafana documentation and logs
