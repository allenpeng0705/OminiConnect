# Cal.com (v1) Tools

Provider: `cal-com-v1` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Cal.com API v1. Cal.com is a scheduling platform that allows you to book meetings, manage event types, and handle calendar integrations. **Requires Cal.com API key.**

## Authentication

**Nango API_KEY**:
- User provides their Cal.com API key
- Token passed via query parameter `apiKey`
- Base URL: `https://api.cal.com/v1`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cal_com_v1_list_event_types` | List event types | GET | /event-types |
| `cal_com_v1_get_event_type` | Get event type details | GET | /event-types/{id} |
| `cal_com_v1_list_bookings` | List bookings | GET | /bookings |
| `cal_com_v1_get_booking` | Get booking details | GET | /bookings/{id} |
| `cal_com_v1_create_booking` | Create a booking | POST | /bookings |
| `cal_com_v1_cancel_booking` | Cancel a booking | DELETE | /bookings/{id} |
| `cal_com_v1_get_current_user` | Get current user | GET | /me |
| `cal_com_v1_list_available_users` | List available users | GET | /users |
| `cal_com_v1_get_user` | Get user details | GET | /users/{username} |
| `cal_com_v1_list_my_event_types` | List my event types | GET | /event-types |

---

## Tool Details

### cal_com_v1_list_event_types

**What it does**: Lists all available event types.

**When to use**: Browse meeting types before booking.

**Arguments**:
- `user` (optional): Filter by username

**Example LLM prompt**: "List all Cal.com event types"

---

### cal_com_v1_get_event_type

**What it does**: Gets details of a specific event type.

**When to use**: View event type settings and scheduling link.

**Arguments**:
- `id` (required): Event type ID

**Example LLM prompt**: "Get event type 123 details"

---

### cal_com_v1_list_bookings

**What it does**: Lists all bookings with optional status filter.

**When to use**: View upcoming or past meetings.

**Arguments**:
- `status` (optional): `upcoming`, `past`, `cancelled`
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List my upcoming Cal.com bookings"

---

### cal_com_v1_get_booking

**What it does**: Gets details of a specific booking.

**When to use**: View booking information.

**Arguments**:
- `id` (required): Booking ID

**Example LLM prompt**: "Get booking 456 details"

---

### cal_com_v1_create_booking

**What it does**: Creates a new booking.

**When to use**: Schedule a meeting.

**Arguments**:
- `eventTypeId` (required): Event type ID
- `start` (required): ISO 8601 start time
- `end` (optional): ISO 8601 end time
- `name` (required): Attendee name
- `email` (required): Attendee email

**Example LLM prompt**: "Create a booking for event type 123 next Monday"

---

### cal_com_v1_cancel_booking

**What it does**: Cancels a booking.

**When to use**: Cancel a scheduled meeting.

**Arguments**:
- `id` (required): Booking ID

**Example LLM prompt**: "Cancel booking 456"

---

### cal_com_v1_get_current_user

**What it does**: Gets the authenticated user profile.

**When to use**: Verify identity, get user settings.

**Arguments**: None

**Example LLM prompt**: "Get my Cal.com profile"

---

### cal_com_v1_list_available_users

**What it does**: Lists users available for booking.

**When to use**: Find team members.

**Arguments**: None

**Example LLM prompt**: "List available Cal.com users"

---

### cal_com_v1_get_user

**What it does**: Gets user details by username.

**When to use**: View specific user profile.

**Arguments**:
- `username` (required): Username

**Example LLM prompt**: "Get user johndoe details"

---

### cal_com_v1_list_my_event_types

**What it does**: Lists event types for the authenticated user.

**When to use**: View personal event types.

**Arguments**: None

**Example LLM prompt**: "List my event types"

---

## Cal.com v1 API Notes

- **API Key**: Passed via `apiKey` query parameter
- **Rate Limits**: Uses `x-ratelimit-reset` header for backoff
- **Event Types**: Define meeting types available for booking
