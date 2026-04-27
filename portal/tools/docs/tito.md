# Tito Tools

Provider: `tito` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Tito API. They allow AI agents to manage events, orders, attendees, check-ins, and promotional codes. Tito is a simplicity-focused event ticketing platform known for its clean interface and developer-friendly API.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Tito
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `events:read`, `events:write`, `orders:read`, `attendees:read`, `attendees:write`, `discounts:read`, `discounts:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tito_list_events` | List all events | GET | /v2/events |
| `tito_get_event` | Get event details | GET | /v2/{account}/{event_slug} |
| `tito_create_event` | Create a new event | POST | /v2/events |
| `tito_list_orders` | List event orders | GET | /v2/{account}/{event_slug}/orders |
| `tito_get_order` | Get order details | GET | /v2/{account}/{event_slug}/orders/{order_id} |
| `tito_list_attendees` | List attendees | GET | /v2/{account}/{event_slug}/attendees |
| `tito_get_attendee` | Get attendee details | GET | /v2/{account}/{event_slug}/attendees/{attendee_id} |
| `tito_check_in_attendee` | Check in attendee | POST | /v2/{account}/{event_slug}/attendees/{attendee_id}/check_ins |
| `tito_list_promo_codes` | List discount codes | GET | /v2/{account}/{event_slug}/discounts |
| `tito_create_promo_code` | Create discount code | POST | /v2/{account}/{event_slug}/discounts |

---

## Tool Details

### tito_list_events

**What it does**: Retrieves all events (ticketed experiences) for the authenticated account.

**When to use**: Get overview of all events, filter by state (live, finished, draft).

**Arguments**:
- `account` (required): The Tito account slug (e.g., my-company)
- `state` (optional): Filter by state (live, finished, draft)

**Example LLM prompt**: "List all live events in my Tito account"

---

### tito_get_event

**What it does**: Retrieves detailed information about a specific event including dates and settings.

**When to use**: Get event details, check event status, or view event configuration.

**Arguments**:
- `account` (required): The Tito account slug
- `event_slug` (required): The URL slug of the event

**Example LLM prompt**: "Get details for the summer-workshop-2024 event"

---

### tito_create_event

**What it does**: Creates a new ticketed event with title, date, and ticket release settings.

**When to use**: Set up a new event with ticketing and pricing.

**Arguments**:
- `account` (required): The Tito account slug
- `title` (required): The title of the event
- `slug` (optional): URL slug for the event (auto-generated if not provided)
- `date` (optional): Start date and time in ISO 8601 format
- `end_date` (optional): End date and time in ISO 8601 format
- `timezone` (optional): Timezone (e.g., America/New_York)
- `currency` (optional): Currency code (e.g., USD, GBP)

**Example LLM prompt**: "Create a new event titled Annual Conference in my tito account"

---

### tito_list_orders

**What it does**: Retrieves all orders for a specific event with payment and fulfillment status.

**When to use**: Track ticket sales, view payment status, and monitor refunds.

**Arguments**:
- `account` (required): The Tito account slug
- `event_slug` (required): The URL slug of the event
- `state` (optional): Filter by state (complete, incomplete, refunded, canceled)

**Example LLM prompt**: "List all complete orders for the Annual Conference event"

---

### tito_get_order

**What it does**: Retrieves complete details of a specific order including all tickets and buyer info.

**When to use**: Look up order details, buyer information, and ticket holders.

**Arguments**:
- `account` (required): The Tito account slug
- `event_slug` (required): The URL slug of the event
- `order_id` (required): The unique identifier of the order

**Example LLM prompt**: "Get details for order abc123 in the Annual Conference event"

---

### tito_list_attendees

**What it does**: Retrieves all attendees (ticket holders) for a specific event.

**When to use**: Get attendee list, check attendance status, or prepare for check-in.

**Arguments**:
- `account` (required): The Tito account slug
- `event_slug` (required): The URL slug of the event
- `checked_in` (optional): Filter by check-in status

**Example LLM prompt**: "List all attendees for the Annual Conference"

---

### tito_get_attendee

**What it does**: Retrieves detailed information about a specific attendee including their tickets.

**When to use**: Get attendee profile, ticket details, or check-in history.

**Arguments**:
- `account` (required): The Tito account slug
- `event_slug` (required): The URL slug of the event
- `attendee_id` (required): The unique identifier of the attendee

**Example LLM prompt**: "Get details for attendee xyz789 in the Annual Conference"

---

### tito_check_in_attendee

**What it does**: Checks in an attendee to an event, marking their ticket as used.

**When to use**: Record attendance at the event venue.

**Arguments**:
- `account` (required): The Tito account slug
- `event_slug` (required): The URL slug of the event
- `attendee_id` (required): The unique identifier of the attendee

**Example LLM prompt**: "Check in attendee xyz789 to the Annual Conference"

---

### tito_list_promo_codes

**What it does**: Retrieves all discount/promotional codes for a specific event.

**When to use**: View active discounts, track promo code usage, or manage promotional campaigns.

**Arguments**:
- `account` (required): The Tito account slug
- `event_slug` (required): The URL slug of the event

**Example LLM prompt**: "List all discount codes for the Annual Conference"

---

### tito_create_promo_code

**What it does**: Creates a new promotional discount code for an event with percentage or fixed amount.

**When to use**: Create early-bird discounts, affiliate codes, or special offers.

**Arguments**:
- `account` (required): The Tito account slug
- `event_slug` (required): The URL slug of the event
- `code` (required): The promotional code (e.g., EARLYBIRD)
- `discount_type` (required): Type of discount (percentage, fixed_amount)
- `discount_value` (required): Value of the discount (e.g., 20 for 20% or 20.00 for fixed amount)
- `max_uses` (optional): Maximum number of times the code can be used
- `tickets` (optional): Array of ticket IDs this code applies to (omit for all tickets)

**Example LLM prompt**: "Create a 20% discount code EARLYBIRD for the Annual Conference with max 100 uses"

---

## Tito API Notes

- **Account Slug**: Your Tito account URL prefix (e.g., `acme` in `acme.tito.io`)
- **Event Slug**: Each event has a URL-friendly slug for easy identification
- **Check-ins**: Real-time check-in tracking with timestamp history
- **Discounts**: Support for percentage and fixed-amount discounts with usage limits
- **Webhooks**: Tito supports webhooks for order and check-in events (future expansion)
