# Thrivecart (OAuth) Tools

Provider: `thrivecart-oauth` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Thrivecart is an e-commerce platform for selling products and services. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Thrivecart (OAuth)
- Token stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `thrivecart_list_products` | List all products | GET | /products |
| `thrivecart_get_product` | Get product details | GET | /products/{product_id} |
| `thrivecart_list_orders` | List all orders | GET | /orders |
| `thrivecart_get_order` | Get order details | GET | /orders/{order_id} |
| `thrivecart_list_customers` | List all customers | GET | /customers |
| `thrivecart_get_customer` | Get customer details | GET | /customers/{customer_id} |
| `thrivecart_list_affiliates` | List all affiliates | GET | /affiliates |
| `thrivecart_create_product` | Create a new product | POST | /products |
| `thrivecart_list_coupons` | List all coupons | GET | /coupons |
| `thrivecart_refund_order` | Refund an order | POST | /orders/{order_id}/refund |

---

## Tool Details

### thrivecart_list_products

**What it does**: List all products

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### thrivecart_get_product

**What it does**: Get product details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### thrivecart_list_orders

**What it does**: List all orders

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### thrivecart_get_order

**What it does**: Get order details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### thrivecart_list_customers

**What it does**: List all customers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### thrivecart_get_customer

**What it does**: Get customer details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### thrivecart_list_affiliates

**What it does**: List all affiliates

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### thrivecart_create_product

**What it does**: Create a new product

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### thrivecart_list_coupons

**What it does**: List all coupons

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### thrivecart_refund_order

**What it does**: Refund an order

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://thrivecart.com/api/external`
- Docs: https://nango.dev/docs/integrations/all/thrivecart-oauth
