# Grafana Tools

Provider: `grafana` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Grafana API. They allow AI agents to browse dashboards, folders, users, teams, alerts, and data sources. Grafana is an open-source observability platform for monitoring and alerting.

## Authentication

**Nango API Key**:
- User provides Grafana service account token via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Domain configured in connection config (e.g., `example.grafana.net`)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `grafana_list_dashboards` | List dashboards | GET | /api/search |
| `grafana_get_dashboard` | Get dashboard details | GET | /api/dashboards/uid/{uid} |
| `grafana_list_folders` | List folders | GET | /api/folders |
| `grafana_get_folder` | Get folder details | GET | /api/folders/{uid} |
| `grafana_list_users` | List org users | GET | /api/org/users |
| `grafana_list_teams` | List teams | GET | /api/teams |
| `grafana_get_team` | Get team details | GET | /api/teams/{teamId} |
| `grafana_list_alerts` | List alerts | GET | /api/alerts |
| `grafana_get_alert` | Get alert details | GET | /api/alerts/{alertId} |
| `grafana_list_data_sources` | List data sources | GET | /api/datasources |

---

## Tool Details

### grafana_list_dashboards

**What it does**: Lists all dashboards in the organization.

**When to use**: Browse available dashboards, find dashboards by folder.

**Arguments**:
- `type` (optional): Type filter (`dash-db` for dashboards, `dash-folder` for folders)
- `folderIds` (optional): Filter by folder ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all dashboards in Grafana"

---

### grafana_get_dashboard

**What it does**: Gets detailed information about a specific dashboard.

**When to use**: View dashboard JSON model, check panel configurations.

**Arguments**:
- `uid` (required): Dashboard UID

**Example LLM prompt**: "Get dashboard with uid abc123"

---

### grafana_list_folders

**What it does**: Lists all folders in the organization.

**When to use**: Browse dashboard organization structure.

**Arguments**: None

**Example LLM prompt**: "List all Grafana folders"

---

### grafana_get_folder

**What it does**: Gets detailed information about a specific folder.

**When to use**: View folder metadata and permissions.

**Arguments**:
- `uid` (required): Folder UID

**Example LLM prompt**: "Get folder with uid myfolder"

---

### grafana_list_users

**What it does**: Lists all users in the organization.

**When to use**: View org members, check user roles.

**Arguments**: None

**Example LLM prompt**: "List all users in the Grafana organization"

---

### grafana_list_teams

**What it does**: Lists all teams in the organization.

**When to use**: Browse teams, find team members.

**Arguments**: None

**Example LLM prompt**: "List all Grafana teams"

---

### grafana_get_team

**What it does**: Gets detailed information about a specific team.

**When to use**: View team members and settings.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "Get team with ID 5"

---

### grafana_list_alerts

**What it does**: Lists all alerts in the organization.

**When to use**: Check active alerts, review alert states.

**Arguments**:
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all active alerts"

---

### grafana_get_alert

**What it does**: Gets detailed information about a specific alert.

**When to use**: View alert rule and conditions.

**Arguments**:
- `alertId` (required): Alert ID

**Example LLM prompt**: "Get alert with ID 12"

---

### grafana_list_data_sources

**What it does**: Lists all configured data sources.

**When to use**: View available data sources for dashboards.

**Arguments**: None

**Example LLM prompt**: "List all Grafana data sources"

---

## Grafana API Notes

- **Domain**: Your Grafana instance domain (e.g., `https://example.grafana.net`)
- **Service Account Token**: Generate from Grafana Settings -> Service Accounts
- **Dashboards**: Identified by UID (not numeric ID)
- **Folders**: Organize dashboards hierarchically
- **Alerts**: Unified alerting uses different endpoints than legacy alerts
