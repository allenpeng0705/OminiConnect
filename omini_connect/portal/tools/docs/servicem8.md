# ServiceM8 Tools

Provider: `servicem8` | Engine: `nango` | Auth: OAUTH2

## Overview

These tools wrap the ServiceM8 API. They allow AI agents to interact with ServiceM8 functionality. **Requires OAUTH2 authentication.**

## Authentication

**OAuth2 Authentication**:
- User authenticates via OAuth2 authorization code flow
- Nango manages the OAuth handshake and token refresh
- Default scopes depend on the provider configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_jobs` | List all jobs | GET | /jobs |
| `get_job` | Get job details | GET | /jobs/{id} |
| `list_activities` | List all activities | GET | /activities |
| `list_contacts` | List all contacts | GET | /contacts |
| `get_contact` | Get contact details | GET | /contacts/{id} |
| `list_invoices` | List all invoices | GET | /invoices |
| `get_invoice` | Get invoice details | GET | /invoices/{id} |
| `list_products` | List all products | GET | /products |
| `list_schedule` | List scheduled items | GET | /schedule |
| `get_account` | Get account details | GET | /account |

---

## Tool Details

### list_jobs

**What it does**: List all jobs

**When to use**: Use this tool when you need to list all jobs.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list jobs to..."

---

### get_job

**What it does**: Get job details

**When to use**: Use this tool when you need to get job details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get job to..."

---

### list_activities

**What it does**: List all activities

**When to use**: Use this tool when you need to list all activities.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list activities to..."

---

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

### list_products

**What it does**: List all products

**When to use**: Use this tool when you need to list all products.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list products to..."

---

### list_schedule

**What it does**: List scheduled items

**When to use**: Use this tool when you need to list scheduled items.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list schedule to..."

---

### get_account

**What it does**: Get account details

**When to use**: Use this tool when you need to get account details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get account to..."

---

## ServiceM8 API Notes

- **Auth mode**: OAUTH2
- **Base URL**: https://api.servicem8.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
