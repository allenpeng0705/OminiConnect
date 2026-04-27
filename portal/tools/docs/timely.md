# Timely Tools

Provider: `timely` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Timely is an appointment scheduling software for service businesses. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Timely
- Token stored in Nango, accessed via `connection_ref`
- Scopes: bookings:read, bookings:write, clients:read, clients:write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `timely_list_bookings` | List all bookings/appointments | GET | /1.1/bookings |
| `timely_get_booking` | Get booking details | GET | /1.1/bookings/{id} |
| `timely_create_booking` | Create a new booking | POST | /1.1/bookings |
| `timely_update_booking` | Update a booking | PUT | /1.1/bookings/{id} |
| `timely_delete_booking` | Delete a booking | DELETE | /1.1/bookings/{id} |
| `timely_list_clients` | List all clients | GET | /1.1/clients |
| `timely_get_client` | Get client details | GET | /1.1/clients/{id} |
| `timely_create_client` | Create a new client | POST | /1.1/clients |
| `timely_list_services` | List all services | GET | /1.1/services |
| `timely_list_staff` | List all staff members | GET | /1.1/staff |

---

## Tool Details

### timely_list_bookings

**What it does**: List all bookings/appointments

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timely_get_booking

**What it does**: Get booking details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timely_create_booking

**What it does**: Create a new booking

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timely_update_booking

**What it does**: Update a booking

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timely_delete_booking

**What it does**: Delete a booking

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timely_list_clients

**What it does**: List all clients

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timely_get_client

**What it does**: Get client details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timely_create_client

**What it does**: Create a new client

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timely_list_services

**What it does**: List all services

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### timely_list_staff

**What it does**: List all staff members

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.timelyapp.com`
- Docs: https://nango.dev/docs/integrations/all/timely
