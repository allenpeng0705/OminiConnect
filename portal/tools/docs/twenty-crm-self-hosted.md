# Twenty CRM (Self-Hosted) Tools

Provider: `twenty-crm-self-hosted` | Engine: `nango` | Auth: API Key via Nango

## Overview

Twenty is an open-source CRM platform (self-hosted version). **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Twenty CRM (Self-Hosted) API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `twenty_list_companies` | List all companies | GET | /companies |
| `twenty_get_company` | Get company details | GET | /companies/{id} |
| `twenty_create_company` | Create a new company | POST | /companies |
| `twenty_list_contacts` | List all contacts | GET | /contacts |
| `twenty_get_contact` | Get contact details | GET | /contacts/{id} |
| `twenty_create_contact` | Create a new contact | POST | /contacts |
| `twenty_list_opportunities` | List all opportunities | GET | /opportunities |
| `twenty_get_opportunity` | Get opportunity details | GET | /opportunities/{id} |
| `twenty_create_opportunity` | Create a new opportunity | POST | /opportunities |
| `twenty_list_tasks` | List all tasks | GET | /tasks |

---

## Tool Details

### twenty_list_companies

**What it does**: List all companies

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twenty_get_company

**What it does**: Get company details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twenty_create_company

**What it does**: Create a new company

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twenty_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twenty_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twenty_create_contact

**What it does**: Create a new contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twenty_list_opportunities

**What it does**: List all opportunities

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twenty_get_opportunity

**What it does**: Get opportunity details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twenty_create_opportunity

**What it does**: Create a new opportunity

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twenty_list_tasks

**What it does**: List all tasks

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.domain}/rest`
- Docs: https://nango.dev/docs/integrations/all/twenty-crm-self-hosted
