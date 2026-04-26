# Trello (SCIM) Tools

Provider: `trello-scim` | Engine: `nango` | Auth: API Key via Nango

## Overview

Trello SCIM API for user provisioning and management. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Trello (SCIM) API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `trello_list_users` | List all users in the organization | GET | /Users |
| `trello_get_user` | Get user details | GET | /Users/{id} |
| `trello_create_user` | Create a new user | POST | /Users |
| `trello_update_user` | Update user attributes | PUT | /Users/{id} |
| `trello_delete_user` | Deactivate a user | DELETE | /Users/{id} |
| `trello_list_groups` | List all groups/organizations | GET | /Groups |
| `trello_get_group` | Get group details | GET | /Groups/{id} |
| `trello_list_org_members` | List members in an organization | GET | /Organizations/{id}/Members |
| `trello_add_user_to_group` | Add a user to a group | POST | /Groups/{id}/Users |
| `trello_remove_user_from_group` | Remove a user from a group | DELETE | /Groups/{id}/Users/{user_id} |

---

## Tool Details

### trello_list_users

**What it does**: List all users in the organization

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trello_get_user

**What it does**: Get user details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trello_create_user

**What it does**: Create a new user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trello_update_user

**What it does**: Update user attributes

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trello_delete_user

**What it does**: Deactivate a user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trello_list_groups

**What it does**: List all groups/organizations

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trello_get_group

**What it does**: Get group details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trello_list_org_members

**What it does**: List members in an organization

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trello_add_user_to_group

**What it does**: Add a user to a group

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trello_remove_user_from_group

**What it does**: Remove a user from a group

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://trello.com/scim`
- Docs: https://nango.dev/docs/integrations/all/trello-scim
