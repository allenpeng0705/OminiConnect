# Calendary Tools

Provider: `calendary` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Calendary API. They allow AI agents to manage events, calendars, availability, and scheduling links. Calendary is a calendar management platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Calendary
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `calendary_list_events` | List events | GET | /events |
| `calendary_get_event` | Get event details | GET | /events/{event_id} |
| `calendary_create_event` | Create an event | POST | /events |
| `calendary_update_event` | Update an event | PUT | /events/{event_id} |
| `calendary_delete_event` | Delete an event | DELETE | /events/{event_id} |
| `calendary_list_calendars` | List calendars | GET | /calendars |
| `calendary_get_calendar` | Get calendar details | GET | /calendars/{calendar_id} |
| `calendary_get_availability` | Get available time slots | GET | /availability |
| `calendary_create_scheduling_link` | Create a scheduling link | POST | /scheduling-links |
| `calendary_list_scheduling_links` | List scheduling links | GET | /scheduling-links |

---

## Tool Details

### calendary_list_events

**What it does**: Lists all events with optional filters.

**When to use**: Find events by date, calendar, or status.

**Arguments**:
- `calendar_id` (optional): Filter by calendar ID
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)
- `status` (optional): Filter by status (confirmed, tentative, cancelled)
- `count` (optional): Max events to return

**Example LLM prompt**: "List all events for calendar abc123 next week"

---

### calendary_get_event

**What it does**: Gets detailed information about a specific event.

**When to use**: Get event details, description, location.

**Arguments**:
- `event_id` (required): Event ID

**Example LLM prompt**: "Get details for event xyz789"

---

### calendary_create_event

**What it does**: Creates a new event on a calendar.

**When to use**: Add a new event to the calendar.

**Arguments**:
- `calendar_id` (required): Calendar ID
- `title` (required): Event title
- `description` (optional): Event description
- `start_time` (required): Start time (ISO 8601)
- `end_time` (required): End time (ISO 8601)
- `location` (optional): Event location

**Example LLM prompt**: "Create a team meeting on calendar abc123 from 2pm to 3pm tomorrow"

---

### calendary_update_event

**What it does**: Updates an existing event.

**When to use**: Modify event details, reschedule.

**Arguments**:
- `event_id` (required): Event ID
- `title` (optional): Event title
- `description` (optional): Event description
- `start_time` (optional): Start time (ISO 8601)
- `end_time` (optional): End time (ISO 8601)
- `location` (optional): Event location

**Example LLM prompt**: "Update event xyz789 to start at 3pm instead"

---

### calendary_delete_event

**What it does**: Deletes an event from the calendar.

**When to use**: Remove an event, cancel a meeting.

**Arguments**:
- `event_id` (required): Event ID

**Example LLM prompt**: "Delete event xyz789"

---

### calendary_list_calendars

**What it does**: Lists all calendars available to the user.

**When to use**: Find available calendars, list all calendars.

**Arguments**:
- `count` (optional): Max calendars to return

**Example LLM prompt**: "List all my calendars"

---

### calendary_get_calendar

**What it does**: Gets detailed information about a specific calendar.

**When to use**: Get calendar settings, description, timezone.

**Arguments**:
- `calendar_id` (required): Calendar ID

**Example LLM prompt**: "Get details for calendar abc123"

---

### calendary_get_availability

**What it does**: Gets available time slots for a calendar on specific dates.

**When to use**: Find open slots, check availability for scheduling.

**Arguments**:
- `calendar_id` (required): Calendar ID
- `start_date` (required): Start date (YYYY-MM-DD)
- `end_date` (required): End date (YYYY-MM-DD)
- `duration` (optional): Required slot duration in minutes

**Example LLM prompt**: "What time slots are available on calendar abc123 next week for 30 minute meetings?"

---

### calendary_create_scheduling_link

**What it does**: Creates a new scheduling link for booking appointments.

**When to use**: Create a shareable link that others can use to schedule time.

**Arguments**:
- `calendar_id` (required): Calendar ID
- `name` (required): Name for the scheduling link
- `duration` (required): Meeting duration in minutes
- `description` (optional): Description shown to booking parties

**Example LLM prompt**: "Create a scheduling link for 30 minute meetings on my calendar"

---

### calendary_list_scheduling_links

**What it does**: Lists all scheduling links for the user.

**When to use**: View all created scheduling links.

**Arguments**:
- `count` (optional): Max scheduling links to return

**Example LLM prompt**: "List all my scheduling links"

---

## Calendary API Notes

- **Pagination**: Use `count` parameter to limit results
- **IDs**: String identifiers for events and calendars
- **Time format**: ISO 8601 format for datetime values
- **Scheduling links**: Create shareable booking links for easy appointment scheduling