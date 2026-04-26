# WhatsApp Business Tools

Provider: `whatsapp-business` | Engine: `nango` | Auth: API Key via Nango

## Overview

WhatsApp Business API for managing business messaging. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their WhatsApp Business API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `whatsapp_list_message_templates` | List all message templates | GET | /v18.0/{phone_number_id}/message_templates |
| `whatsapp_create_message_template` | Create a new message template | POST | /v18.0/{phone_number_id}/message_templates |
| `whatsapp_send_message` | Send a WhatsApp message | POST | /v18.0/{phone_number_id}/messages |
| `whatsapp_list_messages` | List sent messages | GET | /v18.0/{phone_number_id}/messages |
| `whatsapp_get_message_status` | Get status of a sent message | GET | /v18.0/{message_id} |
| `whatsapp_create_template_message` | Send a template message | POST | /v18.0/{phone_number_id}/messages |
| `whatsapp_list_phonenumbers` | List registered phone numbers | GET | /v18.0/{business_id}/phone_numbers |
| `whatsapp_get_business_profile` | Get business profile info | GET | /v18.0/{phone_number_id} |
| `whatsapp_update_business_profile` | Update business profile | POST | /v18.0/{phone_number_id} |
| `whatsapp_list_media` | List media files | GET | /v18.0/{phone_number_id}/media |

---

## Tool Details

### whatsapp_list_message_templates

**What it does**: List all message templates

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whatsapp_create_message_template

**What it does**: Create a new message template

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whatsapp_send_message

**What it does**: Send a WhatsApp message

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whatsapp_list_messages

**What it does**: List sent messages

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whatsapp_get_message_status

**What it does**: Get status of a sent message

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whatsapp_create_template_message

**What it does**: Send a template message

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whatsapp_list_phonenumbers

**What it does**: List registered phone numbers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whatsapp_get_business_profile

**What it does**: Get business profile info

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whatsapp_update_business_profile

**What it does**: Update business profile

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whatsapp_list_media

**What it does**: List media files

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://graph.facebook.com`
- Docs: https://nango.dev/docs/integrations/all/whatsapp-business
