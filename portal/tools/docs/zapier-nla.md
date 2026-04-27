# Zapier NLA Tools

Provider: `zapier-nla` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zapier NLA allows AI agents to interact with Zapier workflows. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zapier NLA
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zapier_list_zaps` | List all Zaps | GET | /zaps |
| `zapier_get_zap` | Get Zap details | GET | /zaps/{id} |
| `zapier_trigger_zap` | Trigger a Zap | POST | /zaps/{id}/trigger |
| `zapier_list_actions` | List available actions | GET | /actions |
| `zapier_perform_action` | Perform an action | POST | /actions/perform |
| `zapier_list_searches` | List available searches | GET | /searches |
| `zapier_perform_search` | Perform a search | POST | /searches/perform |
| `zapier_list_history` | List Zap run history | GET | /history |
| `zapier_get_run` | Get Zap run details | GET | /history/{id} |
| `zapier_get_me` | Get user info | GET | /me |

---

## Tool Details

### zapier_list_zaps

**What it does**: List all Zaps

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zapier_get_zap

**What it does**: Get Zap details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zapier_trigger_zap

**What it does**: Trigger a Zap

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zapier_list_actions

**What it does**: List available actions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zapier_perform_action

**What it does**: Perform an action

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zapier_list_searches

**What it does**: List available searches

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zapier_perform_search

**What it does**: Perform a search

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zapier_list_history

**What it does**: List Zap run history

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zapier_get_run

**What it does**: Get Zap run details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zapier_get_me

**What it does**: Get user info

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://nla.zapier.com`
- Docs: https://nango.dev/docs/integrations/all/zapier-nla
