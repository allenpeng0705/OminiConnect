# Trafft Tools

Provider: `trafft` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

Trafft is a booking and scheduling platform for service businesses. **Requires oauth2 client credentials via nango.**

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client credentials flow for app-level access
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `trafft_list_bookings` | List all bookings | GET | /v2/bookings |
| `trafft_get_booking` | Get booking details | GET | /v2/bookings/{id} |
| `trafft_create_booking` | Create a new booking | POST | /v2/bookings |
| `trafft_update_booking` | Update a booking | PUT | /v2/bookings/{id} |
| `trafft_cancel_booking` | Cancel a booking | POST | /v2/bookings/{id}/cancel |
| `trafft_list_customers` | List all customers | GET | /v2/customers |
| `trafft_get_customer` | Get customer details | GET | /v2/customers/{id} |
| `trafft_list_services` | List all services | GET | /v2/services |
| `trafft_list_staff` | List all staff members | GET | /v2/staff |
| `trafft_get_availability` | Get available time slots | GET | /v2/availability |

---

## Tool Details

### trafft_list_bookings

**What it does**: List all bookings

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trafft_get_booking

**What it does**: Get booking details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trafft_create_booking

**What it does**: Create a new booking

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trafft_update_booking

**What it does**: Update a booking

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trafft_cancel_booking

**What it does**: Cancel a booking

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trafft_list_customers

**What it does**: List all customers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trafft_get_customer

**What it does**: Get customer details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trafft_list_services

**What it does**: List all services

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trafft_list_staff

**What it does**: List all staff members

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trafft_get_availability

**What it does**: Get available time slots

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.subdomain}.admin.trafft.com/api`
- Docs: https://nango.dev/docs/integrations/all/trafft
