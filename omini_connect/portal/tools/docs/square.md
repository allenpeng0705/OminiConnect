# Square Tools

Provider: `square` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Square commerce API. They enable AI agents to manage payments, catalog items, customers, and orders for point-of-sale and e-commerce platforms.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Square
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `payments.read`, `payments.write`, `catalog.read`, `catalog.write`, `customers.read`, `customers.write`, `orders.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `square_list_payments` | List payments | GET | /v2/payments |
| `square_get_payment` | Get details of a specific payment | GET | /v2/payments/{id} |
| `square_create_payment` | Create a new payment | POST | /v2/payments |
| `square_list_items` | List catalog items | GET | /v2/catalog/list |
| `square_get_item` | Get details of a specific catalog item | GET | /v2/catalog/object/{id} |
| `square_create_item` | Create a new catalog item | POST | /v2/catalog/object |
| `square_list_customers` | List customers | GET | /v2/customers |
| `square_get_customer` | Get details of a specific customer | GET | /v2/customers/{id} |
| `square_list_orders` | List orders | POST | /v2/orders/search |
| `square_get_order` | Get details of a specific order | GET | /v2/orders/{id} |

---

## Tool Details

### square_list_payments

**What it does**: List payments processed through Square. Returns payments with amounts, status, and payment method details.

**When to use**: Get an overview of all payment transactions for reconciliation or reporting.

**Arguments**:
- `location_id` (optional): Filter by location ID
- `customer_id` (optional): Filter by customer ID
- `status` (optional): Filter by status (COMPLETED, APPROVED, PENDING, CANCELED, FAILED)
- `begin_time` (optional): Filter by start time (ISO 8601)
- `end_time` (optional): Filter by end time (ISO 8601)
- `sort_order` (optional): Sort order: ASC or DESC (default DESC)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all completed payments this week"

---

### square_get_payment

**What it does**: Get details of a specific payment including amount, status, receipt URL, and funding details.

**When to use**: Review payment details for refunds or customer inquiries.

**Arguments**:
- `id` (required): Payment ID

**Example LLM prompt**: "Get details for payment Jq74mC3mD4"

---

### square_create_payment

**What it does**: Create a new payment for an order. Specify source ID (card nonce) and amount to charge.

**When to use**: Process a payment from a customer's card or stored payment method.

**Arguments**:
- `source_id` (required): Payment source ID (card nonce from Square Web Payments SDK)
- `idempotency_key` (required): Unique key to prevent duplicate charges
- `amount_money` (required): Amount object with amount (in cents) and currency
- `location_id` (optional): Location ID for the payment
- `customer_id` (optional): Customer ID to associate
- `note` (optional): Payment note

**Example LLM prompt**: "Charge $49.99 to card nonce cnon:card-nonce-ok for customer with idempotency key order_123"

---

### square_list_items

**What it does**: List catalog items (products) from the Square catalog. Returns item details, variations, and pricing.

**When to use**: View product catalog for inventory or sales purposes.

**Arguments**:
- `types` (optional): Filter by object type: ITEM, MODIFIER_LIST, CATEGORY (default ITEM)
- `cursor` (optional): Pagination cursor
- `limit` (optional): Max results to return (max 100, default 100)

**Example LLM prompt**: "List all items in the catalog"

---

### square_get_item

**What it does**: Get details of a specific catalog item including name, description, variations, and pricing.

**When to use**: Review item details before updating or including in orders.

**Arguments**:
- `id` (required): Catalog object ID

**Example LLM prompt**: "Get details for item WINE123456"

---

### square_create_item

**What it does**: Create a new item in the Square catalog with variations, pricing, and category associations.

**When to use**: Add new products or services to the Square catalog.

**Arguments**:
- `idempotency_key` (required): Unique key for idempotency
- `object` (required): Catalog object with type ITEM and item_data

**Example LLM prompt**: "Create a new item called Premium Coffee with variations for sizes"

---

### square_list_customers

**What it does**: List all customers in the Square customer directory. Returns names, emails, addresses, and preferences.

**When to use**: Get an overview of all customers for CRM or marketing purposes.

**Arguments**:
- `cursor` (optional): Pagination cursor
- `limit` (optional): Max results to return (max 100, default 100)
- `sort_field` (optional): Sort field: DEFAULT, CREATED_AT, UPDATED_AT (default DEFAULT)
- `sort_order` (optional): Sort order: ASC or DESC (default ASC)

**Example LLM prompt**: "List all customers sorted by creation date"

---

### square_get_customer

**What it does**: Get details of a specific customer including contact info, address, and purchase history.

**When to use**: Review customer details before processing orders or updates.

**Arguments**:
- `id` (required): Customer ID

**Example LLM prompt**: "Get details for customer DQPOH0923"

---

### square_list_orders

**What it does**: List orders from Square. Returns order details including line items, totals, and fulfillment status.

**When to use**: Track orders across all locations for fulfillment or reporting.

**Arguments**:
- `location_ids` (optional): Filter by location IDs
- `customer_id` (optional): Filter by customer ID
- `status` (optional): Filter by status (OPEN, COMPLETED, CANCELED, FAILED)
- `date_time_filter` (optional): Filter by created_at or updated_at
- `limit` (optional): Max results to return (max 200, default 100)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all open orders from today"

---

### square_get_order

**What it does**: Get details of a specific order including line items, modifiers, taxes, and fulfillment info.

**When to use**: Review order details for fulfillment or customer inquiries.

**Arguments**:
- `id` (required): Order ID

**Example LLM prompt**: "Get details for order order_456789"

---

## Square API Notes

- **Commerce Platform**: Square provides POS, e-commerce, and payment processing
- **Catalog Objects**: Items, variations, modifiers, and other catalog entities
- **Location-Based**: Payments and orders are associated with specific locations
- **Idempotency**: POST requests support idempotency keys for safe retries
- **Cursor Pagination**: Uses cursor-based pagination for large result sets
- **ISO 8601 Dates**: Timestamps use ISO 8601 format (e.g., 2026-04-26T10:00:00Z)
- **Amounts**: Amounts are in smallest currency unit (cents for USD)
