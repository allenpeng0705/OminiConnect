# ShipBob (Personal Access Token) Tools

Provider: `shipbob-pat` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the ShipBob (Personal Access Token) API. They allow AI agents to interact with ShipBob (Personal Access Token) functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_channels` | List sales channels | GET | /2026-01/channel |
| `list_orders` | List all orders | GET | /2026-01/orders |
| `get_order` | Get order details | GET | /2026-01/orders/{id} |
| `list_products` | List all products | GET | /2026-01/product |
| `get_product` | Get product details | GET | /2026-01/product/{id} |
| `list_warehouses` | List warehouses | GET | /2026-01/warehouse |
| `list_shipments` | List shipments | GET | /2026-01/shipment |
| `get_shipment` | Get shipment details | GET | /2026-01/shipment/{id} |
| `list_returns` | List returns | GET | /2026-01/return |
| `get_inventory` | Get inventory levels | GET | /2026-01/inventory |

---

## Tool Details

### list_channels

**What it does**: List sales channels

**When to use**: Use this tool when you need to list sales channels.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list channels to..."

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

### list_warehouses

**What it does**: List warehouses

**When to use**: Use this tool when you need to list warehouses.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list warehouses to..."

---

### list_shipments

**What it does**: List shipments

**When to use**: Use this tool when you need to list shipments.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list shipments to..."

---

### get_shipment

**What it does**: Get shipment details

**When to use**: Use this tool when you need to get shipment details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get shipment to..."

---

### list_returns

**What it does**: List returns

**When to use**: Use this tool when you need to list returns.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list returns to..."

---

### get_inventory

**What it does**: Get inventory levels

**When to use**: Use this tool when you need to get inventory levels.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get inventory to..."

---

## ShipBob (Personal Access Token) API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://{apiSubdomain}.shipbob.com
- **API prefix**: /2026-01
- **Rate limits**: Check provider documentation for specific limits
