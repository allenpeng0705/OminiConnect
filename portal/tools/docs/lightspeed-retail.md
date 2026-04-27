# Lightspeed Retail Tools

Provider: `lightspeed-retail` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Lightspeed Retail API. They allow AI agents to manage customers, sales, items, and inventory. **Requires Lightspeed Retail OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with Lightspeed Retail
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{domain_prefix}.retail.lightspeed.app`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lightspeed_retail_list_customers` | List customers | GET | /customer.json |
| `lightspeed_retail_get_customer` | Get customer details | GET | /customer/{customer_id}.json |
| `lightspeed_retail_list_sales` | List sales | GET | /sale.json |
| `lightspeed_retail_get_sale` | Get sale details | GET | /sale/{sale_id}.json |
| `lightspeed_retail_list_items` | List items | GET | /item.json |
| `lightspeed_retail_get_item` | Get item details | GET | /item/{item_id}.json |
| `lightspeed_retail_list_locations` | List locations | GET | /location.json |
| `lightspeed_retail_get_location` | Get location details | GET | /location/{location_id}.json |
| `lightspeed_retail_list_employees` | List employees | GET | /employee.json |
| `lightspeed_retail_get_employee` | Get employee details | GET | /employee/{employee_id}.json |

---

## Tool Details

### lightspeed_retail_list_customers

**What it does**: Lists all customers.

**When to use**: Find customers, view customer list.

**Arguments**:
- `limit` (optional): Max results (default: 20)
- `offset` (optional): Offset for pagination (default: 0)

**Example LLM prompt**: "List all customers in Lightspeed"

---

### lightspeed_retail_get_customer

**What it does**: Gets details for a specific customer.

**When to use**: Get customer information.

**Arguments**:
- `customer_id` (required): Customer ID

**Example LLM prompt**: "Get details for customer 12345"

---

### lightspeed_retail_list_sales

**What it does**: Lists all sales.

**When to use**: View sales history, track transactions.

**Arguments**:
- `limit` (optional): Max results (default: 20)
- `offset` (optional): Offset for pagination (default: 0)

**Example LLM prompt**: "List all sales in Lightspeed"

---

### lightspeed_retail_get_sale

**What it does**: Gets details for a specific sale.

**When to use**: Get sale information.

**Arguments**:
- `sale_id` (required): Sale ID

**Example LLM prompt**: "Get details for sale 67890"

---

### lightspeed_retail_list_items

**What it does**: Lists all items (products).

**When to use**: View inventory, find products.

**Arguments**:
- `limit` (optional): Max results (default: 20)
- `offset` (optional): Offset for pagination (default: 0)

**Example LLM prompt**: "List all items in Lightspeed"

---

### lightspeed_retail_get_item

**What it does**: Gets details for a specific item.

**When to use**: Get item information.

**Arguments**:
- `item_id` (required): Item ID

**Example LLM prompt**: "Get details for item i1"

---

### lightspeed_retail_list_locations

**What it does**: Lists all locations.

**When to use**: View stores, manage multi-location.

**Arguments**: None

**Example LLM prompt**: "List all locations in Lightspeed"

---

### lightspeed_retail_get_location

**What it does**: Gets details for a specific location.

**When to use**: Get location information.

**Arguments**:
- `location_id` (required): Location ID

**Example LLM prompt**: "Get details for location l1"

---

### lightspeed_retail_list_employees

**What it does**: Lists all employees.

**When to use**: View staff, manage employees.

**Arguments**: None

**Example LLM prompt**: "List all employees in Lightspeed"

---

### lightspeed_retail_get_employee

**What it does**: Gets details for a specific employee.

**When to use**: Get employee information.

**Arguments**:
- `employee_id` (required): Employee ID

**Example LLM prompt**: "Get details for employee e1"

---

## Lightspeed Retail API Notes

- **POS/Retail**: Point of sale and retail management
- **Customers**: Customer profiles and history
- **Sales**: Transaction records
- **Items**: Products and inventory
- **Locations**: Multi-store support
