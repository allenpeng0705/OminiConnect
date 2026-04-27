# Bird Tools

Provider: `bird` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Bird Composable API. They allow AI agents to send messages across multiple channels (SMS, email, WhatsApp, etc.), manage contacts, templates, and campaigns. Bird is a customer engagement platform for conversational messaging.

## Authentication

**Nango API_KEY**:
- User provides Bird access key
- Token stored in Nango, accessed via `connection_ref`
- Requires subdomain and workspace ID configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bird_list_messages` | List messages | GET | /v1/messages |
| `bird_get_message` | Get message details | GET | /v1/messages/{messageId} |
| `bird_send_message` | Send a message | POST | /v1/messages |
| `bird_list_templates` | List message templates | GET | /v1/templates |
| `bird_create_template` | Create message template | POST | /v1/templates |
| `bird_list_contacts` | List contacts | GET | /v1/contacts |
| `bird_get_contact` | Get contact details | GET | /v1/contacts/{contactId} |
| `bird_list_campaigns` | List campaigns | GET | /v1/campaigns |
| `bird_get_campaign` | Get campaign details | GET | /v1/campaigns/{campaignId} |
| `bird_get_webhooks` | Get webhook events | GET | /v1/webhooks |

---

## Tool Details

### bird_list_messages

**What it does**: Lists all messages sent through Bird.

**When to use**: View message history, track delivery.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Messages per page (default 20)

**Example LLM prompt**: "List recent messages"

---

### bird_get_message

**What it does**: Gets details for a specific message.

**When to use**: Check message status, delivery details.

**Arguments**:
- `messageId` (required): Message ID

**Example LLM prompt**: "Get details for message M-123"

---

### bird_send_message

**What it does**: Sends a message through Bird.

**When to use**: Send notifications, alerts, or marketing messages.

**Arguments**:
- `channel` (required): Channel (sms, email, whatsapp, etc.)
- `recipient` (required): Recipient identifier
- `content` (required): Message content

**Example LLM prompt**: "Send SMS 'Hello World' to +1234567890"

---

### bird_list_templates

**What it does**: Lists all message templates.

**When to use**: Find existing templates, manage message content.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Templates per page (default 20)

**Example LLM prompt**: "List all templates"

---

### bird_create_template

**What it does**: Creates a new message template.

**When to use**: Create reusable message content.

**Arguments**:
- `name` (required): Template name
- `channel` (required): Channel (sms, email, etc.)
- `content` (required): Template content

**Example LLM prompt**: "Create template 'welcome' for SMS"

---

### bird_list_contacts

**What it does**: Lists all contacts in Bird.

**When to use**: View contact database, find recipients.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Contacts per page (default 20)

**Example LLM prompt**: "List all contacts"

---

### bird_get_contact

**What it does**: Gets details for a specific contact.

**When to use**: View contact info, check preferences.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Get details for contact C-456"

---

### bird_list_campaigns

**What it does**: Lists all campaigns in Bird.

**When to use**: View marketing campaigns, track performance.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Campaigns per page (default 20)

**Example LLM prompt**: "List all campaigns"

---

### bird_get_campaign

**What it does**: Gets details for a specific campaign.

**When to use**: Check campaign metrics, delivery status.

**Arguments**:
- `campaignId` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign CAM-789"

---

### bird_get_webhooks

**What it does**: Gets webhook events and logs.

**When to use**: Debug webhooks, check event history.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Events per page (default 20)

**Example LLM prompt**: "Get recent webhook events"

---

## Bird API Notes

- **Channels**: Supports SMS, email, WhatsApp, Voice, and more
- **Templates**: Pre-approved message formats for regulated channels
- **Workspace**: Messages scoped to a specific workspace
- **Webhooks**: Real-time event notifications for message status
- **Rate Limits**: Vary by channel and plan
