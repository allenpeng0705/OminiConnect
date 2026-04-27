# WorkOS Tools

Provider: `workos` | Engine: `nango` | Auth: API Key via Nango

## Overview

WorkOS is an identity and security platform for enterprise apps. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their WorkOS API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `workos_list_connections` | List all SSO connections | GET | /connections |
| `workos_get_connection` | Get connection details | GET | /connections/{id} |
| `workos_list_directory_users` | List directory users | GET | /directory_users |
| `workos_get_directory_user` | Get directory user details | GET | /directory_users/{id} |
| `workos_list_directory_groups` | List directory groups | GET | /directory_groups |
| `workos_get_directory_group` | Get directory group details | GET | /directory_groups/{id} |
| `workos_list_audit_logs` | List audit log events | GET | /audit_logs/events |
| `workos_create_audit_log` | Create an audit log event | POST | /audit_logs/events |
| `workos_list_users` | List all users | GET | /users |
| `workos_delete_user` | Delete a user | DELETE | /users/{id} |

---

## Tool Details

### workos_list_connections

**What it does**: List all SSO connections

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workos_get_connection

**What it does**: Get connection details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workos_list_directory_users

**What it does**: List directory users

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workos_get_directory_user

**What it does**: Get directory user details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workos_list_directory_groups

**What it does**: List directory groups

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workos_get_directory_group

**What it does**: Get directory group details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workos_list_audit_logs

**What it does**: List audit log events

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workos_create_audit_log

**What it does**: Create an audit log event

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workos_list_users

**What it does**: List all users

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workos_delete_user

**What it does**: Delete a user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.workos.com`
- Docs: https://nango.dev/docs/integrations/all/workos
