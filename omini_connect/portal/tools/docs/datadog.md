# Datadog Tools

Provider: `datadog` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Datadog REST API. They allow AI agents to manage monitors, dashboards, hosts, and metrics. Datadog is a leading monitoring and observability platform for cloud infrastructure.

## Authentication

**Nango API Key**:
- User provides Datadog API key and App key via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `monitors_read`, `monitors_write`, `dashboards_read`, `hosts_read`, `metrics_read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `datadog_list_monitors` | List all monitors | GET | /api/v1/monitor |
| `datadog_get_monitor` | Get monitor details | GET | /api/v1/monitor/{monitorId} |
| `datadog_create_monitor` | Create a new monitor | POST | /api/v1/monitor |
| `datadog_update_monitor` | Update a monitor | PUT | /api/v1/monitor/{monitorId} |
| `datadog_delete_monitor` | Delete a monitor | DELETE | /api/v1/monitor/{monitorId} |
| `datadog_list_dashboards` | List all dashboards | GET | /api/v1/dashboard |
| `datadog_get_dashboard` | Get dashboard details | GET | /api/v1/dashboard/{dashboard_id} |
| `datadog_list_hosts` | List all hosts | GET | /api/v1/hosts |
| `datadog_get_host` | Get host details | GET | /api/v1/hosts/{host_id} |
| `datadog_list_metrics` | List available metrics | GET | /api/v1/metrics |

---

## Tool Details

### datadog_list_monitors

**What it does**: Lists all Datadog monitors with optional filtering by status, tags, or monitor type.

**When to use**: Find monitors by status (alerting, warning, ok), filter by tags, or get an overview of all alerting rules.

**Arguments**:
- `monitor_tags` (optional): Filter by tags (e.g., `env:production`)
- `status` (optional): Filter by status (`alert`, `warn`, `no data`, `ok`)
- `type` (optional): Monitor type (`metric alert`, `service check`, `event alert`, etc.)

**Example LLM prompt**: "List all monitors with status alert in the production environment"

---

### datadog_get_monitor

**What it does**: Gets detailed information about a specific Datadog monitor including query, thresholds, and notification settings.

**When to use**: Review monitor configuration before modifying or understanding why an alert fired.

**Arguments**:
- `monitorId` (required): Monitor ID

**Example LLM prompt**: "Get details for monitor 123456"

---

### datadog_create_monitor

**What it does**: Creates a new Datadog monitor. Define the metric query, alert conditions, notifications, and tags.

**When to use**: Set up alerting for new infrastructure, services, or business metrics.

**Arguments**:
- `name` (required): Monitor name
- `type` (required): Monitor type (`metric alert`, `service check`, `event alert`)
- `query` (required): Monitor query (e.g., `avg(last_5m):sum:system.cpu.user{*} > 90`)
- `message` (optional): Alert message (supports `@{username}` for notify)
- `tags` (optional): Tags for the monitor
- `options` (optional): Monitor options (thresholds, notification, etc.)

**Example LLM prompt**: "Create a monitor named 'High CPU Alert' that alerts when avg CPU over 5 minutes is above 90%"

---

### datadog_update_monitor

**What it does**: Updates an existing Datadog monitor. Modify the query, alert conditions, message, or tags.

**When to use**: Adjust thresholds, change notification recipients, or update the query.

**Arguments**:
- `monitorId` (required): Monitor ID
- `name` (optional): New monitor name
- `query` (optional): New monitor query
- `message` (optional): New alert message
- `tags` (optional): New tags
- `options` (optional): New monitor options

**Example LLM prompt**: "Update monitor 123456 to alert at 80% instead of 90% CPU"

---

### datadog_delete_monitor

**What it does**: Permanently deletes a Datadog monitor.

**When to use**: Remove unwanted or deprecated alerting rules. This cannot be undone.

**Arguments**:
- `monitorId` (required): Monitor ID

**Example LLM prompt**: "Delete the monitor 999999"

---

### datadog_list_dashboards

**What it does**: Lists all Datadog dashboards including timeboards and screenboards.

**When to use**: Find available dashboards, see what visualizations are configured.

**Arguments**:
- `filter` (optional): Filter dashboards by name or tag

**Example LLM prompt**: "List all dashboards"

---

### datadog_get_dashboard

**What it does**: Gets detailed information about a specific Datadog dashboard including all widgets and layout.

**When to use**: View dashboard configuration, understand what metrics are being tracked, or duplicate a dashboard.

**Arguments**:
- `dashboard_id` (required): Dashboard ID

**Example LLM prompt**: "Get details for dashboard abc-123-xyz"

---

### datadog_list_hosts

**What it does**: Lists all hosts reporting to Datadog with optional filtering by tags.

**When to use**: See which machines are reporting, find hosts by name or tag, check agent status.

**Arguments**:
- `filter` (optional): Filter by host name or tag
- `count` (optional): Maximum number of hosts to return (default 100)

**Example LLM prompt**: "List all hosts with tag env:production"

---

### datadog_get_host

**What it does**: Gets detailed information about a specific host including metrics, tags, and agent version.

**When to use**: Investigate a specific host, check its configuration, or verify the Datadog agent is running.

**Arguments**:
- `host_id` (required): Host ID or name

**Example LLM prompt**: "Get details for host web-server-01"

---

### datadog_list_metrics

**What it does**: Lists all metrics available in your Datadog account with optional filtering by tag.

**When to use**: Discover available metrics for monitoring, find metrics by tag pattern.

**Arguments**:
- `filter` (optional): Filter metrics by tag (e.g., `env:production`)

**Example LLM prompt**: "List all metrics with tag env:production"

---

## Datadog API Notes

- **Monitor queries**: Use Datadog's query syntax: `avg(last_5m):sum:system.cpu.user{*} > 90`
- **Notifications**: Use `@{username}` in message to notify specific team members
- **Tags**: Tags help organize and filter monitors, hosts, and metrics (e.g., `env:prod`, `region:us-east-1`)
- **Timestamps**: Unix epoch format required for time-based queries
- **Rate limits**: Datadog API has rate limits - monitor creation should not exceed 60/minute
