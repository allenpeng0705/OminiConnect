# Nyne AI Tools

Provider: `nyne-ai` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Nyne AI analytics API. They allow AI agents to manage events, persons, organizations, and tags for analytics purposes. **Requires Nyne AI API key authentication.**

## Authentication

**API Key**:
- User provides Nyne AI API key and secret
- Key passed via `X-API-Key` and `X-API-Secret` headers
- Base URL: `https://api.nyne.ai`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `nyne_list_events` | List events | GET | /person/events |
| `nyne_get_event` | Get event details | GET | /person/events/{id} |
| `nyne_create_event` | Create event | POST | /person/events |
| `nyne_update_event` | Update event | PUT | /person/events/{id} |
| `nyne_list_persons` | List persons | GET | /person |
| `nyne_get_person` | Get person details | GET | /person/{id} |
| `nyne_list_organizations` | List organizations | GET | /organization |
| `nyne_get_organization` | Get organization details | GET | /organization/{id} |
| `nyne_create_tag` | Create tag | POST | /tag |
| `nyne_list_tags` | List tags | GET | /tag |

---

## Tool Details

### nyne_list_events

**What it does**: Lists all events in Nyne AI.

**When to use**: Browse event data, analyze activity.

**Arguments**:
- `page` (optional): Page number (default 1)
- `limit` (optional): Items per page (default 20)

**Example LLM prompt**: "List all events from this week"

---

### nyne_get_event

**What it does**: Gets detailed information for a specific event.

**When to use**: View event details, check event properties.

**Arguments**:
- `id` (required): Event ID

**Example LLM prompt**: "Get details for event ABC123"

---

### nyne_create_event

**What it does**: Creates a new event in Nyne AI.

**When to use**: Track activities, record analytics events.

**Arguments**:
- `event` (required): Event type
- `person_id` (optional): Person ID
- `properties` (optional): Event properties

**Example LLM prompt**: "Create an event of type 'purchase' for person P123"

---

### nyne_update_event

**What it does**: Updates an existing event.

**When to use**: Modify event data, update properties.

**Arguments**:
- `id` (required): Event ID
- `event` (optional): Event type
- `properties` (optional): Event properties

**Example LLM prompt**: "Update event ABC123 with new properties"

---

### nyne_list_persons

**What it does**: Lists all persons in Nyne AI.

**When to use**: Browse person directory, find individuals.

**Arguments**:
- `page` (optional): Page number

**Example LLM prompt**: "List all persons in the system"

---

### nyne_get_person

**What it does**: Gets detailed information for a specific person.

**When to use**: View person profile, activity history.

**Arguments**:
- `id` (required): Person ID

**Example LLM prompt**: "Get details for person P123"

---

### nyne_list_organizations

**What it does**: Lists all organizations in Nyne AI.

**When to use**: Browse organizations, find companies.

**Arguments**:
- `page` (optional): Page number

**Example LLM prompt**: "List all organizations"

---

### nyne_get_organization

**What it does**: Gets detailed information for a specific organization.

**When to use**: View organization profile, member list.

**Arguments**:
- `id` (required): Organization ID

**Example LLM prompt**: "Get details for organization O123"

---

### nyne_create_tag

**What it does**: Creates a new tag in Nyne AI.

**When to use**: Categorize entities, create segments.

**Arguments**:
- `name` (required): Tag name
- `type` (optional): Tag type

**Example LLM prompt**: "Create a tag called 'vip-customer'"

---

### nyne_list_tags

**What it does**: Lists all tags in Nyne AI.

**When to use**: View existing tags, find tag categories.

**Arguments**: None

**Example LLM prompt**: "List all available tags"

---

## Nyne AI Notes

- **API Key**: Requires both API key and secret
- **Headers**: Both X-API-Key and X-API-Secret required
- **Events**: Track user activities and interactions
- **Persons**: Individual entity records
- **Organizations**: Company/business entity records
