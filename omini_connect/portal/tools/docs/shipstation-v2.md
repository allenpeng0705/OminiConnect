# Shipstation (v2) Tools

Provider: `shipstation-v2` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Shipstation (v2) API. They allow AI agents to interact with Shipstation (v2) functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_orders` | List all orders | GET | /orders |
| `get_order` | Get order details | GET | /orders/{id} |
| `list_shipments` | List shipments | GET | /shipments |
| `create_shipment` | Create a shipment label | POST | /shipments |
| `list_products` | List all products | GET | /products |
| `list_warehouses` | List warehouses | GET | /warehouses |
| `list_carriers` | List shipping carriers | GET | /carriers |
| `get_rates` | Get shipping rates | POST | /shipments/getrates |
| `list_customers` | List customers | GET | /customers |
| `get_user` | Get user account info | GET | /users |

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

### list_shipments

**What it does**: List shipments

**When to use**: Use this tool when you need to list shipments.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list shipments to..."

---

### create_shipment

**What it does**: Create a shipment label

**When to use**: Use this tool when you need to create a shipment label.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create shipment to..."

---

### list_products

**What it does**: List all products

**When to use**: Use this tool when you need to list all products.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list products to..."

---

### list_warehouses

**What it does**: List warehouses

**When to use**: Use this tool when you need to list warehouses.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list warehouses to..."

---

### list_carriers

**What it does**: List shipping carriers

**When to use**: Use this tool when you need to list shipping carriers.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list carriers to..."

---

### get_rates

**What it does**: Get shipping rates

**When to use**: Use this tool when you need to get shipping rates.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get rates to..."

---

### list_customers

**What it does**: List customers

**When to use**: Use this tool when you need to list customers.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list customers to..."

---

### get_user

**What it does**: Get user account info

**When to use**: Use this tool when you need to get user account info.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get user to..."

---

## Shipstation (v2) API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://api.shipstation.com
- **API prefix**: /v2
- **Rate limits**: Check provider documentation for specific limits
