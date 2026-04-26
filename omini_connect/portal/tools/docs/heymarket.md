# Heymarket Tools

Provider: `heymarket` | Engine: `nango` | Auth: JWT via Nango

## Overview

These tools wrap the Heymarket API. They allow AI agents to send messages, manage campaigns, contacts, and templates. Heymarket is a business messaging platform for SMS and WhatsApp.

## Authentication

**Nango JWT**:
- User provides API Secret ID and Secret Key via Nango Connect
- Uses HMAC signature for authentication
- Token stored in Nango, accessed via `connection_ref`
- Signing key format: `{apiSecretId}||{apiSecretKey}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `heymarket_list_messages` | List messages | GET | /api/messages |
| `heymarket_send_message` | Send a message | POST | /api/messages |
| `heymarket_list_campaigns` | List campaigns | GET | /api/campaigns |
| `heymarket_get_campaign` | Get campaign details | GET | /api/campaigns/{id} |
| `heymarket_list_contacts` | List contacts | GET | /api/contacts |
| `heymarket_get_contact` | Get contact details | GET | /api/contacts/{id} |
| `heymarket_list_templates` | List templates | GET | /api/templates |
| `heymarket_get_template` | Get template details | GET | /api/templates/{id} |
| `heymarket_list_tags` | List tags | GET | /api/tags |
| `heymarket_list_conversations` | List conversations | GET | /api/conversations |

---

## Tool Details

### heymarket_list_messages

**What it does**: Lists all messages in Heymarket.

**When to use**: Browse message history.

**Arguments**:
- `conversation_id` (optional): Filter by conversation ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all messages"

---

### heymarket_send_message

**What it does**: Sends a message via Heymarket.

**When to use**: Send SMS or WhatsApp message.

**Arguments**:
- `recipient` (required): Recipient phone number
- `content` (required): Message content
- `channel` (optional): Channel (`sms`, `whatsapp`)

**Example LLM prompt**: "Send message to +1234567890 saying 'Hello'"

---

### heymarket_list_campaigns

**What it does**: Lists all campaigns in Heymarket.

**When to use**: Browse messaging campaigns.

**Arguments**: None

**Example LLM prompt**: "List all campaigns"

---

### heymarket_get_campaign

**What it does**: Gets detailed information about a specific campaign.

**When to use**: View campaign stats and messages.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get campaign with ID abc123"

---

### heymarket_list_contacts

**What it does**: Lists all contacts in Heymarket.

**When to use**: Browse contact list.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all contacts"

---

### heymarket_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: View contact details and history.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact with ID xyz789"

---

### heymarket_list_templates

**What it does**: Lists all message templates in Heymarket.

**When to use**: Browse message templates.

**Arguments**: None

**Example LLM prompt**: "List all templates"

---

### heymarket_get_template

**What it does**: Gets detailed information about a specific template.

**When to use**: View template content and variables.

**Arguments**:
- `id` (required): Template ID

**Example LLM prompt**: "Get template with ID tpl456"

---

### heymarket_list_tags

**What it does**: Lists all tags in Heymarket.

**When to use**: View available tags for organizing.

**Arguments**: None

**Example LLM prompt**: "List all tags"

---

### heymarket_list_conversations

**What it does**: Lists all conversations in Heymarket.

**When to use**: Browse active conversations.

**Arguments**: None

**Example LLM prompt**: "List all conversations"

---

## Heymarket API Notes

- **Auth Mode**: JWT with HMAC signature
- **Channels**: SMS and WhatsApp
- **Messages**: Individual message records
- **Campaigns**: Bulk messaging campaigns
- **Contacts**: Phone number contacts
- **Templates**: Pre-written message templates
- **Tags**: Labels for organizing contacts
- **Conversations**: Threaded message conversations
