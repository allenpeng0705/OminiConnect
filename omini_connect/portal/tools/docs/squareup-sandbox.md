# Squareup (Sandbox) Tools

Provider: `squareup-sandbox` | Engine: `nango` | Auth: OAUTH2

## Overview

These tools wrap the Squareup (Sandbox) API. They allow AI agents to interact with Squareup (Sandbox) functionality. **Requires OAUTH2 authentication.**

## Authentication

**OAuth2 Authentication**:
- User authenticates via OAuth2 authorization code flow
- Nango manages the OAuth handshake and token refresh
- Default scopes depend on the provider configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_payments` | List all payments | GET | /v2/payments |
| `get_payment` | Get payment details | GET | /v2/payments/{id} |
| `list_orders` | List all orders | GET | /v2/orders |
| `get_order` | Get order details | GET | /v2/orders/{id} |
| `list_customers` | List all customers | GET | /v2/customers |
| `get_customer` | Get customer details | GET | /v2/customers/{id} |
| `list_items` | List catalog items | GET | /v2/catalog/list |
| `list_locations` | List business locations | GET | /v2/locations |
| `get_reports` | Get sales reports | GET | /v2/reports |
| `list_refunds` | List all refunds | GET | /v2/refunds |

---

## Tool Details

### list_payments

**What it does**: List all payments

**When to use**: Use this tool when you need to list all payments.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list payments to..."

---

### get_payment

**What it does**: Get payment details

**When to use**: Use this tool when you need to get payment details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get payment to..."

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

### list_items

**What it does**: List catalog items

**When to use**: Use this tool when you need to list catalog items.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list items to..."

---

### list_locations

**What it does**: List business locations

**When to use**: Use this tool when you need to list business locations.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list locations to..."

---

### get_reports

**What it does**: Get sales reports

**When to use**: Use this tool when you need to get sales reports.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get reports to..."

---

### list_refunds

**What it does**: List all refunds

**When to use**: Use this tool when you need to list all refunds.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list refunds to..."

---

## Squareup (Sandbox) API Notes

- **Auth mode**: OAUTH2
- **Base URL**: https://connect.squareupsandbox.com
- **API prefix**: /v2
- **Rate limits**: Check provider documentation for specific limits
