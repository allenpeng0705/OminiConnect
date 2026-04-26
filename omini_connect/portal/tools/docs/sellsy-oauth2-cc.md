# Sellsy (Client Credentials) Tools

Provider: `sellsy-oauth2-cc` | Engine: `nango` | Auth: OAUTH2_CC

## Overview

These tools wrap the Sellsy (Client Credentials) API. They allow AI agents to interact with Sellsy (Client Credentials) functionality. **Requires OAUTH2_CC authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses client_id and client_secret for machine-to-machine auth
- Nango manages token refresh automatically
- Scopes depend on application permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_contacts` | List all contacts | GET | /contacts |
| `get_contact` | Get contact details | GET | /contacts/{id} |
| `list_invoices` | List all invoices | GET | /invoices |
| `get_invoice` | Get invoice details | GET | /invoices/{id} |
| `list_tasks` | List all tasks | GET | /tasks |
| `get_task` | Get task details | GET | /tasks/{id} |
| `list_estimates` | List all estimates | GET | /estimates |
| `get_estimate` | Get estimate details | GET | /estimates/{id} |
| `list_payments` | List all payments | GET | /payments |
| `get_company` | Get company profile | GET | /company |

---

## Tool Details

### list_contacts

**What it does**: List all contacts

**When to use**: Use this tool when you need to list all contacts.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list contacts to..."

---

### get_contact

**What it does**: Get contact details

**When to use**: Use this tool when you need to get contact details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get contact to..."

---

### list_invoices

**What it does**: List all invoices

**When to use**: Use this tool when you need to list all invoices.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list invoices to..."

---

### get_invoice

**What it does**: Get invoice details

**When to use**: Use this tool when you need to get invoice details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get invoice to..."

---

### list_tasks

**What it does**: List all tasks

**When to use**: Use this tool when you need to list all tasks.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list tasks to..."

---

### get_task

**What it does**: Get task details

**When to use**: Use this tool when you need to get task details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get task to..."

---

### list_estimates

**What it does**: List all estimates

**When to use**: Use this tool when you need to list all estimates.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list estimates to..."

---

### get_estimate

**What it does**: Get estimate details

**When to use**: Use this tool when you need to get estimate details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get estimate to..."

---

### list_payments

**What it does**: List all payments

**When to use**: Use this tool when you need to list all payments.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list payments to..."

---

### get_company

**What it does**: Get company profile

**When to use**: Use this tool when you need to get company profile.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get company to..."

---

## Sellsy (Client Credentials) API Notes

- **Auth mode**: OAUTH2_CC
- **Base URL**: https://api.sellsy.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
