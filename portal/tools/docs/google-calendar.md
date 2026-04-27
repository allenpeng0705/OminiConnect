# Google Calendar Tools

Provider: `google-calendar` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Calendar API. They allow AI agents to manage calendars, events, ACL rules, and free/busy information. **Requires Google OAuth2 with Calendar permissions.**

## Authentication

**Nango OAUTH2 (Google Calendar)**:
- User authenticates via OAuth2 with Calendar scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://www.googleapis.com/calendar/v3`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_calendar_list_calendars` | List calendars | GET | /calendar/v3/users/me/calendarList |
| `google_calendar_get_calendar` | Get calendar details | GET | /calendar/v3/calendars/{calendarId} |
| `google_calendar_list_events` | List events | GET | /calendar/v3/calendars/{calendarId}/events |
| `google_calendar_get_event` | Get event details | GET | /calendar/v3/calendars/{calendarId}/events/{eventId} |
| `google_calendar_create_event` | Create event | POST | /calendar/v3/calendars/{calendarId}/events |
| `google_calendar_update_event` | Update event | PUT | /calendar/v3/calendars/{calendarId}/events/{eventId} |
| `google_calendar_delete_event` | Delete event | DELETE | /calendar/v3/calendars/{calendarId}/events/{eventId} |
| `google_calendar_list_acl` | List access controls | GET | /calendar/v3/calendars/{calendarId}/acl |
| `google_calendar_list_freebusy` | Get free/busy info | POST | /calendar/v3/freeBusy |
| `google_calendar_list_settings` | List calendar settings | GET | /calendar/v3/users/me/settings |

---

## Tool Details

### google_calendar_list_calendars

**What it does**: Lists all calendars in the user's Google Calendar.

**When to use**: See all available calendars.

**Arguments**: None

**Example LLM prompt**: "List all my calendars"

---

### google_calendar_get_calendar

**What it does**: Gets detailed information about a calendar.

**When to use**: Get calendar summary and description.

**Arguments**:
- `calendarId` (required): Calendar ID

**Example LLM prompt**: "Get details for calendar abc@group.calendar.google.com"

---

### google_calendar_list_events

**What it does**: Lists events from a calendar.

**When to use**: View upcoming events.

**Arguments**:
- `calendarId` (optional): Calendar ID (default: primary)
- `timeMin` (optional): Start time (ISO 8601)
- `timeMax` (optional): End time (ISO 8601)
- `maxResults` (optional): Maximum results (default 20)

**Example LLM prompt**: "List my events for today"

---

### google_calendar_get_event

**What it does**: Gets detailed information about an event.

**When to use**: Get event details and attendees.

**Arguments**:
- `calendarId` (optional): Calendar ID (default: primary)
- `eventId` (required): Event ID

**Example LLM prompt**: "Get event details for event abc123"

---

### google_calendar_create_event

**What it does**: Creates a new event in a calendar.

**When to use**: Schedule meetings and appointments.

**Arguments**:
- `calendarId` (optional): Calendar ID (default: primary)
- `summary` (required): Event title
- `description` (optional): Event description
- `start` (required): Start time (ISO 8601)
- `end` (required): End time (ISO 8601)
- `location` (optional): Location

**Example LLM prompt**: "Create an event titled 'Meeting' from 2pm to 3pm tomorrow"

---

### google_calendar_update_event

**What it does**: Updates an existing event.

**When to use**: Modify event details.

**Arguments**:
- `calendarId` (optional): Calendar ID (default: primary)
- `eventId` (required): Event ID
- `summary` (optional): New title
- `description` (optional): New description

**Example LLM prompt**: "Update event abc123 with new title 'Team Meeting'"

---

### google_calendar_delete_event

**What it does**: Deletes an event from a calendar.

**When to use**: Cancel meetings.

**Arguments**:
- `calendarId` (optional): Calendar ID (default: primary)
- `eventId` (required): Event ID

**Example LLM prompt**: "Delete event abc123"

---

### google_calendar_list_acl

**What it does**: Lists access control rules for a calendar.

**When to use**: See who has access to a calendar.

**Arguments**:
- `calendarId` (optional): Calendar ID (default: primary)

**Example LLM prompt**: "List ACL rules for my calendar"

---

### google_calendar_list_freebusy

**What it does**: Gets free/busy information for calendars.

**When to use**: Find available meeting times.

**Arguments**:
- `timeMin` (required): Start time (ISO 8601)
- `timeMax` (required): End time (ISO 8601)
- `items` (required): Calendar IDs to check

**Example LLM prompt**: "Check free/busy for alice@company.com tomorrow 9am-5pm"

---

### google_calendar_list_settings

**What it does**: Lists user settings for Calendar.

**When to use**: View Calendar preferences.

**Arguments**: None

**Example LLM prompt**: "List my calendar settings"

---

## Google Calendar API Notes

- **Calendar ID**: Usually email address for primary calendar
- **Time format**: ISO 8601 (e.g., "2024-01-15T14:00:00Z")
- **Event ID**: Unique identifier for events
- **Primary calendar**: Use "primary" or actual email
- **ACL rules**: Control who can access calendars
