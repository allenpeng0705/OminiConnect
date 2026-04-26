# Teamleader Focus Tools

Provider: `teamleader` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Teamleader Focus is an all-in-one CRM, project management, and invoicing platform. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Teamleader Focus
- Token stored in Nango, accessed via `connection_ref`
- Scopes: users:read, contacts:read, contacts:write, projects:read, invoices:read

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `teamleader_list_contacts` | List all contacts in Teamleader | POST | /contacts.list |
| `teamleader_get_contact` | Get details of a specific contact | POST | /contacts.info |
| `teamleader_create_contact` | Create a new contact | POST | /contacts.add |
| `teamleader_update_contact` | Update an existing contact | POST | /contacts.update |
| `teamleader_list_companies` | List all companies | POST | /companies.list |
| `teamleader_get_company` | Get details of a company | POST | /companies.info |
| `teamleader_list_projects` | List all projects | POST | /projects.list |
| `teamleader_get_project` | Get details of a project | POST | /projects.info |
| `teamleader_list_invoices` | List all invoices | POST | /invoices.list |
| `teamleader_get_invoice` | Get details of an invoice | POST | /invoices.info |

---

## Tool Details

### teamleader_list_contacts

**What it does**: List all contacts in Teamleader

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamleader_get_contact

**What it does**: Get details of a specific contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamleader_create_contact

**What it does**: Create a new contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamleader_update_contact

**What it does**: Update an existing contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamleader_list_companies

**What it does**: List all companies

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamleader_get_company

**What it does**: Get details of a company

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamleader_list_projects

**What it does**: List all projects

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamleader_get_project

**What it does**: Get details of a project

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamleader_list_invoices

**What it does**: List all invoices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamleader_get_invoice

**What it does**: Get details of an invoice

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.focus.teamleader.eu`
- Docs: https://nango.dev/docs/integrations/all/teamleader
