# WiseAgent Tools

Provider: `wiseagent` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

WiseAgent is a CRM platform for real estate professionals. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with WiseAgent
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wiseagent_list_contacts` | List all contacts | GET | /api/contacts |
| `wiseagent_get_contact` | Get contact details | GET | /api/contacts/{id} |
| `wiseagent_create_contact` | Create a new contact | POST | /api/contacts |
| `wiseagent_update_contact` | Update a contact | PUT | /api/contacts/{id} |
| `wiseagent_list_leads` | List all leads | GET | /api/leads |
| `wiseagent_get_lead` | Get lead details | GET | /api/leads/{id} |
| `wiseagent_create_lead` | Create a new lead | POST | /api/leads |
| `wiseagent_list_tasks` | List all tasks | GET | /api/tasks |
| `wiseagent_create_task` | Create a new task | POST | /api/tasks |
| `wiseagent_list_emails` | List all emails | GET | /api/emails |

---

## Tool Details

### wiseagent_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiseagent_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiseagent_create_contact

**What it does**: Create a new contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiseagent_update_contact

**What it does**: Update a contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiseagent_list_leads

**What it does**: List all leads

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiseagent_get_lead

**What it does**: Get lead details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiseagent_create_lead

**What it does**: Create a new lead

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiseagent_list_tasks

**What it does**: List all tasks

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiseagent_create_task

**What it does**: Create a new task

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiseagent_list_emails

**What it does**: List all emails

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://sync.thewiseagent.com`
- Docs: https://nango.dev/docs/integrations/all/wiseagent
