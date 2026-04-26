# Incident.io Tools

Provider: `incident-io` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the incident.io API. They allow AI agents to manage incidents, severities, status pages, and the service catalog. Incident.io is an incident management platform.

## Authentication

**Nango API Key**:
- User provides API key via Nango Connect
- Key passed in Authorization header as Bearer token
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.incident.io

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `incident_list_incidents` | List incidents | GET | /v1/incidents |
| `incident_get_incident` | Get incident details | GET | /v1/incidents/{id} |
| `incident_list_severities` | List severities | GET | /v1/severities |
| `incident_list_urgencies` | List urgencies | GET | /v1/urgencies |
| `incident_list_status_pages` | List status pages | GET | /v1/status-pages |
| `incident_list_members` | List members | GET | /v1/members |
| `incident_list_roles` | List roles | GET | /v1/roles |
| `incident_list_catalog` | List catalog items | GET | /v1/catalog |
| `incident_get_catalog` | Get catalog item details | GET | /v1/catalog/{id} |
| `incident_list_metrics` | List metrics | GET | /v1/metrics |

---

## Tool Details

### incident_list_incidents

**What it does**: Lists all incidents in incident.io.

**When to use**: Browse active incidents.

**Arguments**:
- `status` (optional): Filter by status (`investigating`, `identified`, `monitoring`, `resolved`)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all investigating incidents"

---

### incident_get_incident

**What it does**: Gets detailed information about a specific incident.

**When to use**: View incident timeline and details.

**Arguments**:
- `id` (required): Incident ID

**Example LLM prompt**: "Get incident with ID abc123"

---

### incident_list_severities

**What it does**: Lists all severity levels in incident.io.

**When to use**: View available severity levels.

**Arguments**: None

**Example LLM prompt**: "List all severities"

---

### incident_list_urgencies

**What it does**: Lists all urgency levels in incident.io.

**When to use**: View available urgency levels.

**Arguments**: None

**Example LLM prompt**: "List all urgencies"

---

### incident_list_status_pages

**What it does**: Lists all status pages in incident.io.

**When to use**: View public status pages.

**Arguments**: None

**Example LLM prompt**: "List all status pages"

---

### incident_list_members

**What it does**: Lists all team members in incident.io.

**When to use**: View incident response team.

**Arguments**: None

**Example LLM prompt**: "List all members"

---

### incident_list_roles

**What it does**: Lists all incident roles in incident.io.

**When to use**: View incident command structure.

**Arguments**: None

**Example LLM prompt**: "List all roles"

---

### incident_list_catalog

**What it does**: Lists all catalog items in incident.io.

**When to use**: Browse service catalog.

**Arguments**:
- `type` (optional): Filter by type

**Example LLM prompt**: "List all catalog items"

---

### incident_get_catalog

**What it does**: Gets detailed information about a specific catalog item.

**When to use**: View service details.

**Arguments**:
- `id` (required): Catalog item ID

**Example LLM prompt**: "Get catalog item with ID cat456"

---

### incident_list_metrics

**What it does**: Lists all metrics in incident.io.

**When to use**: View available metrics.

**Arguments**: None

**Example LLM prompt**: "List all metrics"

---

## Incident.io API Notes

- **API Base URL**: https://api.incident.io
- **Auth Mode**: API Key in Bearer token
- **Incidents**: Active and past incidents
- **Severities**: Severity level definitions
- **Urgencies**: Urgency level definitions
- **Status Pages**: Public status pages
- **Members**: Team members
- **Roles**: Incident command roles
- **Catalog**: Service catalog items
- **Metrics**: Incident metrics
