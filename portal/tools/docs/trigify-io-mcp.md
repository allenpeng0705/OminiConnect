# Trigify Tools

Provider: `trigify-io-mcp` | Engine: `nango` | Auth: MCP OAuth2 via Nango

## Overview

Trigify is a marketing automation platform. **Requires mcp oauth2 via nango.**

## Authentication

**MCP OAuth2**:
- Uses MCP OAuth2 flow
- Token stored in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `trigify_list_campaigns` | List all marketing campaigns | GET | /api/campaigns |
| `trigify_get_campaign` | Get campaign details | GET | /api/campaigns/{id} |
| `trigify_create_campaign` | Create a new campaign | POST | /api/campaigns |
| `trigify_list_leads` | List all leads | GET | /api/leads |
| `trigify_get_lead` | Get lead details | GET | /api/leads/{id} |
| `trigify_create_lead` | Create a new lead | POST | /api/leads |
| `trigify_list_automations` | List automation workflows | GET | /api/automations |
| `trigify_trigger_automation` | Trigger an automation | POST | /api/automations/{id}/trigger |
| `trigify_list_templates` | List message templates | GET | /api/templates |
| `trigify_send_message` | Send a message to a lead | POST | /api/messages |

---

## Tool Details

### trigify_list_campaigns

**What it does**: List all marketing campaigns

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trigify_get_campaign

**What it does**: Get campaign details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trigify_create_campaign

**What it does**: Create a new campaign

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trigify_list_leads

**What it does**: List all leads

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trigify_get_lead

**What it does**: Get lead details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trigify_create_lead

**What it does**: Create a new lead

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trigify_list_automations

**What it does**: List automation workflows

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trigify_trigger_automation

**What it does**: Trigger an automation

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trigify_list_templates

**What it does**: List message templates

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trigify_send_message

**What it does**: Send a message to a lead

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.trigify.io`
- Docs: https://nango.dev/docs/integrations/all/trigify-io-mcp
