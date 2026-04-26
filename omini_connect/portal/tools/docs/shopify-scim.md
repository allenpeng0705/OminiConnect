# Shopify (SCIM API) Tools

Provider: `shopify-scim` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Shopify (SCIM API) API. They allow AI agents to interact with Shopify (SCIM API) functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_products` | List all products | GET | /products.json |
| `get_product` | Get product details | GET | /products/{id}.json |
| `list_orders` | List all orders | GET | /orders.json |
| `get_order` | Get order details | GET | /orders/{id}.json |
| `list_customers` | List customers | GET | /customers.json |
| `get_customer` | Get customer details | GET | /customers/{id}.json |
| `list_fulfillments` | List fulfillments | GET | /fulfillments.json |
| `create_fulfillment` | Create fulfillment | POST | /orders/{order_id}/fulfillments.json |
| `list_inventory` | List inventory levels | GET | /inventory_levels.json |
| `get_shop` | Get shop details | GET | /shop.json |

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

### list_fulfillments

**What it does**: List fulfillments

**When to use**: Use this tool when you need to list fulfillments.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list fulfillments to..."

---

### create_fulfillment

**What it does**: Create fulfillment

**When to use**: Use this tool when you need to create fulfillment.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create fulfillment to..."

---

### list_inventory

**What it does**: List inventory levels

**When to use**: Use this tool when you need to list inventory levels.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list inventory to..."

---

### get_shop

**What it does**: Get shop details

**When to use**: Use this tool when you need to get shop details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get shop to..."

---

## Shopify (SCIM API) API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://shopifyscim.com/scim
- **API prefix**: /v2
- **Rate limits**: Check provider documentation for specific limits
