# Pax8 Tools

Provider: `pax8` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Pax8 cloud marketplace API. They allow AI agents to manage companies, subscriptions, orders, products, and invoices. **Requires Pax8 OAuth2 Client Credentials authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses Pax8 OAuth client credentials
- Client credentials passed via Nango
- Token URL: `https://api.pax8.com/v1/token`
- Base URL: `https://api.pax8.com`
- Audience: `https://api.pax8.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pax8_list_companies` | List companies | GET | /v1/companies |
| `pax8_get_company` | Get company details | GET | /v1/companies/{id} |
| `pax8_list_subscriptions` | List subscriptions | GET | /v1/subscriptions |
| `pax8_get_subscription` | Get subscription details | GET | /v1/subscriptions/{id} |
| `pax8_list_orders` | List orders | GET | /v1/orders |
| `pax8_get_order` | Get order details | GET | /v1/orders/{id} |
| `pax8_list_products` | List products | GET | /v1/products |
| `pax8_get_product` | Get product details | GET | /v1/products/{id} |
| `pax8_list_invoices` | List invoices | GET | /v1/invoices |
| `pax8_get_invoice` | Get invoice details | GET | /v1/invoices/{id} |

---

## Tool Details

### pax8_list_companies

**What it does**: Lists all companies in Pax8.

**When to use**: Browse companies, find customers.

**Arguments**:
- `limit` (optional): Number of companies (default 50)

**Example LLM prompt**: "List all companies in Pax8"

---

### pax8_get_company

**What it does**: Gets detailed information for a specific company.

**When to use**: View company details, subscriptions.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get details for company ABC123"

---

### pax8_list_subscriptions

**What it does**: Lists all subscriptions in Pax8.

**When to use**: Browse subscriptions, manage services.

**Arguments**:
- `company_id` (optional): Filter by company ID

**Example LLM prompt**: "List all subscriptions for company ABC123"

---

### pax8_get_subscription

**What it does**: Gets detailed information for a specific subscription.

**When to use**: View subscription details, status.

**Arguments**:
- `id` (required): Subscription ID

**Example LLM prompt**: "Get details for subscription SUB123"

---

### pax8_list_orders

**What it does**: Lists all orders in Pax8.

**When to use**: Browse order history, track purchases.

**Arguments**:
- `limit` (optional): Number of orders (default 50)

**Example LLM prompt**: "List all recent orders"

---

### pax8_get_order

**What it does**: Gets detailed information for a specific order.

**When to use**: View order details, line items.

**Arguments**:
- `id` (required): Order ID

**Example LLM prompt**: "Get details for order ORD123"

---

### pax8_list_products

**What it does**: Lists all products in Pax8.

**When to use**: Browse product catalog, find services.

**Arguments**: None

**Example LLM prompt**: "List all available products"

---

### pax8_get_product

**What it does**: Gets detailed information for a specific product.

**When to use**: View product details, pricing.

**Arguments**:
- `id` (required): Product ID

**Example LLM prompt**: "Get details for product PRD123"

---

### pax8_list_invoices

**What it does**: Lists all invoices in Pax8.

**When to use**: Browse invoices, track billing.

**Arguments**:
- `company_id` (optional): Filter by company ID

**Example LLM prompt**: "List all invoices for company ABC123"

---

### pax8_get_invoice

**What it does**: Gets detailed information for a specific invoice.

**When to use**: View invoice details, line items.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV123"

---

## Pax8 Notes

- **Cloud marketplace**: SaaS procurement platform
- **Companies**: End customers in the marketplace
- **Subscriptions**: Active service subscriptions
- **Orders**: Purchase transactions
- **Invoices**: Billing documents
