# Booking.com Tools

Provider: `booking-com` | Engine: `nango` | Auth: BASIC via Nango (Machine Account)

## Overview

These tools wrap the Booking.com Connectivity API. They allow AI agents to manage reservations, guests, properties, availability, and pricing. Booking.com is a travel accommodation platform for hotels and property managers.

## Authentication

**Nango BASIC Auth**:
- User provides Machine Account username and password
- Token stored in Nango, accessed via `connection_ref`
- Requires environment type configuration (secure-supply or supply)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `booking_list_reservations` | List reservations | GET | /reservation |
| `booking_get_reservation` | Get reservation details | GET | /reservation/{reservation_id} |
| `booking_list_guests` | List guests | GET | /guest |
| `booking_get_guest` | Get guest details | GET | /guest/{guest_id} |
| `booking_list_properties` | List properties | GET | /hotel |
| `booking_get_property` | Get property details | GET | /hotel/{hotel_id} |
| `booking_list_availability` | Get availability | GET | /availability |
| `booking_update_pricing` | Update pricing | POST | /price |
| `booking_list_reviews` | List reviews | GET | /review |
| `booking_get_statistics` | Get property statistics | GET | /statistics |

---

## Tool Details

### booking_list_reservations

**What it does**: Lists reservations for the property.

**When to use**: View upcoming and past bookings, manage reservations.

**Arguments**:
- `hotel_id` (optional): Property ID
- `date_from` (optional): Start date
- `date_to` (optional): End date
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all reservations for next week"

---

### booking_get_reservation

**What it does**: Gets details for a specific reservation.

**When to use**: View guest details, check booking information.

**Arguments**:
- `reservation_id` (required): Reservation ID

**Example LLM prompt**: "Get details for reservation R-12345"

---

### booking_list_guests

**What it does**: Lists guests who have stayed or have upcoming reservations.

**When to use**: View guest database, manage relationships.

**Arguments**:
- `hotel_id` (optional): Property ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all guests"

---

### booking_get_guest

**What it does**: Gets details for a specific guest.

**When to use**: View guest profile, check preferences.

**Arguments**:
- `guest_id` (required): Guest ID

**Example LLM prompt**: "Get details for guest G-456"

---

### booking_list_properties

**What it does**: Lists all properties in the account.

**When to use**: View all managed properties.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all my properties"

---

### booking_get_property

**What it does**: Gets details for a specific property.

**When to use**: Check property settings, configuration.

**Arguments**:
- `hotel_id` (required): Property ID

**Example LLM prompt**: "Get details for property H-100"

---

### booking_list_availability

**What it does**: Gets room availability for dates.

**When to use**: Check available rooms, plan bookings.

**Arguments**:
- `hotel_id` (required): Property ID
- `date_from` (required): Start date (YYYY-MM-DD)
- `date_to` (required): End date (YYYY-MM-DD)

**Example LLM prompt**: "Check availability for next weekend"

---

### booking_update_pricing

**What it does**: Updates room pricing for dates.

**When to use**: Adjust rates, manage pricing strategy.

**Arguments**:
- `hotel_id` (required): Property ID
- `room_id` (required): Room ID
- `date_from` (required): Start date (YYYY-MM-DD)
- `date_to` (required): End date (YYYY-MM-DD)
- `price` (required): New price

**Example LLM prompt**: "Update room R-200 price to $150 for next month"

---

### booking_list_reviews

**What it does**: Lists guest reviews for a property.

**When to use**: Monitor guest feedback, respond to reviews.

**Arguments**:
- `hotel_id` (optional): Property ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List recent reviews for property H-100"

---

### booking_get_statistics

**What it does**: Gets performance statistics for a property.

**When to use**: Analyze performance, track metrics.

**Arguments**:
- `hotel_id` (required): Property ID
- `date_from` (optional): Start date
- `date_to` (optional): End date

**Example LLM prompt**: "Get statistics for property H-100 this month"

---

## Booking.com API Notes

- **Hotel ID**: Unique identifier for each property
- **Machine Accounts**: Special accounts for property management software
- **Environment Types**: secure-supply (production) or supply (test)
- **Reservations**: Include guest info, dates, room assignments
- **Reviews**: Guest feedback and ratings
