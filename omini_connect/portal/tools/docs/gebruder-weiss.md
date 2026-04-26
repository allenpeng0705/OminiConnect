# Gebruder Weiss Tools

Provider: `gebruder-weiss` | Engine: `nango` | Auth: OAUTH2_CC via Nango

## Overview

These tools wrap the Gebruder Weiss logistics API. They allow AI agents to manage shipments, orders, tracking, pickups, and addresses. **Requires Gebruder Weiss OAuth2 Client Credentials.**

## Authentication

**Nango OAUTH2_CC (Client Credentials)**:
- Uses client_id and client_secret to obtain access token
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://my.api.gw-world.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gebruder_weiss_list_shipments` | List shipments | GET | /shipments |
| `gebruder_weiss_get_shipment` | Get shipment details | GET | /shipments/{shipmentId} |
| `gebruder_weiss_list_orders` | List orders | GET | /orders |
| `gebruder_weiss_get_order` | Get order details | GET | /orders/{orderId} |
| `gebruder_weiss_list_tracking` | List tracking events | GET | /tracking |
| `gebruder_weiss_track_shipment` | Track shipment | GET | /tracking/{trackingNumber} |
| `gebruder_weiss_list_pickups` | List pickups | GET | /pickups |
| `gebruder_weiss_create_pickup` | Create pickup request | POST | /pickups |
| `gebruder_weiss_list_addresses` | List addresses | GET | /addresses |
| `gebruder_weiss_estimate_rate` | Get shipping estimate | POST | /rates |

---

## Tool Details

### gebruder_weiss_list_shipments

**What it does**: Lists all shipments in the system.

**When to use**: Browse shipment history, filter by status.

**Arguments**:
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all shipments from this month"

---

### gebruder_weiss_get_shipment

**What it does**: Gets detailed information about a specific shipment.

**When to use**: Get shipment contents, routing, and status.

**Arguments**:
- `shipmentId` (required): Shipment ID

**Example LLM prompt**: "Get details for shipment ABC123"

---

### gebruder_weiss_list_orders

**What it does**: Lists all orders in the system.

**When to use**: Browse orders, track order status.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending orders"

---

### gebruder_weiss_get_order

**What it does**: Gets detailed information about a specific order.

**When to use**: Get order details, linked shipments.

**Arguments**:
- `orderId` (required): Order ID

**Example LLM prompt**: "Get details for order 12345"

---

### gebruder_weiss_list_tracking

**What it does**: Lists tracking events for shipments.

**When to use**: Get tracking history for all shipments.

**Arguments**:
- `shipmentId` (optional): Filter by shipment ID

**Example LLM prompt**: "List all tracking events for today"

---

### gebruder_weiss_track_shipment

**What it does**: Tracks a shipment by tracking number.

**When to use**: Get real-time location and status updates.

**Arguments**:
- `trackingNumber` (required): Tracking number

**Example LLM prompt**: "Track shipment with number ABC123456"

---

### gebruder_weiss_list_pickups

**What it does**: Lists scheduled pickup requests.

**When to use**: View upcoming pickups.

**Arguments**: None

**Example LLM prompt**: "List scheduled pickups"

---

### gebruder_weiss_create_pickup

**What it does**: Creates a new pickup request.

**When to use**: Schedule a shipment pickup.

**Arguments**:
- `address` (required): Pickup address
- `date` (required): Pickup date (YYYY-MM-DD)
- `shipmentId` (optional): Associated shipment ID

**Example LLM prompt**: "Create a pickup for tomorrow at our warehouse"

---

### gebruder_weiss_list_addresses

**What it does**: Lists saved addresses in the address book.

**When to use**: Find saved shipping addresses.

**Arguments**: None

**Example LLM prompt**: "List all saved addresses"

---

### gebruder_weiss_estimate_rate

**What it does**: Gets shipping rate estimate for a route.

**When to use**: Get shipping cost estimates before booking.

**Arguments**:
- `origin` (required): Origin postal code
- `destination` (required): Destination postal code
- `weight` (optional): Shipment weight

**Example LLM prompt**: "Estimate shipping cost from 1010 to 1020 for 50kg"

---

## Gebruder Weiss API Notes

- **Logistics focus**: Freight, shipping, and transport services
- **Tracking numbers**: Unique identifiers for each shipment
- **Pickup scheduling**: Request driver pickup for shipments
- **Rate estimation**: Quotes before commitment
