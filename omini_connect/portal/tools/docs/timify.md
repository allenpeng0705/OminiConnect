# Timify Tools

Provider: `timify` | Engine: `nango` | Auth: Two-Step OAuth via Nango

## Overview

Timify is an appointment scheduling platform. **Requires two-step oauth via nango.**

## Authentication

**Nango Two-Step OAuth**:
- User authenticates via two-step OAuth flow
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `timify_list_events` | List all scheduled events | GET | /api/v1/events |
| `timify_get_event` | Get event details | GET | /api/v1/events/{id} |
| `timify_list_customers` | List all customers | GET | /api/v1/customers |
| `timify_get_customer` | Get customer details | GET | /api/v1/customers/{id} |
| `timify_create_booking` | Create a new booking | POST | /api/v1/bookings |
| `timify_cancel_booking` | Cancel a booking | POST | /api/v1/bookings/{id}/cancel |
| `timify_list_services` | List all services | GET | /api/v1/services |
| `timify_list_resources` | List all resources (staff/rooms) | GET | /api/v1/resources |
| `timify_get_availability` | Get available time slots | GET | /api/v1/availability |
| `timify_create_customer` | Create a new customer | POST | /api/v1/customers |

---

## Tool Details

### timify_list_events

**What it does**: List all scheduled events

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timify_get_event

**What it does**: Get event details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timify_list_customers

**What it does**: List all customers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timify_get_customer

**What it does**: Get customer details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timify_create_booking

**What it does**: Create a new booking

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timify_cancel_booking

**What it does**: Cancel a booking

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timify_list_services

**What it does**: List all services

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timify_list_resources

**What it does**: List all resources (staff/rooms)

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timify_get_availability

**What it does**: Get available time slots

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timify_create_customer

**What it does**: Create a new customer

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.subdomain}.timify.com`
- Docs: https://nango.dev/docs/integrations/all/timify
