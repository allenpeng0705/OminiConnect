# Apaleo Tools

Provider: `apaleo` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Apaleo API. They allow AI agents to manage reservations, guests, properties, and invoices. Apaleo is a cloud-based property management system for hotels and vacation rentals.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Apaleo
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `reservations:read`, `reservations:write`, `guests:read`, `guests:write`, `properties:read`, `invoices:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `apaleo_list_reservations` | List all reservations | GET | /reservation-api/reservations |
| `apaleo_get_reservation` | Get reservation details | GET | /reservation-api/reservations/{id} |
| `apaleo_create_reservation` | Create a new reservation | POST | /reservation-api/reservations |
| `apaleo_list_guests` | List all guests | GET | /guest-directory-api/guests |
| `apaleo_get_guest` | Get guest details | GET | /guest-directory-api/guests/{id} |
| `apaleo_create_guest` | Create a new guest | POST | /guest-directory-api/guests |
| `apaleo_list_properties` | List all properties | GET | /property-discovery-api/properties |
| `apaleo_get_property` | Get property details | GET | /property-discovery-api/properties/{id} |
| `apaleo_list_invoices` | List all invoices | GET | /invoice-api/invoices |
| `apaleo_get_invoice` | Get invoice details | GET | /invoice-api/invoices/{id} |

---

## Tool Details

### apaleo_list_reservations

**What it does**: Lists all reservations in Apaleo with optional filtering by property, status, or arrival date.

**When to use**: View reservations for check-in, find reservations by date or property.

**Arguments**:
- `property_id` (optional): Filter by property ID
- `status` (optional): Filter by status (confirmed, cancelled, checked-in, checked-out)
- `arrival_date` (optional): Filter by arrival date (YYYY-MM-DD)
- `departure_date` (optional): Filter by departure date (YYYY-MM-DD)
- `limit` (optional): Max results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all confirmed reservations for tomorrow"

---

### apaleo_get_reservation

**What it does**: Gets detailed information about a specific reservation including guests, room assignment, and services.

**When to use**: Get reservation details before check-in or modification.

**Arguments**:
- `id` (required): Reservation ID

**Example LLM prompt**: "Get details for reservation 12345"

---

### apaleo_create_reservation

**What it does**: Creates a new reservation with property, room type, dates, and guest information.

**When to use**: Book a new stay for a guest.

**Arguments**:
- `property_id` (required): Property ID
- `room_type_id` (required): Room type ID
- `arrival_date` (required): Arrival date (YYYY-MM-DD)
- `departure_date` (required): Departure date (YYYY-MM-DD)
- `guest_email` (optional): Guest email address
- `guest_first_name` (optional): Guest first name
- `guest_last_name` (optional): Guest last name
- `adults` (optional): Number of adults (default 1)

**Example LLM prompt**: "Create a reservation at Beach Resort for June 15-18 with 2 adults"

---

### apaleo_list_guests

**What it does**: Lists all guests in Apaleo with optional filtering by property or search term.

**When to use**: Find guest profiles, search returning guests.

**Arguments**:
- `property_id` (optional): Filter by property ID
- `search` (optional): Search by name or email
- `limit` (optional): Max results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "Search for guest named John Smith"

---

### apaleo_get_guest

**What it does**: Gets detailed information about a specific guest including contact details, preferences, and stay history.

**When to use**: Get guest profile before arrival or check-in.

**Arguments**:
- `id` (required): Guest ID

**Example LLM prompt**: "Get details for guest 67890"

---

### apaleo_create_guest

**What it does**: Creates a new guest profile with name, email, phone, and preferences.

**When to use**: Add a new guest to the directory.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (required): Email address
- `phone` (optional): Phone number
- `language` (optional): Preferred language code

**Example LLM prompt**: "Create a guest profile for Jane Doe with email jane@email.com"

---

### apaleo_list_properties

**What it does**: Lists all properties (hotels) in Apaleo with their IDs, names, and addresses.

**When to use**: View available properties, select property for reservation.

**Arguments**:
- `limit` (optional): Max results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all properties in the portfolio"

---

### apaleo_get_property

**What it does**: Gets detailed information about a specific property including room types, amenities, and contact info.

**When to use**: Get property details, view available room types.

**Arguments**:
- `id` (required): Property ID

**Example LLM prompt**: "Get details for property beach-resort"

---

### apaleo_list_invoices

**What it does**: Lists all invoices in Apaleo with optional filtering by property, status, or date range.

**When to use**: View invoice history, find invoices by status or property.

**Arguments**:
- `property_id` (optional): Filter by property ID
- `status` (optional): Filter by status (open, paid, cancelled)
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `limit` (optional): Max results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all open invoices for this month"

---

### apaleo_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items, totals, and payment status.

**When to use**: Get invoice details for accounting or guest inquiries.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice 99999"

---

## Apaleo API Notes

- **Multi-property**: Apaleo supports multiple properties under one account.
- **Reservation lifecycle**: Reservations go through statuses (confirmed, cancelled, checked-in, checked-out).
- **Guest profiles**: Guests can have preferences and stay history across properties.
- **Invoice management**: Invoices are associated with reservations and can have multiple line items.
