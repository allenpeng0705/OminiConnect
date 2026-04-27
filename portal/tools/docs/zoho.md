# Zoho Tools

Provider: `zoho` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zoho is an all-in-one business software suite. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoho
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_list_records` | List all CRM records | GET | /crm/v2.1/contacts |
| `zoho_get_record` | Get record details | GET | /crm/v2.1/contacts/{id} |
| `zoho_create_record` | Create a new record | POST | /crm/v2.1/contacts |
| `zoho_update_record` | Update a record | PUT | /crm/v2.1/contacts/{id} |
| `zoho_delete_record` | Delete a record | DELETE | /crm/v2.1/contacts/{id} |
| `zoho_list_deals` | List all deals | GET | /crm/v2.1/deals |
| `zoho_get_deal` | Get deal details | GET | /crm/v2.1/deals/{id} |
| `zoho_create_deal` | Create a new deal | POST | /crm/v2.1/deals |
| `zoho_list_tasks` | List all tasks | GET | /crm/v2.1/tasks |
| `zoho_create_task` | Create a new task | POST | /crm/v2.1/tasks |

---

## Tool Details

### zoho_list_records

**What it does**: List all CRM records

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_get_record

**What it does**: Get record details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_create_record

**What it does**: Create a new record

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_update_record

**What it does**: Update a record

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_delete_record

**What it does**: Delete a record

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_list_deals

**What it does**: List all deals

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_get_deal

**What it does**: Get deal details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_create_deal

**What it does**: Create a new deal

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_list_tasks

**What it does**: List all tasks

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_create_task

**What it does**: Create a new task

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://www.zohoapis.${connectionConfig.extension}`
- Docs: https://nango.dev/docs/integrations/all/zoho
