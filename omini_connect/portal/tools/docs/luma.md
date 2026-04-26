# Luma Tools

Provider: `luma` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Luma API. They allow AI agents to manage events, tickets, orders, attendees, and check-ins. Luma is an event platform focused on creating memorable experiences with modern ticketing and event management features.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Luma
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `events:read`, `events:write`, `tickets:read`, `orders:read`, `attendees:read`, `attendees:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `luma_list_events` | List all events | GET | /v1/events |
| `luma_get_event` | Get event details | GET | /v1/events/{event_id_or_slug} |
| `luma_create_event` | Create a new event | POST | /v1/events |
| `luma_update_event` | Update an event | PUT | /v1/events/{event_id} |
| `luma_list_tickets` | List ticket types | GET | /v1/events/{event_id}/tickets |
| `luma_get_ticket` | Get ticket details | GET | /v1/tickets/{ticket_id} |
| `luma_list_orders` | List orders | GET | /v1/events/{event_id}/orders |
| `luma_get_order` | Get order details | GET | /v1/orders/{order_id} |
| `luma_list_attendees` | List attendees | GET | /v1/events/{event_id}/attendees |
| `luma_check_in_attendee` | Check in attendee | POST | /v1/attendees/{attendee_id}/check_in |

---

## Tool Details

### luma_list_events

**What it does**: Retrieves a list of all events organized by the authenticated account.

**When to use**: Find events, filter by status, or get an overview of all event activity.

**Arguments**:
- `limit` (optional): Maximum number of events to return (default: 50)
- `offset` (optional): Number of events to skip for pagination
- `status` (optional): Filter by event status (draft, published, canceled, completed)

**Example LLM prompt**: "List all published events in my Luma account"

---

### luma_get_event

**What it does**: Retrieves detailed information about a specific event by its ID or slug.

**When to use**: Get full event details including timing, location, capacity, and description.

**Arguments**:
- `event_id_or_slug` (required): The unique identifier or URL slug of the event

**Example LLM prompt**: "Get details for the tech-conference-2024 event"

---

### luma_create_event

**What it does**: Creates a new event with specified details including timing, location, and description.

**When to use**: Set up a new event with tickets and registration.

**Arguments**:
- `name` (required): The name/title of the event
- `start_date` (required): Event start date and time in ISO 8601 format
- `end_date` (required): Event end date and time in ISO 8601 format
- `timezone` (optional): Timezone for the event (e.g., America/New_York)
- `location` (optional): Physical or virtual location of the event
- `description` (optional): HTML description of the event
- `capacity` (optional): Maximum number of attendees allowed

**Example LLM prompt**: "Create a new event called Summer Summit starting July 15th at 9am PST"

---

### luma_update_event

**What it does**: Updates an existing event's details including timing, location, and description.

**When to use**: Modify event details after creation.

**Arguments**:
- `event_id` (required): The unique identifier of the event
- `name` (optional): Updated name/title of the event
- `start_date` (optional): Updated start date and time in ISO 8601 format
- `end_date` (optional): Updated end date and time in ISO 8601 format
- `timezone` (optional): Updated timezone for the event
- `location` (optional): Updated location of the event
- `description` (optional): Updated HTML description of the event
- `status` (optional): Updated status (draft, published, canceled)

**Example LLM prompt**: "Update the Summer Summit event to be at the downtown convention center"

---

### luma_list_tickets

**What it does**: Retrieves all ticket types available for a specific event.

**When to use**: See available ticket tiers, pricing, and availability.

**Arguments**:
- `event_id` (required): The unique identifier of the event

**Example LLM prompt**: "What ticket types are available for the tech conference?"

---

### luma_get_ticket

**What it does**: Retrieves detailed information about a specific ticket type by ID.

**When to use**: Check ticket details, pricing, and sales status.

**Arguments**:
- `ticket_id` (required): The unique identifier of the ticket type

**Example LLM prompt**: "Get details for the VIP early-bird ticket type"

---

### luma_list_orders

**What it does**: Retrieves all orders (ticket purchases) for a specific event.

**When to use**: Track sales, view order history, and monitor revenue.

**Arguments**:
- `event_id` (required): The unique identifier of the event
- `status` (optional): Filter by order status (pending, completed, refunded, canceled)

**Example LLM prompt**: "List all completed orders for the Summer Summit event"

---

### luma_get_order

**What it does**: Retrieves detailed information about a specific order including items and payment status.

**When to use**: Look up order details, buyer information, and ticket details.

**Arguments**:
- `order_id` (required): The unique identifier of the order

**Example LLM prompt**: "Get details for order ORD-12345"

---

### luma_list_attendees

**What it does**: Retrieves all attendees for a specific event with their registration details.

**When to use**: Get attendee list, check registration status, or prepare for check-in.

**Arguments**:
- `event_id` (required): The unique identifier of the event
- `status` (optional): Filter by attendee status (registered, checked_in, canceled, no_show)

**Example LLM prompt**: "List all registered attendees for the tech conference"

---

### luma_check_in_attendee

**What it does**: Marks an attendee as checked in to an event using their ticket or registration ID.

**When to use**: Mark attendance at the event venue.

**Arguments**:
- `attendee_id` (required): The unique identifier of the attendee or ticket
- `event_id` (optional): The event ID where checking in (required if not in path)

**Example LLM prompt**: "Check in attendee ATT-67890 to the Summer Summit"

---

## Luma API Notes

- **Event Slugs**: Luma supports both numeric IDs and URL slugs for event identification
- **Statuses**: Events go through lifecycle: draft -> published -> completed/canceled
- **Check-in**: Real-time check-in tracking available via the API
- **Webhooks**: Luma supports webhooks for order and check-in notifications (future expansion)
