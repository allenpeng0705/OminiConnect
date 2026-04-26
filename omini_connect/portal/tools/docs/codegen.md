# Codegen Tools

Provider: `codegen` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Codegen API. Codegen is a data pipeline platform for connecting and transforming customer data. **Requires Codegen API token.**

## Authentication

**Nango API_KEY**:
- User provides their Codegen API token
- Token passed via `Authorization: Bearer` header
- Base URL: `https://api.codegen.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `codegen_list_organizations` | List organizations | GET | /v1/organizations |
| `codegen_get_organization` | Get organization details | GET | /v1/organizations/{id} |
| `codegen_list_sources` | List sources | GET | /v1/sources |
| `codegen_get_source` | Get source details | GET | /v1/sources/{id} |
| `codegen_list_events` | List events | GET | /v1/events |
| `codegen_get_event` | Get event details | GET | /v1/events/{id} |
| `codegen_list_segments` | List segments | GET | /v1/segments |
| `codegen_get_segment` | Get segment details | GET | /v1/segments/{id} |
| `codegen_list_users` | List users | GET | /v1/users |
| `codegen_get_user` | Get user details | GET | /v1/users/{id} |

---

## Tool Details

### codegen_list_organizations

**What it does**: Lists all organizations.

**When to use**: View your organizations.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List my Codegen organizations"

---

### codegen_get_organization

**What it does**: Gets details of a specific organization.

**When to use**: View organization settings.

**Arguments**:
- `id` (required): Organization ID

**Example LLM prompt**: "Get organization 123 details"

---

### codegen_list_sources

**What it does**: Lists all data sources.

**When to use**: View connected data sources.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Codegen sources"

---

### codegen_get_source

**What it does**: Gets details of a specific source.

**When to use**: View source configuration.

**Arguments**:
- `id` (required): Source ID

**Example LLM prompt**: "Get source 456 details"

---

### codegen_list_events

**What it does**: Lists all events.

**When to use**: View event data.

**Arguments**:
- `source_id` (optional): Filter by source
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List events from source 456"

---

### codegen_get_event

**What it does**: Gets details of a specific event.

**When to use**: View individual event details.

**Arguments**:
- `id` (required): Event ID

**Example LLM prompt**: "Get event 789 details"

---

### codegen_list_segments

**What it does**: Lists all segments.

**When to use**: View user segments.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all segments"

---

### codegen_get_segment

**What it does**: Gets details of a specific segment.

**When to use**: View segment definition.

**Arguments**:
- `id` (required): Segment ID

**Example LLM prompt**: "Get segment 101 details"

---

### codegen_list_users

**What it does**: Lists all users.

**When to use**: View user data.

**Arguments**:
- `segment_id` (optional): Filter by segment
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List users in segment 101"

---

### codegen_get_user

**What it does**: Gets details of a specific user.

**When to use**: View user profile and events.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user 202 details"

---

## Codegen API Notes

- **Organizations**: Top-level containers for data
- **Sources**: Connected data sources (databases, APIs)
- **Events**: Individual data events from sources
- **Segments**: Groups of users based on criteria
- **Users**: User profiles aggregated from events
