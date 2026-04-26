# TapClicks Tools

Provider: `tapclicks` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

TapClicks is a marketing analytics platform that aggregates data from multiple sources. **Requires oauth2 client credentials via nango.**

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client credentials flow for app-level access
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tapclicks_list_clients` | List all clients in TapClicks | GET | /clients |
| `tapclicks_get_client` | Get details of a specific client | GET | /clients/{client_id} |
| `tapclicks_list_sources` | List all data sources | GET | /sources |
| `tapclicks_get_source` | Get details of a data source | GET | /sources/{source_id} |
| `tapclicks_list_reports` | List all reports | GET | /reports |
| `tapclicks_get_report` | Get a specific report | GET | /reports/{report_id} |
| `tapclicks_create_report` | Create a new report | POST | /reports |
| `tapclicks_list_dashboards` | List all dashboards | GET | /dashboards |
| `tapclicks_get_dashboard` | Get a specific dashboard | GET | /dashboards/{dashboard_id} |
| `tapclicks_export_data` | Export data from a source | POST | /sources/{source_id}/export |

---

## Tool Details

### tapclicks_list_clients

**What it does**: List all clients in TapClicks

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tapclicks_get_client

**What it does**: Get details of a specific client

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tapclicks_list_sources

**What it does**: List all data sources

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tapclicks_get_source

**What it does**: Get details of a data source

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tapclicks_list_reports

**What it does**: List all reports

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tapclicks_get_report

**What it does**: Get a specific report

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tapclicks_create_report

**What it does**: Create a new report

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tapclicks_list_dashboards

**What it does**: List all dashboards

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tapclicks_get_dashboard

**What it does**: Get a specific dashboard

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tapclicks_export_data

**What it does**: Export data from a source

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.tapclicks.com/v2`
- Docs: https://nango.dev/docs/integrations/all/tapclicks
