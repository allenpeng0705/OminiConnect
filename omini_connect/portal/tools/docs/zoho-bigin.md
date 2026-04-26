# Zoho Bigin Tools

Provider: `zoho-bigin` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zoho Bigin is a CRM for small businesses. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoho Bigin
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_bigin_list_contacts` | List all contacts | GET | /crm/v2.1/contacts |
| `zoho_bigin_get_contact` | Get contact details | GET | /crm/v2.1/contacts/{id} |
| `zoho_bigin_create_contact` | Create a new contact | POST | /crm/v2.1/contacts |
| `zoho_bigin_list_deals` | List all deals | GET | /crm/v2.1/deals |
| `zoho_bigin_get_deal` | Get deal details | GET | /crm/v2.1/deals/{id} |
| `zoho_bigin_create_deal` | Create a new deal | POST | /crm/v2.1/deals |
| `zoho_bigin_list_pipeline` | List all pipelines | GET | /crm/v2.1/pipeline |
| `zoho_bigin_list_tasks` | List all tasks | GET | /crm/v2.1/tasks |
| `zoho_bigin_create_task` | Create a new task | POST | /crm/v2.1/tasks |
| `zoho_bigin_list_events` | List all events | GET | /crm/v2.1/events |

---

## Tool Details

### zoho_bigin_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_bigin_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_bigin_create_contact

**What it does**: Create a new contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_bigin_list_deals

**What it does**: List all deals

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_bigin_get_deal

**What it does**: Get deal details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_bigin_create_deal

**What it does**: Create a new deal

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_bigin_list_pipeline

**What it does**: List all pipelines

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_bigin_list_tasks

**What it does**: List all tasks

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_bigin_create_task

**What it does**: Create a new task

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_bigin_list_events

**What it does**: List all events

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://www.zohoapis.com/bigin`
- Docs: https://nango.dev/docs/integrations/all/zoho-bigin
