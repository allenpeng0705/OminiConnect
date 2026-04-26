# Rootly Tools

Provider: `rootly` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Rootly is an incident management platform for SRE teams. These tools allow AI agents to manage incidents, responders, postmortems, and track service health.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Rootly
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `incidents:read`, `incidents:write`, `responders:read`, `responders:write`, `postmortems:read`, `postmortems:write`, `services:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `rootly_list_incidents` | List all incidents | GET | /v1/incidents |
| `rootly_get_incident` | Get incident details | GET | /v1/incidents/{incidentId} |
| `rootly_create_incident` | Create an incident | POST | /v1/incidents |
| `rootly_update_incident` | Update an incident | PUT | /v1/incidents/{incidentId} |
| `rootly_list_responders` | List responders | GET | /v1/incidents/{incidentId}/responders |
| `rootly_add_responder` | Add responder to incident | POST | /v1/incidents/{incidentId}/responders |
| `rootly_list_postmortems` | List postmortems | GET | /v1/postmortems |
| `rootly_create_postmortem` | Create a postmortem | POST | /v1/postmortems |
| `rootly_list_services` | List services | GET | /v1/services |
| `rootly_get_incident_timeline` | Get incident timeline | GET | /v1/incidents/{incidentId}/timeline |

---

## Tool Details

### rootly_list_incidents

**What it does**: Returns a list of all incidents.

**When to use**: View ongoing and past incidents.

**Arguments**:
- `limit` (optional): Number of incidents (default 50)
- `status` (optional): Filter by status (triggered, acknowledged, resolved)

**Example LLM prompt**: "List all active incidents"

---

### rootly_get_incident

**What it does**: Gets details of a specific incident.

**When to use**: Get incident status, severity, and details.

**Arguments**:
- `incidentId` (required): The incident ID

**Example LLM prompt**: "Get details for incident inc_abc123"

---

### rootly_create_incident

**What it does**: Creates a new incident.

**When to use**: Report a new issue or outage.

**Arguments**:
- `title` (required): Incident title
- `severity` (required): Severity (critical, high, medium, low)
- `serviceId` (optional): Affected service ID
- `description` (optional): Incident description

**Example LLM prompt**: "Create an incident 'API is down' with severity high"

---

### rootly_update_incident

**What it does**: Updates an existing incident.

**When to use**: Change incident status or severity.

**Arguments**:
- `incidentId` (required): The incident ID
- `status` (optional): Updated status
- `severity` (optional): Updated severity

**Example LLM prompt**: "Update incident inc_abc123 to status resolved"

---

### rootly_list_responders

**What it does**: Returns responders for an incident.

**When to use**: See who's working on an incident.

**Arguments**:
- `incidentId` (required): The incident ID

**Example LLM prompt**: "List responders for incident inc_abc123"

---

### rootly_add_responder

**What it does**: Adds a responder to an incident.

**When to use**: Page additional help during an incident.

**Arguments**:
- `incidentId` (required): The incident ID
- `userId` (required): User ID to add

**Example LLM prompt**: "Add user usr_456 to incident inc_abc123"

---

### rootly_list_postmortems

**What it does**: Returns a list of postmortems.

**When to use**: View historical incident reviews.

**Arguments**:
- `limit` (optional): Number of postmortems (default 50)

**Example LLM prompt**: "List all postmortems"

---

### rootly_create_postmortem

**What it does**: Creates a postmortem for an incident.

**When to use**: Document learnings after an incident.

**Arguments**:
- `incidentId` (required): Related incident ID
- `title` (required): Postmortem title
- `summary` (optional): Incident summary

**Example LLM prompt**: "Create postmortem for incident inc_abc123"

---

### rootly_list_services

**What it does**: Returns a list of all services.

**When to use**: View monitored services.

**Arguments**:
- `limit` (optional): Number of services (default 50)

**Example LLM prompt**: "List all services"

---

### rootly_get_incident_timeline

**What it does**: Returns the timeline of events.

**When to use**: See what happened during an incident.

**Arguments**:
- `incidentId` (required): The incident ID

**Example LLM prompt**: "Get timeline for incident inc_abc123"

---

## Rootly Notes

- Incidents have severity levels and statuses
- Responders are users working on incidents
- Postmortems document incident learnings
- Timeline shows chronological events
- Services are the systems being monitored
