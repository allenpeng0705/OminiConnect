# Stay AI Tools

Provider: `stay-ai` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Stay AI API. They allow AI agents to interact with Stay AI functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_customers` | List all customers | GET | /customers |
| `get_customer` | Get customer details | GET | /customers/{id} |
| `list_subscriptions` | List all subscriptions | GET | /subscriptions |
| `get_subscription` | Get subscription details | GET | /subscriptions/{id} |
| `list_orders` | List all orders | GET | /orders |
| `get_order` | Get order details | GET | /orders/{id} |
| `list_products` | List all products | GET | /products |
| `get_product` | Get product details | GET | /products/{id} |
| `get_churn_analysis` | Get churn analysis | GET | /churn-analysis |
| `get_revenue` | Get revenue metrics | GET | /revenue |

---

## Tool Details

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

### get_churn_analysis

**What it does**: Get churn analysis

**When to use**: Use this tool when you need to get churn analysis.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get churn analysis to..."

---

### get_revenue

**What it does**: Get revenue metrics

**When to use**: Use this tool when you need to get revenue metrics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get revenue to..."

---

## Stay AI API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://api.retextion.com/api
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
