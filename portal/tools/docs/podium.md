# Podium Tools

Provider: `podium` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Podium API. They allow AI agents to manage leads, contacts, messages, reviews, and campaigns. Podium is a customer communication platform. **Requires Podium OAuth2 authentication.**

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Podium
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.podium.com
- Requires apiVersion in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `podium_list_leads` | List leads | GET | /api/leads |
| `podium_get_lead` | Get lead details | GET | /api/leads/{leadId} |
| `podium_list_contacts` | List contacts | GET | /api/contacts |
| `podium_get_contact` | Get contact details | GET | /api/contacts/{contactId} |
| `podium_list_messages` | List messages | GET | /api/messages |
| `podium_get_message` | Get message details | GET | /api/messages/{messageId} |
| `podium_list_reviews` | List reviews | GET | /api/reviews |
| `podium_get_review` | Get review details | GET | /api/reviews/{reviewId} |
| `podium_list_campaigns` | List campaigns | GET | /api/campaigns |
| `podium_get_location` | Get location info | GET | /api/locations/{locationId} |

---

## Tool Details

### podium_list_leads

**What it does**: Lists all leads in the organization.

**When to use**: Browse lead pipeline.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all new leads"

---

### podium_get_lead

**What it does**: Gets detailed information about a specific lead.

**When to use**: Get lead details, convert to customer.

**Arguments**:
- `leadId` (required): Lead ID

**Example LLM prompt**: "Get details for lead 12345"

---

### podium_list_contacts

**What it does**: Lists all contacts.

**When to use**: Browse contact directory.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all contacts"

---

### podium_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Get contact details, history.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Get details for contact 67890"

---

### podium_list_messages

**What it does**: Lists all messages.

**When to use**: Browse message history.

**Arguments**:
- `contactId` (optional): Filter by contact

**Example LLM prompt**: "List messages for contact 67890"

---

### podium_get_message

**What it does**: Gets detailed information about a specific message.

**When to use**: Get message content, status.

**Arguments**:
- `messageId` (required): Message ID

**Example LLM prompt**: "Get details for message 11111"

---

### podium_list_reviews

**What it does**: Lists all reviews.

**When to use**: Monitor reputation, respond to reviews.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending reviews"

---

### podium_get_review

**What it does**: Gets detailed information about a specific review.

**When to use**: Get review content, respond.

**Arguments**:
- `reviewId` (required): Review ID

**Example LLM prompt**: "Get details for review 22222"

---

### podium_list_campaigns

**What it does**: Lists all campaigns.

**When to use**: Browse marketing campaigns.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active campaigns"

---

### podium_get_location

**What it does**: Gets location information.

**When to use**: Get location settings.

**Arguments**:
- `locationId` (required): Location ID

**Example LLM prompt**: "Get our location information"

---

## Podium API Notes

- **OAuth2**: Requires user authentication via OAuth flow
- **Multi-location**: Supports multiple locations
- **Rate limits**: Apply to API calls
