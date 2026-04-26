# Sellercloud Tools

Provider: `sellercloud` | Engine: `nango` | Auth: TWO_STEP

## Overview

These tools wrap the Sellercloud API. They allow AI agents to interact with Sellercloud functionality. **Requires TWO_STEP authentication.**

## Authentication

**Two-Step Authentication**:
- Uses a refresh token to obtain access token
- Nango manages token refresh automatically
- Scopes depend on the initial authorization

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_products` | List all products | GET | /products |
| `get_product` | Get product details | GET | /products/{id} |
| `list_orders` | List all orders | GET | /orders |
| `get_order` | Get order details | GET | /orders/{id} |
| `list_customers` | List all customers | GET | /customers |
| `get_customer` | Get customer details | GET | /customers/{id} |
| `list_inventory` | List inventory levels | GET | /inventory |
| `update_inventory` | Update inventory levels | PUT | /inventory/{id} |
| `list_shipments` | List shipments | GET | /shipments |
| `get_vendor` | Get vendor details | GET | /vendors/{id} |

---

## Tool Details

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

### list_customers

**What it does**: List all customers

**When to use**: Use this tool when you need to list all customers.

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

### update_inventory

**What it does**: Update inventory levels

**When to use**: Use this tool when you need to update inventory levels.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use update inventory to..."

---

### list_shipments

**What it does**: List shipments

**When to use**: Use this tool when you need to list shipments.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list shipments to..."

---

### get_vendor

**What it does**: Get vendor details

**When to use**: Use this tool when you need to get vendor details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get vendor to..."

---

## Sellercloud API Notes

- **Auth mode**: TWO_STEP
- **Base URL**: {restApiEndpoint}
- **API prefix**: /api
- **Rate limits**: Check provider documentation for specific limits
