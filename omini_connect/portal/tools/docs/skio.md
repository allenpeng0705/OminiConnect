# Skio Tools

Provider: `skio` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Skio API. They allow AI agents to interact with Skio functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `get_products` | List all subscription products | GET | /products |
| `get_product` | Get product details | GET | /products/{id} |
| `list_subscriptions` | List all subscriptions | GET | /subscriptions |
| `get_subscription` | Get subscription details | GET | /subscriptions/{id} |
| `list_customers` | List all customers | GET | /customers |
| `get_customer` | Get customer details | GET | /customers/{id} |
| `list_orders` | List all orders | GET | /orders |
| `get_order` | Get order details | GET | /orders/{id} |
| `list_charges` | List all charges | GET | /charges |
| `get_checkout` | Get checkout session | GET | /checkouts/{id} |

---

## Tool Details

### get_products

**What it does**: List all subscription products

**When to use**: Use this tool when you need to list all subscription products.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get products to..."

---

### get_product

**What it does**: Get product details

**When to use**: Use this tool when you need to get product details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get product to..."

---

### list_subscriptions

**What it does**: List all subscriptions

**When to use**: Use this tool when you need to list all subscriptions.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list subscriptions to..."

---

### get_subscription

**What it does**: Get subscription details

**When to use**: Use this tool when you need to get subscription details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get subscription to..."

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

### list_charges

**What it does**: List all charges

**When to use**: Use this tool when you need to list all charges.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list charges to..."

---

### get_checkout

**What it does**: Get checkout session

**When to use**: Use this tool when you need to get checkout session.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get checkout to..."

---

## Skio API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://graphql.skio.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
