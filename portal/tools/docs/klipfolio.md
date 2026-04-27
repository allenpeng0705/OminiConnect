# Klipfolio Tools

Provider: `klipfolio` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Klipfolio API. They allow AI agents to manage dashboards, klips, data sources, and connections. **Requires Klipfolio API key.**

## Authentication

**API Key via Nango**:
- User provides their Klipfolio API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://app.klipfolio.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `klipfolio_get_profile` | Get profile | GET | /api/1.0/profile |
| `klipfolio_list_datasources` | List data sources | GET | /api/1.0/datasources |
| `klipfolio_get_datasource` | Get data source details | GET | /api/1.0/datasources/{datasource_id} |
| `klipfolio_list_klips` | List klips | GET | /api/1.0/klips |
| `klipfolio_get_klip` | Get klip details | GET | /api/1.0/klips/{klip_id} |
| `klipfolio_list_dashboards` | List dashboards | GET | /api/1.0/dashboards |
| `klipfolio_get_dashboard` | Get dashboard details | GET | /api/1.0/dashboards/{dashboard_id} |
| `klipfolio_list_connections` | List connections | GET | /api/1.0/connections |
| `klipfolio_get_connection` | Get connection details | GET | /api/1.0/connections/{connection_id} |
| `klipfolio_list_assets` | List assets | GET | /api/1.0/assets |

---

## Tool Details

### klipfolio_get_profile

**What it does**: Gets the current user profile.

**When to use**: Verify authentication, get account info.

**Arguments**: None

**Example LLM prompt**: "Get my Klipfolio profile"

---

### klipfolio_list_datasources

**What it does**: Lists all data sources.

**When to use**: View data sources, find data connections.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all data sources in Klipfolio"

---

### klipfolio_get_datasource

**What it does**: Gets details for a specific data source.

**When to use**: Get data source information.

**Arguments**:
- `datasource_id` (required): Data source ID

**Example LLM prompt**: "Get details for data source abc123"

---

### klipfolio_list_klips

**What it does**: Lists all klips.

**When to use**: View klips, find visualizations.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all klips in Klipfolio"

---

### klipfolio_get_klip

**What it does**: Gets details for a specific klip.

**When to use**: Get klip information.

**Arguments**:
- `klip_id` (required): Klip ID

**Example LLM prompt**: "Get details for klip xyz789"

---

### klipfolio_list_dashboards

**What it does**: Lists all dashboards.

**When to use**: View dashboards, find metric boards.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all dashboards in Klipfolio"

---

### klipfolio_get_dashboard

**What it does**: Gets details for a specific dashboard.

**When to use**: Get dashboard information.

**Arguments**:
- `dashboard_id` (required): Dashboard ID

**Example LLM prompt**: "Get details for dashboard d1"

---

### klipfolio_list_connections

**What it does**: Lists all connections.

**When to use**: View data connections.

**Arguments**: None

**Example LLM prompt**: "List all connections in Klipfolio"

---

### klipfolio_get_connection

**What it does**: Gets details for a specific connection.

**When to use**: Get connection information.

**Arguments**:
- `connection_id` (required): Connection ID

**Example LLM prompt**: "Get details for connection c1"

---

### klipfolio_list_assets

**What it does**: Lists all assets.

**When to use**: View assets, find resources.

**Arguments**: None

**Example LLM prompt**: "List all assets in Klipfolio"

---

## Klipfolio API Notes

- **Dashboards**: Collections of klips and visualizations
- **Klips**: Individual visualizations and charts
- **Data Sources**: Connections to external data
- **Connections**: Data source configurations
