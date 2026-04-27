# Sap-business-one Tools

Provider: `sap-business-one` | Engine: `nango` | Auth: OAuth via Nango

## Overview

SAP Business One is an ERP system for small and medium businesses. These tools allow AI agents to manage items, business partners, orders, invoices, and audit logs.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with SAP Business One
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `items:read`, `items:write`, `businessPartners:read`, `businessPartners:write`, `orders:read`, `orders:write`, `invoices:read`, `audit:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sap-business-one_list_items` | List all items | GET | /v1/items |
| `sap-business-one_get_item` | Get item details | GET | /v1/items/{itemId} |
| `sap-business-one_create_item` | Create an item | POST | /v1/items |
| `sap-business-one_list_business_partners` | List business partners | GET | /v1/businessPartners |
| `sap-business-one_get_business_partner` | Get partner details | GET | /v1/businessPartners/{partnerId} |
| `sap-business-one_create_business_partner` | Create a partner | POST | /v1/businessPartners |
| `sap-business-one_list_orders` | List sales orders | GET | /v1/orders |
| `sap-business-one_create_order` | Create a sales order | POST | /v1/orders |
| `sap-business-one_list_invoices` | List invoices | GET | /v1/invoices |
| `sap-business-one_get_audit_log` | Get audit log | GET | /v1/auditLog |

---

## Tool Details

### sap-business-one_list_items

**What it does**: Returns a list of all items.

**When to use**: View product catalog.

**Arguments**:
- `limit` (optional): Number of items (default 50)

**Example LLM prompt**: "List all items"

---

### sap-business-one_get_item

**What it does**: Gets details of a specific item.

**When to use**: Get item information and pricing.

**Arguments**:
- `itemId` (required): The item ID

**Example LLM prompt**: "Get details for item itm_abc123"

---

### sap-business-one_create_item

**What it does**: Creates a new item.

**When to use**: Add new products to inventory.

**Arguments**:
- `itemCode` (required): Item code
- `itemName` (required): Item name
- `itemType` (optional): Item type (items, labor, travel)
- `price` (optional): Item price

**Example LLM prompt**: "Create an item called 'Widget' with code WDG001"

---

### sap-business-one_list_business_partners

**What it does**: Returns business partners.

**When to use**: View customers and vendors.

**Arguments**:
- `limit` (optional): Number of partners (default 50)
- `type` (optional): Filter by type (customer, vendor)

**Example LLM prompt**: "List all customers"

---

### sap-business-one_get_business_partner

**What it does**: Gets business partner details.

**When to use**: Get partner information.

**Arguments**:
- `partnerId` (required): The business partner ID

**Example LLM prompt**: "Get details for partner bp_xyz789"

---

### sap-business-one_create_business_partner

**What it does**: Creates a new business partner.

**When to use**: Add new customers or vendors.

**Arguments**:
- `cardCode` (required): Partner code
- `cardName` (required): Partner name
- `cardType` (optional): Type (customer, vendor)

**Example LLM prompt**: "Create a customer called 'Acme Corp' with code ACM001"

---

### sap-business-one_list_orders

**What it does**: Returns a list of sales orders.

**When to use**: View sales order history.

**Arguments**:
- `limit` (optional): Number of orders (default 50)

**Example LLM prompt**: "List all orders"

---

### sap-business-one_create_order

**What it does**: Creates a new sales order.

**When to use**: Place orders for customers.

**Arguments**:
- `cardCode` (required): Business partner code
- `docDate` (optional): Document date
- `lines` (required): Order lines

**Example LLM prompt**: "Create an order for customer ACM001"

---

### sap-business-one_list_invoices

**What it does**: Returns a list of invoices.

**When to use**: View invoice history.

**Arguments**:
- `limit` (optional): Number of invoices (default 50)

**Example LLM prompt**: "List all invoices"

---

### sap-business-one_get_audit_log

**What it does**: Returns audit log entries.

**When to use**: Track system changes.

**Arguments**:
- `limit` (optional): Number of entries (default 50)

**Example LLM prompt**: "Get recent audit log entries"

---

## SAP Business One Notes

- Items are products and services in inventory
- Business partners are customers and vendors
- Orders track sales transactions
- Audit log tracks system changes
