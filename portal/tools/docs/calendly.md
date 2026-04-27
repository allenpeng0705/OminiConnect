# Calendly Tools

Provider: `calendly` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Calendly API. They allow AI agents to manage event types, scheduled events, invitees, and user availability. Calendly is a popular scheduling and appointment booking platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Calendly
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read_event_types`, `write_event_types`, `read_scheduled_events`, `write_scheduled_events`, `read_users`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `calendly_list_event_types` | List event types | GET | /event_types |
| `calendly_get_event_type` | Get event type details | GET | /event_types/{event_type_id} |
| `calendly_list_scheduled_events` | List scheduled events | GET | /scheduled_events |
| `calendly_get_scheduled_event` | Get scheduled event details | GET | /scheduled_events/{event_id} |
| `calendly_cancel_scheduled_event` | Cancel a scheduled event | POST | /scheduled_events/{event_id}/cancellation |
| `calendly_list_invitees` | List event invitees | GET | /scheduled_events/{event_id}/invitees |
| `calendly_get_invitee` | Get invitee details | GET | /scheduled_events/{event_id}/invitees/{invitee_id} |
| `calendly_get_current_user` | Get current user | GET | /users/me |
| `calendly_list_organization_event_types` | List organization event types | GET | /organizations/{organization_id}/event_types |
| `calendly_get_user_availability` | Get user availability | GET | /users/{user_id}/availability |

---

## Tool Details

### calendly_list_event_types

**What it does**: Lists all event types (appointment types) available for a user or organization.

**When to use**: Find available appointment types, check durations and descriptions.

**Arguments**:
- `user` (optional): Filter by user URI
- `organization` (optional): Filter by organization URI
- `count` (optional): Max event types to return (default 100)

**Example LLM prompt**: "What event types are available for booking with Dr. Smith?"

---

### calendly_get_event_type

**What it does**: Gets detailed information about a specific event type.

**When to use**: Get event type details, description, pricing, location settings.

**Arguments**:
- `event_type_id` (required): Event type ID or URI

**Example LLM prompt**: "Get details for the initial consultation event type"

---

### calendly_list_scheduled_events

**What it does**: Lists scheduled events (booked appointments).

**When to use**: Find upcoming or past appointments, filter by date or status.

**Arguments**:
- `user` (optional): Filter by user URI
- `min_start_time` (optional): Events after this time (ISO 8601)
- `max_start_time` (optional): Events before this time (ISO 8601)
- `status` (optional): Filter by status: active, canceled
- `count` (optional): Max events to return (default 100)

**Example LLM prompt**: "List all upcoming events for next week"

---

### calendly_get_scheduled_event

**What it does**: Gets detailed information about a specific scheduled event.

**When to use**: Get event details, location, invitee information.

**Arguments**:
- `event_id` (required): Event ID or URI

**Example LLM prompt**: "Get details for event abc-123"

---

### calendly_cancel_scheduled_event

**What it does**: Cancels a scheduled event and notifies invitees.

**When to use**: Cancel a booking, reschedule an appointment.

**Arguments**:
- `event_id` (required): Event ID or URI
- `reason` (optional): Cancellation reason

**Example LLM prompt**: "Cancel the meeting with john@example.com tomorrow"

---

### calendly_list_invitees

**What it does**: Lists all invitees for a specific scheduled event.

**When to use**: See who has booked a particular event.

**Arguments**:
- `event_id` (required): Event ID or URI
- `count` (optional): Max invitees to return (default 100)

**Example LLM prompt**: "Who is attending the sales call at 3pm?"

---

### calendly_get_invitee

**What it does**: Gets detailed information about a specific invitee.

**When to use**: Get invitee contact details, answers to event questions.

**Arguments**:
- `event_id` (required): Event ID or URI
- `invitee_id` (required): Invitee ID or URI

**Example LLM prompt**: "Get details for the invitee on event abc-123"

---

### calendly_get_current_user

**What it does**: Gets the current authenticated user's profile information.

**When to use**: Get the user's own profile, scheduling links.

**Arguments**: None

**Example LLM prompt**: "What is my Calendly profile?"

---

### calendly_list_organization_event_types

**What it does**: Lists all event types for an organization.

**When to use**: Browse all services offered by a company or team.

**Arguments**:
- `organization_id` (required): Organization ID or URI
- `active` (optional): Filter to only active event types (default true)
- `count` (optional): Max event types to return (default 100)

**Example LLM prompt**: "What appointment types does Acme Corp offer?"

---

### calendly_get_user_availability

**What it does**: Gets availability schedule for a user or event type.

**When to use**: Check when someone is available for booking.

**Arguments**:
- `user_id` (required): User ID or URI
- `start_date` (optional): Start date for availability check (YYYY-MM-DD)
- `end_date` (optional): End date for availability check (YYYY-MM-DD)

**Example LLM prompt**: "When is Dr. Smith available next week?"

---

## Calendly API Notes

- **URIs**: Calendly uses URI-format IDs (e.g., `https://api.calendly.com/users/ABC123`)
- **Pagination**: Use `count` parameter to limit results
- **Status values**: `active` for confirmed, `canceled` for cancelled events
- **Availability**: Check `min_start_time` and `max_start_time` for date filtering
