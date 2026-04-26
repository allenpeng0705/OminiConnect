# Tremendous Tools

Provider: `tremendous` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Tremendous is a payments platform for disbursing rewards and incentives. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Tremendous
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tremendous_list_campaigns` | List all campaigns | GET | /campaigns |
| `tremendous_get_campaign` | Get campaign details | GET | /campaigns/{id} |
| `tremendous_create_campaign` | Create a new campaign | POST | /campaigns |
| `tremendous_list_orders` | List all orders | GET | /orders |
| `tremendous_get_order` | Get order details | GET | /orders/{id} |
| `tremendous_create_order` | Create a new order for payouts | POST | /orders |
| `tremendous_list_recipients` | List all recipients | GET | /recipients |
| `tremendous_get_recipient` | Get recipient details | GET | /recipients/{id} |
| `tremendous_create_recipient` | Create a new recipient | POST | /recipients |
| `tremendous_list_payments` | List all payments | GET | /payments |

---

## Tool Details

### tremendous_list_campaigns

**What it does**: List all campaigns

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tremendous_get_campaign

**What it does**: Get campaign details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tremendous_create_campaign

**What it does**: Create a new campaign

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tremendous_list_orders

**What it does**: List all orders

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tremendous_get_order

**What it does**: Get order details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tremendous_create_order

**What it does**: Create a new order for payouts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tremendous_list_recipients

**What it does**: List all recipients

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tremendous_get_recipient

**What it does**: Get recipient details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tremendous_create_recipient

**What it does**: Create a new recipient

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tremendous_list_payments

**What it does**: List all payments

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.tremendous.com/api/v2`
- Docs: https://nango.dev/docs/integrations/all/tremendous
