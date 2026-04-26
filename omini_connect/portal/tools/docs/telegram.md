# Telegram Tools

Provider: `telegram` | Engine: `nango` | Auth: API Key via Nango

## Overview

Telegram is a cloud-based messaging platform with bot API support. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Telegram API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `telegram_get_me` | Get information about the bot | GET | /getMe |
| `telegram_get_updates` | Get updates from the bot | GET | /getUpdates |
| `telegram_send_message` | Send a message to a chat | POST | /sendMessage |
| `telegram_send_photo` | Send a photo to a chat | POST | /sendPhoto |
| `telegram_send_document` | Send a document to a chat | POST | /sendDocument |
| `telegram_get_chat` | Get chat information | GET | /getChat |
| `telegram_list_chat_members` | Get chat member count | GET | /getChatMemberCount |
| `telegram_set_webhook` | Set webhook URL for updates | POST | /setWebhook |
| `telegram_delete_webhook` | Delete webhook URL | POST | /deleteWebhook |
| `telegram_get_file` | Get file information for download | GET | /getFile |

---

## Tool Details

### telegram_get_me

**What it does**: Get information about the bot

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### telegram_get_updates

**What it does**: Get updates from the bot

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### telegram_send_message

**What it does**: Send a message to a chat

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### telegram_send_photo

**What it does**: Send a photo to a chat

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### telegram_send_document

**What it does**: Send a document to a chat

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### telegram_get_chat

**What it does**: Get chat information

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### telegram_list_chat_members

**What it does**: Get chat member count

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### telegram_set_webhook

**What it does**: Set webhook URL for updates

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### telegram_delete_webhook

**What it does**: Delete webhook URL

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### telegram_get_file

**What it does**: Get file information for download

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.telegram.org/bot${apiKey}`
- Docs: https://nango.dev/docs/api-integrations/telegram
