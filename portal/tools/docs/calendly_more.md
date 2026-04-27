# Calendly More Tools

Provider: `calendly_more` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Calendly Scheduling API. They allow AI agents to manage event types, scheduled events, invitees, availability, and organization-level scheduling resources. Calendly is a popular scheduling platform for professional meetings and appointments.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Calendly
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `scheduling:r`, `scheduling:w` for read/write operations

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `calendly_more_list_event_types` | List all event types | GET | /event_types |
| `calendly_more_get_event_type` | Get event type details | GET | /event_types/{uuid} |
| `calendly_more_create_event_type` | Create a new event type | POST | /event_types |
| `calendly_more_list_scheduled_events` | List scheduled events | GET | /scheduled_events |
| `calendly_more_get_scheduled_event` | Get scheduled event details | GET | /scheduled_events/{uuid} |
| `calendly_more_cancel_scheduled_event` | Cancel a scheduled event | POST | /scheduled_events/{uuid}/cancel |
| `calendly_more_list_invitees` | List event invitees | GET | /scheduled_events/{uuid}/invitees |
| `calendly_more_get_invitee` | Get invitee details | GET | /invitees/{uuid} |
| `calendly_more_get_user_availability` | Get user availability | GET | /users/{user_id}/availability |
| `calendly_more_list_organization_event_types` | List org event types | GET | /organizations/{organization_id}/event_types |

---

## Tool Details

### calendly_more_list_event_types

**What it does**: Lists all event types for the authenticated user or organization. Event types define the scheduling options available to invitees.

**When to use**: Find available meeting types, see what scheduling options exist before creating events.

**Arguments**:
- `user` (optional): Filter by user URI
- `active` (optional): Filter by active status
- `count` (optional): Number of results to return (default 100)
- `page_token` (optional): Pagination token

**Example LLM prompt**: "What event types are available for scheduling?"

---

### calendly_more_get_event_type

**What it does**: Gets detailed information about a specific event type including duration, description, and scheduling options.

**When to use**: Before scheduling, understand what a meeting type involves and its settings.

**Arguments**:
- `uuid` (required): Event type UUID

**Example LLM prompt**: "What are the details of the 30-minute consultation event type?"

---

### calendly_more_create_event_type

**What it does**: Creates a new event type with name, duration, description, and scheduling location options.

**When to use**: Add a new meeting type to your scheduling options.

**Arguments**:
- `name` (required): Event type name
- `duration_minutes` (optional): Duration in minutes
- `description` (optional): Event type description
- `color` (optional): Color for the event type (hex code)
- `location` (optional): Location settings
- `scheduling_url` (optional): Custom scheduling URL

**Example LLM prompt**: "Create a new 45-minute discovery call event type"

---

### calendly_more_list_scheduled_events

**What it does**: Lists all scheduled events (invitations) for the authenticated user. Filter by status, date range, or event type.

**When to use**: See upcoming meetings, find events by date, check for cancellations.

**Arguments**:
- `user` (optional): Filter by user URI
- `start_time` (optional): Start time (ISO 8601)
- `end_time` (optional): End time (ISO 8601)
- `status` (optional): Filter by status (`active`, `canceled`)
- `count` (optional): Number of results (default 100)
- `page_token` (optional): Pagination token

**Example LLM prompt**: "List all my scheduled events for next week"

---

### calendly_more_get_scheduled_event

**What it does**: Gets detailed information about a specific scheduled event including timing, invitees, and event type.

**When to use**: Get full details of a meeting before attending or before making changes.

**Arguments**:
- `uuid` (required): Scheduled event UUID

**Example LLM prompt**: "What are the details of event abc-123?"

---

### calendly_more_cancel_scheduled_event

**What it does**: Cancels a scheduled event (invitation). Optionally sends notification to invitee.

**When to use**: Cancel a meeting that no longer needs to happen.

**Arguments**:
- `uuid` (required): Scheduled event UUID
- `reason` (optional): Cancellation reason
- `notify` (optional): Send notification to invitee (default true)

**Example LLM prompt**: "Cancel the meeting with abc-123 and notify the invitees"

---

### calendly_more_list_invitees

**What it does**: Lists all invitees for a specific scheduled event. Shows who has been invited and their response status.

**When to use**: See who is attending a meeting, check RSVP status.

**Arguments**:
- `uuid` (required): Scheduled event UUID
- `count` (optional): Number of results (default 100)
- `page_token` (optional): Pagination token

**Example LLM prompt**: "Who is invited to the 2pm team meeting?"

---

### calendly_more_get_invitee

**What it does**: Gets detailed information about a specific invitee including their answers to event questions and cancellation status.

**When to use**: Understand invitee details, check if they have any special requirements noted.

**Arguments**:
- `uuid` (required): Invitee UUID

**Example LLM prompt**: "What are the details of invitee xyz-789?"

---

### calendly_more_get_user_availability

**What it does**: Gets available time slots for a user based on their calendar availability and event type settings.

**When to use**: Find open slots when scheduling a meeting with a specific user.

**Arguments**:
- `user_id` (required): User ID or URI
- `event_type` (optional): Event type URI
- `start_time` (optional): Start of availability window (ISO 8601)
- `end_time` (optional): End of availability window (ISO 8601)

**Example LLM prompt**: "When is John available for a 30-minute call next week?"

---

### calendly_more_list_organization_event_types

**What it does**: Lists all event types for an organization. Includes event types created by all organization members.

**When to use**: See all scheduling options available across an organization.

**Arguments**:
- `organization_id` (required): Organization ID or URI
- `active` (optional): Filter by active status
- `count` (optional): Number of results (default 100)
- `page_token` (optional): Pagination token

**Example LLM prompt**: "What event types are available in the Acme organization?"

---

## Calendly API Notes

- **UUIDs**: All resources use UUIDs for identification (format: `XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX`)
- **URIs**: Calendly uses full URIs for referencing users and organizations
- **Pagination**: Use page_token for retrieving subsequent pages of results
- **Cancellation**: Canceled events remain in the system with `status: canceled`
- **ISO 8601**: All timestamps are in ISO 8601 format (e.g., `2024-01-15T09:00:00Z`)
- **Organization scope**: List organization event types requires valid organization membership