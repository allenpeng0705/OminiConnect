# Cal.com (OAuth) Tools

Provider: `cal-com-oauth` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Cal.com API v2. Cal.com is a scheduling platform that allows you to book meetings, manage event types, and handle calendar integrations. **Requires Cal.com OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Cal.com
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.cal.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cal_com_list_event_types` | List event types | GET | /event-types |
| `cal_com_get_event_type` | Get event type details | GET | /event-types/{id} |
| `cal_com_list_bookings` | List bookings | GET | /bookings |
| `cal_com_get_booking` | Get booking details | GET | /bookings/{id} |
| `cal_com_create_booking` | Create a booking | POST | /bookings |
| `cal_com_cancel_booking` | Cancel a booking | DELETE | /bookings/{id} |
| `cal_com_list_users` | List users | GET | /users |
| `cal_com_get_current_user` | Get current user | GET | /me |
| `cal_com_list_webhooks` | List webhooks | GET | /event-types/{id}/webhooks |
| `cal_com_create_webhook` | Create a webhook | POST | /event-types/{id}/webhooks |

---

## Tool Details

### cal_com_list_event_types

**What it does**: Lists all event types for the authenticated user or team.

**When to use**: View available meeting types, find event IDs for booking.

**Arguments**:
- `user` (optional): Username to filter by
- `team` (optional): Team ID to filter by

**Example LLM prompt**: "List all my event types in Cal.com"

---

### cal_com_get_event_type

**What it does**: Gets details of a specific event type.

**When to use**: Get scheduling link, duration, description for a specific event type.

**Arguments**:
- `id` (required): Event type ID

**Example LLM prompt**: "Get details for event type 123"

---

### cal_com_list_bookings

**What it does**: Lists all bookings filtered by status.

**When to use**: See upcoming meetings, find past bookings, check for cancellations.

**Arguments**:
- `status` (optional): `upcoming`, `past`, or `cancelled`
- `user` (optional): Filter by user
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List my upcoming Cal.com bookings"

---

### cal_com_get_booking

**What it does**: Gets details of a specific booking including attendees and timing.

**When to use**: View booking details before cancelling or rescheduling.

**Arguments**:
- `id` (required): Booking ID

**Example LLM prompt**: "Get details for booking 456"

---

### cal_com_create_booking

**What it does**: Creates a new booking for an event type.

**When to use**: Schedule a meeting on behalf of the user.

**Arguments**:
- `eventTypeId` (required): Event type ID
- `start` (required): ISO 8601 start time
- `end` (optional): ISO 8601 end time
- `attendees` (optional): Array of attendee objects with email and name

**Example LLM prompt**: "Create a booking for event type 123 next Monday at 10am"

---

### cal_com_cancel_booking

**What it does**: Cancels a specific booking.

**When to use**: Cancel a meeting or scheduled call.

**Arguments**:
- `id` (required): Booking ID
- `reason` (optional): Cancellation reason

**Example LLM prompt**: "Cancel booking 456"

---

### cal_com_list_users

**What it does**: Lists all users in the workspace.

**When to use**: Find team members for scheduling.

**Arguments**:
- `limit` (optional): Max results

**Example LLM prompt**: "List all users in my Cal.com workspace"

---

### cal_com_get_current_user

**What it does**: Gets the currently authenticated user profile.

**When to use**: Verify user identity, get user settings.

**Arguments**: None

**Example LLM prompt**: "Get my Cal.com profile"

---

### cal_com_list_webhooks

**What it does**: Lists all webhooks configured for an event type.

**When to use**: Check existing integrations, find webhook IDs.

**Arguments**:
- `eventTypeId` (required): Event type ID

**Example LLM prompt**: "List webhooks for event type 123"

---

### cal_com_create_webhook

**What it does**: Creates a new webhook for an event type.

**When to use**: Set up event notifications for external systems.

**Arguments**:
- `eventTypeId` (required): Event type ID
- `webhookUrl` (required): URL to send events to
- `events` (optional): Array of event types to subscribe to

**Example LLM prompt**: "Create a webhook for event type 123 pointing to https://example.com/webhook"

---

## Cal.com API Notes

- **Event Types**: Define the type of meetings you offer (e.g., "15 min call", "Demo")
- **Bookings**: Each booking has a unique ID and contains attendee information
- **Webhooks**: Subscribe to booking events like `booking.created`, `booking.cancelled`
- **OAuth2**: Uses the OAuth2 authorization code flow via Nango
