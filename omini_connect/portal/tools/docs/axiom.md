# Axiom Tools

Provider: `axiom` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Axiom API. They allow AI agents to query log data, manage datasets, views, alerts, and monitors. Axiom is a cloud-native log management platform that provides powerful analytics and alerting capabilities.

## Authentication

**Nango API_KEY**:
- User provides API token from Axiom
- Token stored in Nango, accessed via `connection_ref`
- Bearer token in Authorization header
- Requires subdomain configuration (API, us-east-1.aws.edge, or eu-central-1.aws.edge)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `axiom_list_datasets` | List available datasets | GET | /v1/datasets |
| `axiom_get_dataset` | Get dataset details | GET | /v1/datasets/{name} |
| `axiom_query_dataset` | Query a dataset with APL | POST | /v1/datasets/{name}/query |
| `axiom_list_views` | List saved views | GET | /v1/views |
| `axiom_get_view` | Get view details | GET | /v1/views/{name} |
| `axiom_list_alerts` | List alerting rules | GET | /v1/alerts |
| `axiom_get_alert` | Get alert details | GET | /v1/alerts/{id} |
| `axiom_list_monitors` | List monitors | GET | /v1/monitors |
| `axiom_get_monitor` | Get monitor details | GET | /v1/monitors/{id} |
| `axiom_get_usage` | Get usage statistics | GET | /v1/usage |

---

## Tool Details

### axiom_list_datasets

**What it does**: Lists all available datasets in Axiom.

**When to use**: Browse available log data, select dataset for querying.

**Arguments**:
- `limit` (optional): Max datasets to return (default 20)

**Example LLM prompt**: "List all datasets in Axiom"

---

### axiom_get_dataset

**What it does**: Gets detailed information about a specific dataset including schema.

**When to use**: Understand dataset structure, check field types.

**Arguments**:
- `name` (required): Dataset name

**Example LLM prompt**: "Get details for the nginx-access dataset"

---

### axiom_query_dataset

**What it does**: Queries a dataset using APL (Axiom Processing Language).

**When to use**: Search logs, analyze data, create visualizations.

**Arguments**:
- `name` (required): Dataset name
- `apl` (required): APL query expression
- `startTime` (optional): Query start time
- `endTime` (optional): Query end time
- `limit` (optional): Max rows to return (default 1000)

**Example LLM prompt**: "Query the nginx-access dataset for 404 errors in the last hour"

---

### axiom_list_views

**What it does**: Lists all saved views in Axiom.

**When to use**: Find pre-configured queries, share analysis.

**Arguments**:
- `limit` (optional): Max views to return (default 20)

**Example LLM prompt**: "List all saved views"

---

### axiom_get_view

**What it does**: Gets details of a specific saved view.

**When to use**: See saved query configuration, use as template.

**Arguments**:
- `name` (required): View name

**Example LLM prompt**: "Get details for view 'error-logs'"

---

### axiom_list_alerts

**What it does**: Lists all alerting rules in Axiom.

**When to use**: View configured alerts, check monitoring setup.

**Arguments**:
- `limit` (optional): Max alerts to return (default 20)

**Example LLM prompt**: "List all configured alerts"

---

### axiom_get_alert

**What it does**: Gets details of a specific alerting rule.

**When to use**: Check alert configuration, modify alert settings.

**Arguments**:
- `id` (required): Alert ID

**Example LLM prompt**: "Get details for alert ID abc-123"

---

### axiom_list_monitors

**What it does**: Lists all monitors in Axiom.

**When to use**: View continuous monitoring setup.

**Arguments**:
- `limit` (optional): Max monitors to return (default 20)

**Example LLM prompt**: "List all monitors"

---

### axiom_get_monitor

**What it does**: Gets details of a specific monitor.

**When to use**: Check monitor configuration, view query and thresholds.

**Arguments**:
- `id` (required): Monitor ID

**Example LLM prompt**: "Get details for monitor ID m-456"

---

### axiom_get_usage

**What it does**: Gets usage statistics for the Axiom account.

**When to use**: Monitor consumption, check billing.

**Arguments**:
- `startTime` (optional): Usage start time
- `endTime` (optional): Usage end time

**Example LLM prompt**: "Get usage for the last 30 days"

---

## Axiom API Notes

- **APL**: Axiom Processing Language is used for querying - similar to KQL (Kusto Query Language)
- **Datasets**: Containers for structured log data with defined schemas
- **Views**: Saved APL queries for quick access
- **Monitors**: Continuous queries that can trigger alerts
- **Ingest Token**: Separate from API token for data ingestion
