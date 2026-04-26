# Setmore Tools

Provider: `setmore` | Engine: `nango` | Auth: TWO_STEP

## Overview

These tools wrap the Setmore API. They allow AI agents to interact with Setmore functionality. **Requires TWO_STEP authentication.**

## Authentication

**Two-Step Authentication**:
- Uses a refresh token to obtain access token
- Nango manages token refresh automatically
- Scopes depend on the initial authorization

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_customers` | List all customers | GET | /customer |
| `get_customer` | Get customer details | GET | /customer/{id} |
| `list_appointments` | List all appointments | GET | /appointment |
| `get_appointment` | Get appointment details | GET | /appointment/{id} |
| `create_appointment` | Create an appointment | POST | /appointment |
| `cancel_appointment` | Cancel an appointment | POST | /appointment/{id}/cancel |
| `list_services` | List available services | GET | /service |
| `list_staff` | List staff members | GET | /staff |
| `get_availability` | Get staff availability | GET | /availability/{staff_id} |
| `get_business_profile` | Get business profile | GET | /business |

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

### list_appointments

**What it does**: List all appointments

**When to use**: Use this tool when you need to list all appointments.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list appointments to..."

---

### get_appointment

**What it does**: Get appointment details

**When to use**: Use this tool when you need to get appointment details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get appointment to..."

---

### create_appointment

**What it does**: Create an appointment

**When to use**: Use this tool when you need to create an appointment.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create appointment to..."

---

### cancel_appointment

**What it does**: Cancel an appointment

**When to use**: Use this tool when you need to cancel an appointment.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use cancel appointment to..."

---

### list_services

**What it does**: List available services

**When to use**: Use this tool when you need to list available services.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list services to..."

---

### list_staff

**What it does**: List staff members

**When to use**: Use this tool when you need to list staff members.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list staff to..."

---

### get_availability

**What it does**: Get staff availability

**When to use**: Use this tool when you need to get staff availability.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get availability to..."

---

### get_business_profile

**What it does**: Get business profile

**When to use**: Use this tool when you need to get business profile.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get business profile to..."

---

## Setmore API Notes

- **Auth mode**: TWO_STEP
- **Base URL**: https://developer.setmore.com/api
- **API prefix**: /v1
- **Rate limits**: Check provider documentation for specific limits
