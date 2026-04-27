# Zoho Calendar Tools

Provider: `zoho-calendar` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zoho Calendar is an online calendar service. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoho Calendar
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_calendar_list_calendars` | List all calendars | GET | /v1/calendars |
| `zoho_calendar_get_calendar` | Get calendar details | GET | /v1/calendars/{calendar_id} |
| `zoho_calendar_list_events` | List all events | GET | /v1/calendars/{calendar_id}/events |
| `zoho_calendar_get_event` | Get event details | GET | /v1/calendars/{calendar_id}/events/{event_id} |
| `zoho_calendar_create_event` | Create a new event | POST | /v1/calendars/{calendar_id}/events |
| `zoho_calendar_update_event` | Update an event | PUT | /v1/calendars/{calendar_id}/events/{event_id} |
| `zoho_calendar_delete_event` | Delete an event | DELETE | /v1/calendars/{calendar_id}/events/{event_id} |
| `zoho_calendar_list_tasks` | List all tasks | GET | /v1/calendars/{calendar_id}/tasks |
| `zoho_calendar_get_task` | Get task details | GET | /v1/calendars/{calendar_id}/tasks/{task_id} |
| `zoho_calendar_create_task` | Create a new task | POST | /v1/calendars/{calendar_id}/tasks |

---

## Tool Details

### zoho_calendar_list_calendars

**What it does**: List all calendars

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_calendar_get_calendar

**What it does**: Get calendar details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_calendar_list_events

**What it does**: List all events

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_calendar_get_event

**What it does**: Get event details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_calendar_create_event

**What it does**: Create a new event

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_calendar_update_event

**What it does**: Update an event

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_calendar_delete_event

**What it does**: Delete an event

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_calendar_list_tasks

**What it does**: List all tasks

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_calendar_get_task

**What it does**: Get task details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_calendar_create_task

**What it does**: Create a new task

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://calendar.zoho.com/api`
- Docs: https://nango.dev/docs/integrations/all/zoho-calendar
