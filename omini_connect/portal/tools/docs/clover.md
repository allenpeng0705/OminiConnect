# Clover Integration

## Overview

Clover is a point-of-sale (POS) and commerce platform that helps merchants run their businesses. It provides tools for processing payments, managing orders, tracking inventory, and handling customer relationships.

## Connection

Clover uses OAuth 2.0 for authentication. To connect your Clover account:

1. Log in to the Clover developer portal
2. Create a new app with the required scopes
3. Configure the OAuth redirect URL
4. Authorize the application

## Available Scopes

| Scope | Description |
|-------|-------------|
| `merchant` | Access merchant information |
| `orders:read` | Read order data |
| `items:read` | Read item/product data |
| `employees:read` | Read employee data |
| `customers:read` | Read customer data |

## Tools

### Merchants

#### List Merchants
Retrieve a list of all merchants associated with the account.

**Endpoint:** `GET /merchants`

#### Get Merchant
Retrieve details of a specific merchant.

**Endpoint:** `GET /merchants/{merchant_id}`

### Orders

#### List Orders
Retrieve all orders for a merchant.

**Endpoint:** `GET /merchants/{merchant_id}/orders`

**Parameters:**
- `merchant_id` (string, required): The merchant identifier
- `limit` (integer): Maximum number of orders to return
- `offset` (integer): Number of orders to skip for pagination

#### Get Order
Retrieve details of a specific order.

**Endpoint:** `GET /merchants/{merchant_id}/orders/{order_id}`

**Parameters:**
- `merchant_id` (string, required): The merchant identifier
- `order_id` (string, required): The order identifier

### Items

#### List Items
Retrieve all items (products) for a merchant.

**Endpoint:** `GET /merchants/{merchant_id}/items`

**Parameters:**
- `merchant_id` (string, required): The merchant identifier
- `limit` (integer): Maximum number of items to return

#### Get Item
Retrieve details of a specific item.

**Endpoint:** `GET /merchants/{merchant_id}/items/{item_id}`

**Parameters:**
- `merchant_id` (string, required): The merchant identifier
- `item_id` (string, required): The item identifier

### Employees

#### List Employees
Retrieve all employees for a merchant.

**Endpoint:** `GET /merchants/{merchant_id}/employees`

**Parameters:**
- `merchant_id` (string, required): The merchant identifier
- `limit` (integer): Maximum number of employees to return

#### Get Employee
Retrieve details of a specific employee.

**Endpoint:** `GET /merchants/{merchant_id}/employees/{employee_id}`

**Parameters:**
- `merchant_id` (string, required): The merchant identifier
- `employee_id` (string, required): The employee identifier

### Customers

#### List Customers
Retrieve all customers for a merchant.

**Endpoint:** `GET /merchants/{merchant_id}/customers`

**Parameters:**
- `merchant_id` (string, required): The merchant identifier
- `limit` (integer): Maximum number of customers to return

#### Get Customer
Retrieve details of a specific customer.

**Endpoint:** `GET /merchants/{merchant_id}/customers/{customer_id}`

**Parameters:**
- `merchant_id` (string, required): The merchant identifier
- `customer_id` (string, required): The customer identifier

## Use Cases

- **Order Management**: Track and manage customer orders across multiple locations
- **Inventory Tracking**: Monitor stock levels and product availability
- **Employee Scheduling**: Manage employee schedules and time tracking
- **Customer Insights**: Analyze customer purchase history and behavior
