# ShopWorks Tools

Provider: `shopworks` | Engine: `nango` | Auth: TWO_STEP

## Overview

These tools wrap the ShopWorks API. They allow AI agents to interact with ShopWorks functionality. **Requires TWO_STEP authentication.**

## Authentication

**Two-Step Authentication**:
- Uses a refresh token to obtain access token
- Nango manages token refresh automatically
- Scopes depend on the initial authorization

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_orders` | List all orders | GET | /orders |
| `get_order` | Get order details | GET | /orders/{id} |
| `list_products` | List all products | GET | /products |
| `get_product` | Get product details | GET | /products/{id} |
| `list_customers` | List customers | GET | /customers |
| `get_customer` | Get customer details | GET | /customers/{id} |
| `list_inventory` | List inventory levels | GET | /inventory |
| `list_shipments` | List shipments | GET | /shipments |
| `list_payments` | List payments | GET | /payments |
| `get_settings` | Get store settings | GET | /settings |

---

## Tool Details

### list_orders

**What it does**: List all orders

**When to use**: Use this tool when you need to list all orders.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list orders to..."

---

### get_order

**What it does**: Get order details

**When to use**: Use this tool when you need to get order details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get order to..."

---

### list_products

**What it does**: List all products

**When to use**: Use this tool when you need to list all products.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list products to..."

---

### get_product

**What it does**: Get product details

**When to use**: Use this tool when you need to get product details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get product to..."

---

### list_customers

**What it does**: List customers

**When to use**: Use this tool when you need to list customers.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list customers to..."

---

### get_customer

**What it does**: Get customer details

**When to use**: Use this tool when you need to get customer details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get customer to..."

---

### list_inventory

**What it does**: List inventory levels

**When to use**: Use this tool when you need to list inventory levels.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list inventory to..."

---

### list_shipments

**What it does**: List shipments

**When to use**: Use this tool when you need to list shipments.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list shipments to..."

---

### list_payments

**What it does**: List payments

**When to use**: Use this tool when you need to list payments.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list payments to..."

---

### get_settings

**What it does**: Get store settings

**When to use**: Use this tool when you need to get store settings.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get settings to..."

---

## ShopWorks API Notes

- **Auth mode**: TWO_STEP
- **Base URL**: https://{hostname}
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
