# Onlogist Tools

Provider: `onlogist` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Onlogist logistics API. They allow AI agents to manage orders, bookings, carriers, and track shipments. **Requires Onlogist API key authentication.**

## Authentication

**API Key**:
- User provides Onlogist API key
- Key passed via Nango
- Base URL: Uses Onlogist API

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `onlogist_list_orders` | List orders | GET | /api/orders |
| `onlogist_get_order` | Get order details | GET | /api/orders/{id} |
| `onlogist_create_order` | Create order | POST | /api/orders |
| `onlogist_update_order` | Update order | PUT | /api/orders/{id} |
| `onlogist_list_bookings` | List bookings | GET | /api/bookings |
| `onlogist_get_booking` | Get booking details | GET | /api/bookings/{id} |
| `onlogist_create_booking` | Create booking | POST | /api/bookings |
| `onlogist_list_carriers` | List carriers | GET | /api/carriers |
| `onlogist_get_quote` | Get shipping quote | GET | /api/quotes |
| `onlogist_track_shipment` | Track shipment | GET | /api/shipments/{trackingNumber}/track |

---

## Tool Details

### onlogist_list_orders

**What it does**: Lists all orders in Onlogist.

**When to use**: Browse orders, track shipments.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all pending orders"

---

### onlogist_get_order

**What it does**: Gets detailed information for a specific order.

**When to use**: View order details, check status.

**Arguments**:
- `id` (required): Order ID

**Example LLM prompt**: "Get details for order ABC123"

---

### onlogist_create_order

**What it does**: Creates a new order in Onlogist.

**When to use**: Create new shipments, register logistics requests.

**Arguments**:
- `origin_address` (required): Origin address
- `destination_address` (required): Destination address
- `weight` (optional): Package weight

**Example LLM prompt**: "Create an order from New York to Los Angeles"

---

### onlogist_update_order

**What it does**: Updates an existing order.

**When to use**: Modify order details, change status.

**Arguments**:
- `id` (required): Order ID
- `status` (optional): Order status

**Example LLM prompt**: "Update order ABC123 status to shipped"

---

### onlogist_list_bookings

**What it does**: Lists all bookings in Onlogist.

**When to use**: Browse bookings, manage scheduled pickups.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all bookings for this week"

---

### onlogist_get_booking

**What it does**: Gets detailed information for a specific booking.

**When to use**: View booking details, pickup schedules.

**Arguments**:
- `id` (required): Booking ID

**Example LLM prompt**: "Get details for booking XYZ789"

---

### onlogist_create_booking

**What it does**: Creates a new booking in Onlogist.

**When to use**: Schedule pickups, reserve carrier capacity.

**Arguments**:
- `order_id` (required): Order ID
- `carrier_id` (required): Carrier ID
- `pickup_date` (optional): Pickup date

**Example LLM prompt**: "Create a booking for order ABC123 with carrier XYZ"

---

### onlogist_list_carriers

**What it does**: Lists all available carriers.

**When to use**: Find carriers, compare services.

**Arguments**: None

**Example LLM prompt**: "List all available carriers"

---

### onlogist_get_quote

**What it does**: Gets a shipping quote.

**When to use**: Get pricing, compare options.

**Arguments**:
- `origin` (required): Origin address
- `destination` (required): Destination address
- `weight` (required): Package weight

**Example LLM prompt**: "Get a quote for shipping 10kg from NY to LA"

---

### onlogist_track_shipment

**What it does**: Tracks a shipment by tracking number.

**When to use**: Monitor delivery, check status.

**Arguments**:
- `trackingNumber` (required): Tracking number

**Example LLM prompt**: "Track shipment 1Z999AA10123456784"

---

## Onlogist Notes

- **Orders**: Core entity for shipping requests
- **Bookings**: Scheduled pickups with carriers
- **Carriers**: Shipping providers available
- **Quotes**: Pricing for shipping routes
- **Tracking**: Real-time shipment tracking
