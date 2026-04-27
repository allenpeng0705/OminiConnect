# Torii Tools

Provider: `torii` | Engine: `nango` | Auth: API Key via Nango

## Overview

Torii is a SaaS management platform for tracking and managing apps. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Torii API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `torii_list_apps` | List all tracked SaaS applications | GET | /api/v1/apps |
| `torii_get_app` | Get application details | GET | /api/v1/apps/{id} |
| `torii_list_users` | List all users across apps | GET | /api/v1/users |
| `torii_get_user` | Get user details | GET | /api/v1/users/{id} |
| `torii_list_groups` | List all groups | GET | /api/v1/groups |
| `torii_create_app` | Add a new SaaS app to track | POST | /api/v1/apps |
| `torii_update_app` | Update app details | PUT | /api/v1/apps/{id} |
| `torii_list_activities` | List user activity logs | GET | /api/v1/activities |
| `torii_list_integrations` | List connected integrations | GET | /api/v1/integrations |
| `torii_get_spend` | Get spending analytics | GET | /api/v1/spend |

---

## Tool Details

### torii_list_apps

**What it does**: List all tracked SaaS applications

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### torii_get_app

**What it does**: Get application details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### torii_list_users

**What it does**: List all users across apps

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### torii_get_user

**What it does**: Get user details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### torii_list_groups

**What it does**: List all groups

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### torii_create_app

**What it does**: Add a new SaaS app to track

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### torii_update_app

**What it does**: Update app details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### torii_list_activities

**What it does**: List user activity logs

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### torii_list_integrations

**What it does**: List connected integrations

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### torii_get_spend

**What it does**: Get spending analytics

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.toriihq.com`
- Docs: https://nango.dev/docs/integrations/all/torii
