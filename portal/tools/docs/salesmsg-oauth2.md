# Salesmsg-oauth2 Tools

Provider: `salesmsg-oauth2` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Salesmsg with OAuth2 authentication (different auth flow, same API). These tools allow AI agents to manage conversations, contacts, message templates, campaigns, and messaging statistics.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Salesmsg using OAuth2
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversations:read`, `conversations:write`, `messages:read`, `messages:write`, `contacts:read`, `contacts:write`, `templates:read`, `templates:write`, `campaigns:read`, `stats:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `salesmsg-oauth2_list_conversations` | List all conversations | GET | /v1/conversations |
| `salesmsg-oauth2_get_conversation` | Get conversation details | GET | /v1/conversations/{conversationId} |
| `salesmsg-oauth2_send_message` | Send a message | POST | /v1/messages |
| `salesmsg-oauth2_list_contacts` | List all contacts | GET | /v1/contacts |
| `salesmsg-oauth2_create_contact` | Create a contact | POST | /v1/contacts |
| `salesmsg-oauth2_list_templates` | List message templates | GET | /v1/templates |
| `salesmsg-oauth2_create_template` | Create a template | POST | /v1/templates |
| `salesmsg-oauth2_list_campaigns` | List campaigns | GET | /v1/campaigns |
| `salesmsg-oauth2_get_contact_messages` | Get contact message history | GET | /v1/contacts/{contactId}/messages |
| `salesmsg-oauth2_get_stats` | Get messaging stats | GET | /v1/stats |

---

## Tool Details

### salesmsg-oauth2_list_conversations

**What it does**: Returns a list of all conversations.

**When to use**: View ongoing message threads.

**Arguments**:
- `limit` (optional): Number of conversations (default 50)
- `status` (optional): Filter by status (open, closed)

**Example LLM prompt**: "List all open conversations"

---

### salesmsg-oauth2_get_conversation

**What it does**: Gets details of a specific conversation.

**When to use**: View conversation details and messages.

**Arguments**:
- `conversationId` (required): The conversation ID

**Example LLM prompt**: "Get details for conversation cvn_abc123"

---

### salesmsg-oauth2_send_message

**What it does**: Sends a message to a contact.

**When to use**: Send SMS to a customer.

**Arguments**:
- `contactId` (required): Contact ID
- `content` (required): Message content

**Example LLM prompt**: "Send message 'Thanks for your inquiry!' to contact cnt_456"

---

### salesmsg-oauth2_list_contacts

**What it does**: Returns a list of all contacts.

**When to use**: View contact database.

**Arguments**:
- `limit` (optional): Number of contacts (default 50)

**Example LLM prompt**: "List all contacts"

---

### salesmsg-oauth2_create_contact

**What it does**: Creates a new contact.

**When to use**: Add new leads to messaging.

**Arguments**:
- `name` (required): Contact name
- `phone` (required): Phone number
- `email` (optional): Email address

**Example LLM prompt**: "Create a contact for John Smith with phone 555-1234"

---

### salesmsg-oauth2_list_templates

**What it does**: Returns message templates.

**When to use**: View saved message templates.

**Arguments**:
- `limit` (optional): Number of templates (default 50)

**Example LLM prompt**: "List all message templates"

---

### salesmsg-oauth2_create_template

**What it does**: Creates a new message template.

**When to use**: Save commonly sent messages.

**Arguments**:
- `name` (required): Template name
- `content` (required): Template content

**Example LLM prompt**: "Create a template called 'Welcome' with content 'Welcome to our service!'"

---

### salesmsg-oauth2_list_campaigns

**What it does**: Returns a list of campaigns.

**When to use**: View SMS marketing campaigns.

**Arguments**:
- `limit` (optional): Number of campaigns (default 50)

**Example LLM prompt**: "List all campaigns"

---

### salesmsg-oauth2_get_contact_messages

**What it does**: Returns message history for a contact.

**When to use**: View conversation with a contact.

**Arguments**:
- `contactId` (required): The contact ID
- `limit` (optional): Number of messages (default 50)

**Example LLM prompt**: "Get message history for contact cnt_123"

---

### salesmsg-oauth2_get_stats

**What it does**: Returns messaging statistics.

**When to use**: Track messaging performance.

**Arguments**:
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Get messaging stats for this month"

---

## Salesmsg OAuth2 Notes

- Same API as standard Salesmsg - only auth differs
- OAuth2 provides secure token-based authentication
- All other functionality remains identical
