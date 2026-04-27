# Typefully v2 Tools

Provider: `typefully-v2` | Engine: `nango` | Auth: API Key via Nango

## Overview

Typefully v2 API for thread and content management. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Typefully v2 API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `typefully_list_threads` | List all threads | GET | /v1/threads |
| `typefully_get_thread` | Get thread details | GET | /v1/threads/{id} |
| `typefully_create_thread` | Create a new thread | POST | /v1/threads |
| `typefully_update_thread` | Update an existing thread | PUT | /v1/threads/{id} |
| `typefully_delete_thread` | Delete a thread | DELETE | /v1/threads/{id} |
| `typefully_publish_thread` | Publish a thread to Twitter | POST | /v1/threads/{id}/publish |
| `typefully_list_drafts` | List all drafts | GET | /v1/drafts |
| `typefully_get_analytics` | Get thread analytics | GET | /v1/threads/{id}/analytics |
| `typefully_get_profile` | Get user profile | GET | /v1/profile |
| `typefully_share_thread` | Share a thread | POST | /v1/threads/{id}/share |

---

## Tool Details

### typefully_list_threads

**What it does**: List all threads

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### typefully_get_thread

**What it does**: Get thread details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### typefully_create_thread

**What it does**: Create a new thread

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### typefully_update_thread

**What it does**: Update an existing thread

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### typefully_delete_thread

**What it does**: Delete a thread

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### typefully_publish_thread

**What it does**: Publish a thread to Twitter

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### typefully_list_drafts

**What it does**: List all drafts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### typefully_get_analytics

**What it does**: Get thread analytics

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### typefully_get_profile

**What it does**: Get user profile

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### typefully_share_thread

**What it does**: Share a thread

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.typefully.com`
- Docs: https://nango.dev/docs/integrations/all/typefully-v2
