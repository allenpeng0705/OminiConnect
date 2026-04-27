# Zoho More Tools

Provider: `zoho_more` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

This tool registry covers additional Zoho services beyond core CRM: Zoho Books (invoicing), Zoho People (HR), and Zoho Inventory (stock management). AI agents can use these tools to manage invoices, employees, items, and purchase orders across your Zoho workspace.

## Authentication

**Nango (OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes vary by service: `zoho_books`, `zoho_people`, `zoho_inventory`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_list_invoices` | List invoices from Zoho Books | GET | /books/v3/invoices |
| `zoho_get_invoice` | Get invoice details | GET | /books/v3/invoices/{invoice_id} |
| `zoho_create_invoice` | Create a new invoice | POST | /books/v3/invoices |
| `zoho_list_contacts` | List contacts from Zoho People | GET | /people/v3/contacts |
| `zoho_get_contact` | Get contact details | GET | /people/v3/contacts/{contact_id} |
| `zoho_list_items` | List items from Zoho Inventory | GET | /inventory/v3/items |
| `zoho_get_item` | Get item details | GET | /inventory/v3/items/{item_id} |
| `zoho_list_purchase_orders` | List purchase orders | GET | /inventory/v3/purchaseorders |
| `zoho_get_purchase_order` | Get purchase order details | GET | /inventory/v3/purchaseorders/{purchase_order_id} |
| `zoho_list_employees` | List employees from Zoho People | GET | /people/v3/employees |

---

## Tool Details

### zoho_list_invoices

**What it does**: Returns a paginated list of invoices from Zoho Books, optionally filtered by status, customer, or date range.

**When to use**: Review outstanding invoices, track payments, or find invoices for a specific customer or period.

**Arguments**:
- `status` (optional): `draft`, `sent`, `paid`, `overdue`, `void`
- `customer_id` (optional): Filter by customer
- `date_after` (optional): Start date (YYYY-MM-DD)
- `date_before` (optional): End date (YYYY-MM-DD)
- `page` (optional): default 1
- `per_page` (optional): default 25

**Example LLM prompt**: "Show me all overdue invoices from the last 30 days"

---

### zoho_get_invoice

**What it does**: Returns detailed information about a specific invoice including line items, taxes, payment terms, and status history.

**When to use**: Review invoice details before sending a reminder or marking as paid.

**Arguments**:
- `invoice_id` (required): Invoice ID

**Example LLM prompt**: "What are the line items on invoice INV-2024-001?"

---

### zoho_create_invoice

**What it does**: Creates a new invoice in Zoho Books with customer, line items, and payment terms.

**When to use**: Generate invoices for services rendered, or automate invoice creation from a workflow.

**Arguments**:
- `customer_id` (required): Customer ID
- `invoice_number` (optional): Custom invoice number
- `date` (optional): Invoice date (YYYY-MM-DD)
- `due_date` (optional): Payment due date
- `line_items` (optional): Array of `{item_id, quantity, rate}`
- `notes` (optional): Additional notes

**Example LLM prompt**: "Create an invoice for customer ACME Corp with 3 hours of consulting at $150/hour"

---

### zoho_list_contacts

**What it does**: Returns a paginated list of contacts from Zoho People, filtered by department, designation, or search term.

**When to use**: Find employees by name, browse organizational structure, or locate someone in a specific department.

**Arguments**:
- `department` (optional): Filter by department
- `designation` (optional): Filter by job title
- `search` (optional): Search by name or email
- `page` (optional): default 1
- `per_page` (optional): default 25

**Example LLM prompt**: "List all employees in the Engineering department"

---

### zoho_get_contact

**What it does**: Returns detailed information about a specific contact including employment details, reporting structure, and contact information.

**When to use**: Get full context on an employee before scheduling a meeting or sending a message.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Show me the full profile for contact john.doe@company.com"

---

### zoho_list_items

**What it does**: Returns all items (products and services) from Zoho Inventory, filtered by name, category, or status.

**When to use**: Browse product catalog, check stock levels, or find items for an order.

**Arguments**:
- `name` (optional): Filter by item name
- `category` (optional): Filter by category
- `status` (optional): `active`, `inactive`
- `page` (optional): default 1
- `per_page` (optional): default 25

**Example LLM prompt**: "List all active products in the Electronics category"

---

### zoho_get_item

**What it does**: Returns detailed information about a specific item including pricing, stock levels, and custom fields.

**When to use**: Check current stock before confirming an order, or review item details before adding to a purchase order.

**Arguments**:
- `item_id` (required): Item ID

**Example LLM prompt**: "What is the current stock level for item SKU-12345?"

---

### zoho_list_purchase_orders

**What it does**: Returns a paginated list of purchase orders from Zoho Inventory, filtered by status, vendor, or date.

**When to use**: Track open orders, review pending deliveries, or find orders for a specific vendor.

**Arguments**:
- `status` (optional): `draft`, `confirmed`, `partially_received`, `received`, `cancelled`
- `vendor_id` (optional): Filter by vendor
- `page` (optional): default 1
- `per_page` (optional): default 25

**Example LLM prompt**: "Show me all purchase orders with status 'confirmed' for vendor Acme Supplies"

---

### zoho_get_purchase_order

**What it does**: Returns full details of a specific purchase order including line items, vendor info, and delivery status.

**When to use**: Review what's been ordered, check expected delivery dates, or track partial shipments.

**Arguments**:
- `purchase_order_id` (required): Purchase Order ID

**Example LLM prompt**: "Show me the full details of PO-2024-001"

---

### zoho_list_employees

**What it does**: Returns a paginated list of employees from Zoho People, filtered by department, status, or search term.

**When to use**: Directory lookup, org chart exploration, or finding active employees for scheduling.

**Arguments**:
- `department` (optional): Filter by department
- `status` (optional): `active`, `inactive`
- `search` (optional): Search by name or email
- `page` (optional): default 1
- `per_page` (optional): default 25

**Example LLM prompt**: "List all active employees in the Sales department"

---

## Zoho API Reference

These tools use various Zoho APIs. See official docs for full details:
- Zoho Books: https://www.zoho.com/books/api/v3/
- Zoho People: https://www.zoho.com/people/api/v3/
- Zoho Inventory: https://www.zoho.com/inventory/api/v3/
- Rate limits: Varies by plan
- Pagination: Use `page` and `per_page` parameters
- All dates: ISO 8601 format (YYYY-MM-DD)