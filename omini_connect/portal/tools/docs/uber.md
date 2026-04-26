# Uber Tools

Provider: `uber` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Uber API for ride delivery and logistics. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Uber
- Token stored in Nango, accessed via `connection_ref`
- Scopes: profile, history, places, request

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `uber_get_user_profile` | Get user profile information | GET | /v1/me |
| `uber_get_user_history` | Get user trip history | GET | /v1.2/history |
| `uber_list_products` | List available Uber products | GET | /v1/products |
| `uber_get_product` | Get product details | GET | /v1/products/{product_id} |
| `uber_get_price_estimate` | Get price estimate for a trip | GET | /v1.2/price |
| `uber_get_time_estimate` | Get time estimate for pickup | GET | /v1.2/time |
| `uber_request_ride` | Request a ride | POST | /v1/requests |
| `uber_get_request` | Get request details | GET | /v1/requests/{request_id} |
| `uber_cancel_request` | Cancel a ride request | DELETE | /v1/requests/{request_id} |
| `uber_list_places` | List saved places | GET | /v1/places/{user_id} |

---

## Tool Details

### uber_get_user_profile

**What it does**: Get user profile information

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### uber_get_user_history

**What it does**: Get user trip history

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### uber_list_products

**What it does**: List available Uber products

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### uber_get_product

**What it does**: Get product details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### uber_get_price_estimate

**What it does**: Get price estimate for a trip

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### uber_get_time_estimate

**What it does**: Get time estimate for pickup

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### uber_request_ride

**What it does**: Request a ride

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### uber_get_request

**What it does**: Get request details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### uber_cancel_request

**What it does**: Cancel a ride request

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### uber_list_places

**What it does**: List saved places

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.uber.com`
- Docs: https://nango.dev/docs/integrations/all/uber
