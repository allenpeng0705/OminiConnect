# Wildix PBX Tools

Provider: `wildix-pbx` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Wildix is a cloud PBX and communications platform. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Wildix PBX
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wildix_list_extensions` | List all extensions | GET | /api/v1/extensions |
| `wildix_get_extension` | Get extension details | GET | /api/v1/extensions/{id} |
| `wildix_list_calls` | List all calls | GET | /api/v1/calls |
| `wildix_make_call` | Initiate a call | POST | /api/v1/calls |
| `wildix_transfer_call` | Transfer a call | POST | /api/v1/calls/{id}/transfer |
| `wildix_hangup_call` | Hangup a call | POST | /api/v1/calls/{id}/hangup |
| `wildix_list_voicemails` | List all voicemails | GET | /api/v1/voicemails |
| `wildix_get_voicemail` | Get voicemail details | GET | /api/v1/voicemails/{id} |
| `wildix_list_conferences` | List all conferences | GET | /api/v1/conferences |
| `wildix_get_presence` | Get user presence status | GET | /api/v1/presence |

---

## Tool Details

### wildix_list_extensions

**What it does**: List all extensions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wildix_get_extension

**What it does**: Get extension details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wildix_list_calls

**What it does**: List all calls

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wildix_make_call

**What it does**: Initiate a call

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wildix_transfer_call

**What it does**: Transfer a call

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wildix_hangup_call

**What it does**: Hangup a call

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wildix_list_voicemails

**What it does**: List all voicemails

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wildix_get_voicemail

**What it does**: Get voicemail details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wildix_list_conferences

**What it does**: List all conferences

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wildix_get_presence

**What it does**: Get user presence status

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.subdomain}.wildixin.com`
- Docs: https://nango.dev/docs/integrations/all/wildix-pbx
