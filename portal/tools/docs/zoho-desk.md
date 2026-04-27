# Zoho Desk Tools

Provider: `zoho-desk` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zoho Desk is a customer support platform. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoho Desk
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_desk_list_tickets` | List all tickets | GET | /v1/tickets |
| `zoho_desk_get_ticket` | Get ticket details | GET | /v1/tickets/{id} |
| `zoho_desk_create_ticket` | Create a new ticket | POST | /v1/tickets |
| `zoho_desk_update_ticket` | Update a ticket | PUT | /v1/tickets/{id} |
| `zoho_desk_list_comments` | List all comments on a ticket | GET | /v1/tickets/{id}/comments |
| `zoho_desk_add_comment` | Add a comment to a ticket | POST | /v1/tickets/{id}/comments |
| `zoho_desk_list_contacts` | List all contacts | GET | /v1/contacts |
| `zoho_desk_get_contact` | Get contact details | GET | /v1/contacts/{id} |
| `zoho_desk_list_teams` | List all support teams | GET | /v1/teams |
| `zoho_desk_list_agents` | List all agents | GET | /v1/agents |

---

## Tool Details

### zoho_desk_list_tickets

**What it does**: List all tickets

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_desk_get_ticket

**What it does**: Get ticket details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_desk_create_ticket

**What it does**: Create a new ticket

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_desk_update_ticket

**What it does**: Update a ticket

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_desk_list_comments

**What it does**: List all comments on a ticket

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_desk_add_comment

**What it does**: Add a comment to a ticket

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_desk_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_desk_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_desk_list_teams

**What it does**: List all support teams

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_desk_list_agents

**What it does**: List all agents

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://desk.zoho.com/api`
- Docs: https://nango.dev/docs/integrations/all/zoho-desk
