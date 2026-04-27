# Cal.com (v2) Tools

Provider: `cal-com-v2` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Cal.com API v2. Cal.com is a scheduling platform that allows you to book meetings, manage event types, handle schedules, and configure webhooks. **Requires Cal.com API key.**

## Authentication

**Nango API_KEY**:
- User provides their Cal.com API key (Bearer token)
- Token passed via `Authorization: Bearer` header
- Base URL: `https://api.cal.com/v2`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cal_com_v2_list_event_types` | List event types | GET | /event-types |
| `cal_com_v2_get_event_type` | Get event type details | GET | /event-types/{id} |
| `cal_com_v2_list_bookings` | List bookings | GET | /bookings |
| `cal_com_v2_get_booking` | Get booking details | GET | /bookings/{id} |
| `cal_com_v2_create_booking` | Create a booking | POST | /bookings |
| `cal_com_v2_cancel_booking` | Cancel a booking | DELETE | /bookings/{id} |
| `cal_com_v2_get_current_user` | Get current user | GET | /me |
| `cal_com_v2_list_schedules` | List schedules | GET | /schedules |
| `cal_com_v2_get_schedule` | Get schedule details | GET | /schedules/{id} |
| `cal_com_v2_list_webhooks` | List webhooks | GET | /event-types/{id}/webhooks |

---

## Tool Details

### cal_com_v2_list_event_types

**What it does**: Lists all event types for the authenticated user.

**When to use**: View available meeting types before booking.

**Arguments**:
- `userId` (optional): Filter by user ID

**Example LLM prompt**: "List all my Cal.com event types"

---

### cal_com_v2_get_event_type

**What it does**: Gets details of a specific event type.

**When to use**: Get event type settings, duration, description.

**Arguments**:
- `id` (required): Event type ID

**Example LLM prompt**: "Get event type 123 details"

---

### cal_com_v2_list_bookings

**What it does**: Lists all bookings with cursor-based pagination.

**When to use**: Browse upcoming or past bookings.

**Arguments**:
- `status` (optional): `upcoming`, `past`, `cancelled`
- `limit` (optional): Max results (default 20)
- `cursor` (optional): Pagination cursor from previous response

**Example LLM prompt**: "List my upcoming Cal.com bookings"

---

### cal_com_v2_get_booking

**What it does**: Gets details of a specific booking.

**When to use**: View booking details, attendees, timing.

**Arguments**:
- `id` (required): Booking ID

**Example LLM prompt**: "Get booking 456 details"

---

### cal_com_v2_create_booking

**What it does**: Creates a new booking.

**When to use**: Schedule a meeting on behalf of the user.

**Arguments**:
- `eventTypeId` (required): Event type ID
- `start` (required): ISO 8601 start time
- `end` (optional): ISO 8601 end time
- `attendee` (required): Object with `name` and `email`

**Example LLM prompt**: "Create a booking for event type 123 next Monday at 2pm"

---

### cal_com_v2_cancel_booking

**What it does**: Cancels a specific booking.

**When to use**: Cancel a scheduled meeting.

**Arguments**:
- `id` (required): Booking ID
- `reason` (optional): Cancellation reason

**Example LLM prompt**: "Cancel booking 456"

---

### cal_com_v2_get_current_user

**What it does**: Gets the authenticated user profile.

**When to use**: Verify identity, get user settings.

**Arguments**: None

**Example LLM prompt**: "Get my Cal.com profile"

---

### cal_com_v2_list_schedules

**What it does**: Lists all schedules for the user.

**When to use**: View available calendars/schedules for booking.

**Arguments**:
- `userId` (optional): Filter by user ID

**Example LLM prompt**: "List my Cal.com schedules"

---

### cal_com_v2_get_schedule

**What it does**: Gets details of a specific schedule.

**When to use**: View schedule availability and settings.

**Arguments**:
- `id` (required): Schedule ID

**Example LLM prompt**: "Get schedule 789 details"

---

### cal_com_v2_list_webhooks

**What it does**: Lists all webhooks for an event type.

**When to use**: Check existing webhook integrations.

**Arguments**:
- `eventTypeId` (required): Event type ID

**Example LLM prompt**: "List webhooks for event type 123"

---

## Cal.com v2 API Notes

- **Cursor Pagination**: Uses `cursor` parameter for paginating through results
- **Bearer Auth**: API key passed via `Authorization: Bearer` header
- **Webhooks**: Subscribe to events like `booking.created`, `booking.cancelled`
- **Schedules**: Define availability windows for booking
