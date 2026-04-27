# MessageBird Tools

Provider: `messagebird` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the MessageBird API. They allow AI agents to send SMS messages, manage omnichannel conversations, handle webhooks, and check account balance. MessageBird is a leading omnichannel communications platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with MessageBird
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `messagebird_list_messages` | List messages | GET | /messages |
| `messagebird_get_message` | Get message details | GET | /messages/{messageId} |
| `messagebird_send_message` | Send message | POST | /messages |
| `messagebird_send_sms` | Send SMS | POST | /messages |
| `messagebird_list_conversations` | List conversations | GET | /conversations |
| `messagebird_get_conversation` | Get conversation details | GET | /conversations/{conversationId} |
| `messagebird_create_conversation` | Create conversation | POST | /conversations |
| `messagebird_get_balance` | Get account balance | GET | /balance |
| `messagebird_list_webhooks` | List webhooks | GET | /webhooks |
| `messagebird_create_webhook` | Create webhook | POST | /webhooks |

---

## Tool Details

### messagebird_list_messages

**What it does**: Lists all SMS messages sent or received through MessageBird.

**When to use**: Review message history, check delivery status, find messages by time range.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List the last 50 messages"

---

### messagebird_get_message

**What it does**: Gets detailed information about a specific MessageBird message.

**When to use**: Check message status, view delivery details, get message metadata.

**Arguments**:
- `messageId` (required): Message ID

**Example LLM prompt**: "Get details for message abc123"

---

### messagebird_send_message

**What it does**: Sends a message via MessageBird.

**When to use**: Send notifications, alerts, or promotional messages to customers.

**Arguments**:
- `recipients` (required): Array of recipient phone numbers
- `body` (required): Message text
- `originator` (optional): Sender name or phone number
- `reference` (optional): Custom reference string for tracking

**Example LLM prompt**: "Send an SMS to +1234567890 with text 'Hello, your code is 1234'"

---

### messagebird_send_sms

**What it does**: Sends an SMS to a single recipient via MessageBird.

**When to use**: Send quick SMS notifications to a single contact.

**Arguments**:
- `recipient` (required): Recipient phone number
- `message` (required): Message text
- `originator` (optional): Sender name or phone number

**Example LLM prompt**: "Send 'Hello!' to +1234567890 from MyApp"

---

### messagebird_list_conversations

**What it does**: Lists all conversations in MessageBird's omnichannel inbox.

**When to use**: View customer conversations across SMS, WhatsApp, and other channels.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all active conversations"

---

### messagebird_get_conversation

**What it does**: Gets detailed information about a specific conversation including all messages.

**When to use**: Review conversation history, understand context before responding.

**Arguments**:
- `conversationId` (required): Conversation ID

**Example LLM prompt**: "Get details for conversation abc123"

---

### messagebird_create_conversation

**What it does**: Creates a new conversation or sends an initial message.

**When to use**: Start a new customer conversation, initiate outreach.

**Arguments**:
- `recipient` (required): Recipient contact identifier
- `channel` (required): Messaging channel (sms, whatsapp, etc.)
- `message` (optional): Initial message content

**Example LLM prompt**: "Create a WhatsApp conversation with +1234567890"

---

### messagebird_get_balance

**What it does**: Gets your MessageBird account balance and currency.

**When to use**: Check account credits before sending messages.

**Arguments**: None

**Example LLM prompt**: "What's my current MessageBird balance?"

---

### messagebird_list_webhooks

**What it does**: Lists all webhooks configured in MessageBird.

**When to use**: Review webhook configurations, check subscribed events.

**Arguments**:
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List all webhooks"

---

### messagebird_create_webhook

**What it does**: Creates a new webhook to receive event notifications.

**When to use**: Set up integrations to receive SMS, delivery, and conversation events.

**Arguments**:
- `url` (required): Webhook URL
- `token` (required): Secret token for webhook verification
- `channel` (optional): Channel for the webhook

**Example LLM prompt**: "Create a webhook for message events at https://myapp.com/webhook"

---

## MessageBird API Notes

- **Phone Numbers**: Must be in E.164 format (e.g., +1234567890)
- **Conversation API**: Supports SMS, WhatsApp, and other channels
- **Webhooks**: Receive events for incoming messages, delivery reports, conversation updates
- **Originator**: Can be a phone number or alphanumeric sender name (where supported)
- **Reference Field**: Custom reference strings help track messages in your own system
