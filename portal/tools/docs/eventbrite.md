# Eventbrite Tools

Provider: `eventbrite` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Eventbrite API. They allow AI agents to manage events, orders, attendees, categories, and venues. Eventbrite is a comprehensive event management platform supporting both free and paid events with integrated ticketing.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Eventbrite
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `event:read`, `event:write`, `order:read`, `attendee:read`, `venue:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `eventbrite_list_events` | List organization events | GET | /v3/organizations/{organization_id}/events/ |
| `eventbrite_get_event` | Get event details | GET | /v3/events/{event_id}/ |
| `eventbrite_create_event` | Create a new event | POST | /v3/organizations/{organization_id}/events/ |
| `eventbrite_list_orders` | List event orders | GET | /v3/events/{event_id}/orders/ |
| `eventbrite_get_order` | Get order details | GET | /v3/orders/{order_id}/ |
| `eventbrite_list_attendees` | List event attendees | GET | /v3/events/{event_id}/attendees/ |
| `eventbrite_get_attendee` | Get attendee details | GET | /v3/attendees/{attendee_id}/ |
| `eventbrite_list_categories` | List event categories | GET | /v3/categories/ |
| `eventbrite_get_category` | Get category details | GET | /v3/categories/{category_id}/ |
| `eventbrite_list_venues` | List organization venues | GET | /v3/organizations/{organization_id}/venues/ |

---

## Tool Details

### eventbrite_list_events

**What it does**: Retrieves a list of all events for the authenticated organization.

**When to use**: Get an overview of all events, filter by status, or find events by category.

**Arguments**:
- `organization_id` (required): The Eventbrite organization ID
- `status` (optional): Filter by status (live, started, ended, canceled, draft)
- `expand` (optional): Comma-separated list of fields to expand (venue, ticket_classes)

**Example LLM prompt**: "List all live events for my Eventbrite organization"

---

### eventbrite_get_event

**What it does**: Retrieves detailed information about a specific event by ID.

**When to use**: Get full event details including venue, ticket classes, and organizers.

**Arguments**:
- `event_id` (required): The unique identifier of the event
- `expand` (optional): Comma-separated list of fields to expand (venue, ticket_classes, organizers)

**Example LLM prompt**: "Get details for event ID 123456789"

---

### eventbrite_create_event

**What it does**: Creates a new event under the specified organization with timing and venue details.

**When to use**: Publish a new event with ticketing and venue information.

**Arguments**:
- `organization_id` (required): The Eventbrite organization ID
- `name` (required): The name/title of the event
- `description_html` (optional): HTML description of the event
- `start_utc` (required): Start datetime in UTC ISO 8601 format
- `end_utc` (required): End datetime in UTC ISO 8601 format
- `currency` (required): Currency code (e.g., USD, EUR)
- `online_event` (optional): Whether the event is online
- `venue_id` (optional): ID of an existing venue to use

**Example LLM prompt**: "Create a new tech meetup event on June 1st from 6pm to 9pm UTC"

---

### eventbrite_list_orders

**What it does**: Retrieves all orders for a specific event including purchaser and payment details.

**When to use**: Track ticket sales, view order history, and monitor refunds.

**Arguments**:
- `event_id` (required): The unique identifier of the event
- `status` (optional): Filter by order status (placed, completed, refunded, canceled)

**Example LLM prompt**: "List all completed orders for the tech meetup event"

---

### eventbrite_get_order

**What it does**: Retrieves detailed information about a specific order including attendee list.

**When to use**: Look up order details, buyer contact info, and ticket assignments.

**Arguments**:
- `order_id` (required): The unique identifier of the order

**Example LLM prompt**: "Get details for order 123456789"

---

### eventbrite_list_attendees

**What it does**: Retrieves all attendees for a specific event with their ticket and status information.

**When to use**: Get attendee list, check-in status, or ticket type distribution.

**Arguments**:
- `event_id` (required): The unique identifier of the event
- `status` (optional): Filter by status (attending, not_attending)

**Example LLM prompt**: "List all attendees for the tech meetup"

---

### eventbrite_get_attendee

**What it does**: Retrieves detailed information about a specific attendee including their ticket type.

**When to use**: Get attendee profile, ticket details, or contact information.

**Arguments**:
- `attendee_id` (required): The unique identifier of the attendee

**Example LLM prompt**: "Get details for attendee ID 987654321"

---

### eventbrite_list_categories

**What it does**: Retrieves all event categories available in Eventbrite for classification.

**When to use**: Find appropriate categories for events or understand category taxonomy.

**Arguments**: None

**Example LLM prompt**: "What event categories are available on Eventbrite?"

---

### eventbrite_get_category

**What it does**: Retrieves information about a specific event category by ID.

**When to use**: Get category details, subcategories, or related categories.

**Arguments**:
- `category_id` (required): The unique identifier of the category

**Example LLM prompt**: "Get details for the Music category"

---

### eventbrite_list_venues

**What it does**: Retrieves all venues associated with the authenticated organization.

**When to use**: Find existing venues for event creation or manage venue information.

**Arguments**:
- `organization_id` (required): The Eventbrite organization ID

**Example LLM prompt**: "List all venues in my Eventbrite organization"

---

## Eventbrite API Notes

- **Organization ID**: Required for organization-level operations; found in Eventbrite dashboard
- **Expands**: Use `expand` parameter to include related objects (venue, ticket_classes, organizers)
- **Pagination**: Eventbrite uses cursor-based pagination for large result sets
- **Webhooks**: Eventbrite supports webhooks for order and attendance notifications (future expansion)
