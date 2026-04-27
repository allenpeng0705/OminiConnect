# Yotpo Tools

Provider: `yotpo` | Engine: `nango` | Auth: Two-Step OAuth via Nango

## Overview

Yotpo is a reviews and loyalty platform for e-commerce. **Requires two-step oauth via nango.**

## Authentication

**Nango Two-Step OAuth**:
- User authenticates via two-step OAuth flow
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `yotpo_list_reviews` | List all product reviews | GET | /v1/reviews |
| `yotpo_get_review` | Get review details | GET | /v1/reviews/{id} |
| `yotpo_create_review` | Create a new review | POST | /v1/reviews |
| `yotpo_list_products` | List all products | GET | /v1/products |
| `yotpo_get_product` | Get product details | GET | /v1/products/{id} |
| `yotpo_list_customers` | List all customers | GET | /v1/customers |
| `yotpo_get_loyalty_status` | Get customer loyalty status | GET | /v1/loyalty/status |
| `yotpo_create_loyalty_event` | Create a loyalty event | POST | /v1/loyalty/events |
| `yotpo_list_coupons` | List all coupons | GET | /v1/coupons |
| `yotpo_create_coupon` | Create a new coupon | POST | /v1/coupons |

---

## Tool Details

### yotpo_list_reviews

**What it does**: List all product reviews

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yotpo_get_review

**What it does**: Get review details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yotpo_create_review

**What it does**: Create a new review

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yotpo_list_products

**What it does**: List all products

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yotpo_get_product

**What it does**: Get product details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yotpo_list_customers

**What it does**: List all customers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yotpo_get_loyalty_status

**What it does**: Get customer loyalty status

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yotpo_create_loyalty_event

**What it does**: Create a loyalty event

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yotpo_list_coupons

**What it does**: List all coupons

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yotpo_create_coupon

**What it does**: Create a new coupon

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.yotpo.com`
- Docs: https://nango.dev/docs/integrations/all/yotpo
