# Shopify (Client Credentials) Tools

Provider: `shopify-cc` | Engine: `nango` | Auth: OAUTH2_CC

## Overview

These tools wrap the Shopify (Client Credentials) API. They allow AI agents to interact with Shopify (Client Credentials) functionality. **Requires OAUTH2_CC authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses client_id and client_secret for machine-to-machine auth
- Nango manages token refresh automatically
- Scopes depend on application permissions

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

## Shopify (Client Credentials) API Notes

- **Auth mode**: OAUTH2_CC
- **Base URL**: https://{subdomain}.myshopify.com
- **API prefix**: /admin/api
- **Rate limits**: Check provider documentation for specific limits
