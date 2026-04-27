# Zendesk Sell Tools

Provider: `zendesk-sell` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zendesk Sell is a CRM platform for sales teams. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zendesk Sell
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zendesk_list_contacts` | List all contacts | GET | /v2/contacts |
| `zendesk_get_contact` | Get contact details | GET | /v2/contacts/{id} |
| `zendesk_create_contact` | Create a new contact | POST | /v2/contacts |
| `zendesk_update_contact` | Update a contact | PUT | /v2/contacts/{id} |
| `zendesk_list_deals` | List all deals | GET | /v2/deals |
| `zendesk_get_deal` | Get deal details | GET | /v2/deals/{id} |
| `zendesk_create_deal` | Create a new deal | POST | /v2/deals |
| `zendesk_list_tasks` | List all tasks | GET | /v2/tasks |
| `zendesk_get_task` | Get task details | GET | /v2/tasks/{id} |
| `zendesk_create_task` | Create a new task | POST | /v2/tasks |

---

## Tool Details

### zendesk_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zendesk_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zendesk_create_contact

**What it does**: Create a new contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zendesk_update_contact

**What it does**: Update a contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zendesk_list_deals

**What it does**: List all deals

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zendesk_get_deal

**What it does**: Get deal details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zendesk_create_deal

**What it does**: Create a new deal

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zendesk_list_tasks

**What it does**: List all tasks

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zendesk_get_task

**What it does**: Get task details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zendesk_create_task

**What it does**: Create a new task

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.getbase.com`
- Docs: https://nango.dev/docs/integrations/all/zendesk-sell
