# ServiceNow (Client Credentials) Tools

Provider: `servicenow-oauth2-cc` | Engine: `nango` | Auth: OAUTH2_CC

## Overview

These tools wrap the ServiceNow (Client Credentials) API. They allow AI agents to interact with ServiceNow (Client Credentials) functionality. **Requires OAUTH2_CC authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses client_id and client_secret for machine-to-machine auth
- Nango manages token refresh automatically
- Scopes depend on application permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_incidents` | List all incidents | GET | /api/now/table/incident |
| `get_incident` | Get incident details | GET | /api/now/table/incident/{id} |
| `create_incident` | Create a new incident | POST | /api/now/table/incident |
| `list_users` | List all users | GET | /api/now/table/sys_user |
| `get_user` | Get user details | GET | /api/now/table/sys_user/{id} |
| `list_groups` | List all groups | GET | /api/now/table/sys_user_group |
| `list_kb_articles` | List knowledge articles | GET | /api/now/table/kb_knowledge |
| `list_changes` | List change requests | GET | /api/now/table/change_request |
| `get_catalogue` | Get service catalogue | GET | /api/now/table/sc_cat_item |
| `get_record` | Get any table record | GET | /api/now/table/{table}/{sys_id} |

---

## Tool Details

### list_incidents

**What it does**: List all incidents

**When to use**: Use this tool when you need to list all incidents.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list incidents to..."

---

### get_incident

**What it does**: Get incident details

**When to use**: Use this tool when you need to get incident details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get incident to..."

---

### create_incident

**What it does**: Create a new incident

**When to use**: Use this tool when you need to create a new incident.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create incident to..."

---

### list_users

**What it does**: List all users

**When to use**: Use this tool when you need to list all users.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list users to..."

---

### get_user

**What it does**: Get user details

**When to use**: Use this tool when you need to get user details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get user to..."

---

### list_groups

**What it does**: List all groups

**When to use**: Use this tool when you need to list all groups.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list groups to..."

---

### list_kb_articles

**What it does**: List knowledge articles

**When to use**: Use this tool when you need to list knowledge articles.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list kb articles to..."

---

### list_changes

**What it does**: List change requests

**When to use**: Use this tool when you need to list change requests.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list changes to..."

---

### get_catalogue

**What it does**: Get service catalogue

**When to use**: Use this tool when you need to get service catalogue.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get catalogue to..."

---

### get_record

**What it does**: Get any table record

**When to use**: Use this tool when you need to get any table record.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get record to..."

---

## ServiceNow (Client Credentials) API Notes

- **Auth mode**: OAUTH2_CC
- **Base URL**: https://{subdomain}.service-now.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
