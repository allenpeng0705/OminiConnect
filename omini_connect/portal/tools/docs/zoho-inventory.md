# Zoho Inventory Tools

Provider: `zoho-inventory` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zoho Inventory is an inventory management software. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoho Inventory
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_inventory_list_items` | List all inventory items | GET | /items |
| `zoho_inventory_get_item` | Get item details | GET | /items/{id} |
| `zoho_inventory_create_item` | Create a new item | POST | /items |
| `zoho_inventory_list_sales_orders` | List all sales orders | GET | /salesorders |
| `zoho_inventory_get_sales_order` | Get sales order details | GET | /salesorders/{id} |
| `zoho_inventory_create_sales_order` | Create a new sales order | POST | /salesorders |
| `zoho_inventory_list_contacts` | List all contacts | GET | /contacts |
| `zoho_inventory_get_contact` | Get contact details | GET | /contacts/{id} |
| `zoho_inventory_list_warehouses` | List all warehouses | GET | /warehouses |
| `zoho_inventory_list_packages` | List all packages | GET | /packages |

---

## Tool Details

### zoho_inventory_list_items

**What it does**: List all inventory items

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_inventory_get_item

**What it does**: Get item details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_inventory_create_item

**What it does**: Create a new item

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_inventory_list_sales_orders

**What it does**: List all sales orders

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_inventory_get_sales_order

**What it does**: Get sales order details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_inventory_create_sales_order

**What it does**: Create a new sales order

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_inventory_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_inventory_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_inventory_list_warehouses

**What it does**: List all warehouses

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_inventory_list_packages

**What it does**: List all packages

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://www.zohoapis.com/inventory/v2`
- Docs: https://nango.dev/docs/integrations/all/zoho-inventory
