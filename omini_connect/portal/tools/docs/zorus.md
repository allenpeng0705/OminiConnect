# Zorus Tools

Provider: `zorus` | Engine: `nango` | Auth: API Key via Nango

## Overview

Zorus is an IT management platform for MSPs. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Zorus API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zorus_list_tenants` | List all managed tenants | GET | /api/v1/tenants |
| `zorus_get_tenant` | Get tenant details | GET | /api/v1/tenants/{id} |
| `zorus_list_devices` | List all devices | GET | /api/v1/devices |
| `zorus_get_device` | Get device details | GET | /api/v1/devices/{id} |
| `zorus_list_software` | List installed software | GET | /api/v1/software |
| `zorus_list_tickets` | List all tickets | GET | /api/v1/tickets |
| `zorus_create_ticket` | Create a new ticket | POST | /api/v1/tickets |
| `zorus_list_alerts` | List all alerts | GET | /api/v1/alerts |
| `zorus_acknowledge_alert` | Acknowledge an alert | POST | /api/v1/alerts/{id}/acknowledge |
| `zorus_list_users` | List all users | GET | /api/v1/users |

---

## Tool Details

### zorus_list_tenants

**What it does**: List all managed tenants

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zorus_get_tenant

**What it does**: Get tenant details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zorus_list_devices

**What it does**: List all devices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zorus_get_device

**What it does**: Get device details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zorus_list_software

**What it does**: List installed software

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zorus_list_tickets

**What it does**: List all tickets

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zorus_create_ticket

**What it does**: Create a new ticket

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zorus_list_alerts

**What it does**: List all alerts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zorus_acknowledge_alert

**What it does**: Acknowledge an alert

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zorus_list_users

**What it does**: List all users

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://developer.zorustech.com`
- Docs: https://nango.dev/docs/integrations/all/zorus
