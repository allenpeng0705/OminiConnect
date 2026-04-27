# Unipile Tools

Provider: `unipile` | Engine: `nango` | Auth: API Key via Nango

## Overview

Unipile provides unified APIs for professional communication platforms. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Unipile API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `unipile_list_messages` | List messages from channels | GET | /api/v1/messages |
| `unipile_send_message` | Send a message | POST | /api/v1/messages |
| `unipile_list_threads` | List message threads | GET | /api/v1/threads |
| `unipile_get_thread` | Get thread details | GET | /api/v1/threads/{id} |
| `unipile_list_contacts` | List contacts | GET | /api/v1/contacts |
| `unipile_get_contact` | Get contact details | GET | /api/v1/contacts/{id} |
| `unipile_list_channels` | List available channels | GET | /api/v1/channels |
| `unipile_get_channel` | Get channel details | GET | /api/v1/channels/{id} |
| `unipile_list_accounts` | List connected accounts | GET | /api/v1/accounts |
| `unipile_webhook_test` | Test webhook configuration | POST | /api/v1/webhooks/test |

---

## Tool Details

### unipile_list_messages

**What it does**: List messages from channels

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unipile_send_message

**What it does**: Send a message

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unipile_list_threads

**What it does**: List message threads

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unipile_get_thread

**What it does**: Get thread details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unipile_list_contacts

**What it does**: List contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unipile_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unipile_list_channels

**What it does**: List available channels

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unipile_get_channel

**What it does**: Get channel details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unipile_list_accounts

**What it does**: List connected accounts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unipile_webhook_test

**What it does**: Test webhook configuration

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.subdomain}.unipile.com:${connectionConfig.port}`
- Docs: https://nango.dev/docs/integrations/all/unipile
