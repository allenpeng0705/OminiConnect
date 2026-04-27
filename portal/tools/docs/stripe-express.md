# Stripe Express Tools

Provider: `stripe-express` | Engine: `nango` | Auth: OAUTH2

## Overview

These tools wrap the Stripe Express API. They allow AI agents to interact with Stripe Express functionality. **Requires OAUTH2 authentication.**

## Authentication

**OAuth2 Authentication**:
- User authenticates via OAuth2 authorization code flow
- Nango manages the OAuth handshake and token refresh
- Default scopes depend on the provider configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_customers` | List all customers | GET | /customers |
| `get_customer` | Get customer details | GET | /customers/{id} |
| `list_payments` | List all payments | GET | /payment_intents |
| `get_payment` | Get payment details | GET | /payment_intents/{id} |
| `list_invoices` | List all invoices | GET | /invoices |
| `get_invoice` | Get invoice details | GET | /invoices/{id} |
| `list_subscriptions` | List all subscriptions | GET | /subscriptions |
| `get_subscription` | Get subscription details | GET | /subscriptions/{id} |
| `list_products` | List all products | GET | /products |
| `get_balance` | Get account balance | GET | /balance |

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

### list_invoices

**What it does**: List all invoices

**When to use**: Use this tool when you need to list all invoices.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list invoices to..."

---

### get_invoice

**What it does**: Get invoice details

**When to use**: Use this tool when you need to get invoice details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get invoice to..."

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

### list_products

**What it does**: List all products

**When to use**: Use this tool when you need to list all products.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list products to..."

---

### get_balance

**What it does**: Get account balance

**When to use**: Use this tool when you need to get account balance.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get balance to..."

---

## Stripe Express API Notes

- **Auth mode**: OAUTH2
- **Base URL**: https://api.stripe.com
- **API prefix**: /v1
- **Rate limits**: Check provider documentation for specific limits
