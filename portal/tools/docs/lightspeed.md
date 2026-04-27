# Lightspeed Tools

Provider: `lightspeed` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Lightspeed Retail POS API. They allow AI agents to manage items (products), customers, transactions, suppliers, and staff in a Lightspeed retail account. Lightspeed is a popular POS and e-commerce platform for retail businesses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Lightspeed
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read_inventory`, `write_inventory`, `read_customers`, `read_sales`, `read_suppliers`, `read_employees`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lightspeed_list_items` | List items/products | GET | /api/v2/Item.json |
| `lightspeed_get_item` | Get item details | GET | /api/v2/Item/{itemID}.json |
| `lightspeed_create_item` | Create a new item | POST | /api/v2/Item.json |
| `lightspeed_list_customers` | List customers | GET | /api/v2/Customer.json |
| `lightspeed_get_customer` | Get customer details | GET | /api/v2/Customer/{customerID}.json |
| `lightspeed_list_transactions` | List transactions | GET | /api/v2/Sale.json |
| `lightspeed_get_transaction` | Get transaction details | GET | /api/v2/Sale/{saleID}.json |
| `lightspeed_list_suppliers` | List suppliers | GET | /api/v2/Supplier.json |
| `lightspeed_get_supplier` | Get supplier details | GET | /api/v2/Supplier/{supplierID}.json |
| `lightspeed_list_staff` | List staff members | GET | /api/v2/Employee.json |

---

## Tool Details

### lightspeed_list_items

**What it does**: Lists all items (products) in the Lightspeed Retail POS. Supports filtering by category, supplier, and archived status.

**When to use**: Browse inventory, find items by category or supplier.

**Arguments**:
- `categoryID` (optional): Filter by category ID
- `supplierID` (optional): Filter by supplier ID
- `isArchived` (optional): Include archived items
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all items in category 5"

---

### lightspeed_get_item

**What it does**: Gets detailed information about a specific item including variants, pricing, and stock levels.

**When to use**: Check full item details, view inventory levels, get pricing.

**Arguments**:
- `itemID` (required): The item ID

**Example LLM prompt**: "Get details for item 12345"

---

### lightspeed_create_item

**What it does**: Creates a new item in Lightspeed Retail POS with name, SKU, price, and inventory settings.

**When to use**: Add new products to inventory.

**Arguments**:
- `name` (required): Item name
- `sku` (required): Stock keeping unit
- `description` (optional): Item description
- `price` (optional): Selling price
- `cost` (optional): Cost price
- `quantity` (optional): Initial inventory quantity
- `categoryID` (optional): Category ID

**Example LLM prompt**: "Create a new item called 'Blue T-Shirt' with SKU TSHIRT-BLUE and price $29.99"

---

### lightspeed_list_customers

**What it does**: Lists all customers in Lightspeed Retail POS. Supports filtering by customer group and archived status.

**When to use**: Browse customer base, find customers by group.

**Arguments**:
- `customerGroupID` (optional): Filter by customer group ID
- `isArchived` (optional): Include archived customers
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all customers in customer group 3"

---

### lightspeed_get_customer

**What it does**: Gets detailed information about a specific customer including contact info, addresses, and purchase history.

**When to use**: Get customer profile, view contact information.

**Arguments**:
- `customerID` (required): The customer ID

**Example LLM prompt**: "Get details for customer 67890"

---

### lightspeed_list_transactions

**What it does**: Lists all transactions (sales) in Lightspeed Retail POS. Supports filtering by date, register, and employee.

**When to use**: Track sales, find transactions by date or employee.

**Arguments**:
- `saleDate` (optional): Filter by date (YYYY-MM-DD)
- `registerID` (optional): Filter by register ID
- `employeeID` (optional): Filter by employee ID
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all transactions from today"

---

### lightspeed_get_transaction

**What it does**: Gets detailed information about a specific transaction including line items, payments, and totals.

**When to use**: Check transaction details, view line items, get payment information.

**Arguments**:
- `saleID` (required): The transaction/sale ID

**Example LLM prompt**: "Get details for transaction 11111"

---

### lightspeed_list_suppliers

**What it does**: Lists all suppliers (vendors) in Lightspeed Retail POS. Supports filtering by archived status.

**When to use**: Browse suppliers, manage vendor relationships.

**Arguments**:
- `isArchived` (optional): Include archived suppliers
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all active suppliers"

---

### lightspeed_get_supplier

**What it does**: Gets detailed information about a specific supplier including contact info and terms.

**When to use**: Get supplier details, view contact information.

**Arguments**:
- `supplierID` (required): The supplier ID

**Example LLM prompt**: "Get details for supplier 22222"

---

### lightspeed_list_staff

**What it does**: Lists all staff members (employees) in Lightspeed Retail POS. Supports filtering by inactive status.

**When to use**: Browse staff, manage employee records.

**Arguments**:
- `isInactive` (optional): Include inactive staff
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all active staff members"

---

## Lightspeed API Notes

- **API Version**: Uses Lightspeed Retail API v2
- **Rate Limits**: 1000 requests per minute for most endpoints
- **Items**: Called "Items" in Lightspeed (products in other systems)
- **Transactions**: Called "Sales" in the API
- **Inventory**: Supports multi-location inventory tracking
- **Employees**: Staff members have roles and permissions
- **Webhooks**: Lightspeed supports webhooks for real-time updates (future expansion)
