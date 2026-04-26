# Brevo (API Key) Tools

Provider: `brevo-api-key` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Brevo API. They allow AI agents to manage contacts, send email and SMS campaigns, and automate marketing workflows. Brevo (formerly Sendinblue) is an all-in-one marketing platform for email, SMS, and more.

## Authentication

**Nango API_KEY**:
- User provides Brevo API key
- Token stored in Nango, accessed via `connection_ref`
- API key passed in api-key header

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `brevo_list_contacts` | List contacts | GET | /contacts |
| `brevo_get_contact` | Get contact details | GET | /contacts/{identifier} |
| `brevo_create_contact` | Create contact | POST | /contacts |
| `brevo_update_contact` | Update contact | PUT | /contacts/{identifier} |
| `brevo_delete_contact` | Delete contact | DELETE | /contacts/{identifier} |
| `brevo_list_campaigns` | List email campaigns | GET | /emailCampaigns |
| `brevo_get_campaign` | Get campaign details | GET | /emailCampaigns/{campaignId} |
| `brevo_send_campaign` | Send campaign | POST | /emailCampaigns/{campaignId}/sendNow |
| `brevo_list_sms_campaigns` | List SMS campaigns | GET | /smsCampaigns |
| `brevo_get_sms_campaign` | Get SMS campaign details | GET | /smsCampaigns/{campaignId} |

---

## Tool Details

### brevo_list_contacts

**What it does**: Lists all contacts in Brevo.

**When to use**: Browse contact database, find recipients.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Contacts per page (default 20)

**Example LLM prompt**: "List all contacts in Brevo"

---

### brevo_get_contact

**What it does**: Gets details for a specific contact.

**When to use**: View contact info, attributes, history.

**Arguments**:
- `identifier` (required): Contact email or ID

**Example LLM prompt**: "Get details for contact john@example.com"

---

### brevo_create_contact

**What it does**: Creates a new contact in Brevo.

**When to use**: Add new subscribers, import contacts.

**Arguments**:
- `email` (required): Contact email
- `firstName` (optional): First name
- `lastName` (optional): Last name
- `attributes` (optional): Custom attributes

**Example LLM prompt**: "Create contact for John Doe with email john@example.com"

---

### brevo_update_contact

**What it does**: Updates an existing contact.

**When to use**: Modify contact attributes, update info.

**Arguments**:
- `identifier` (required): Contact email or ID
- `attributes` (optional): Attributes to update

**Example LLM prompt**: "Update contact john@example.com with new phone number"

---

### brevo_delete_contact

**What it does**: Deletes a contact from Brevo.

**When to use**: Remove outdated contacts, handle unsubscribes.

**Arguments**:
- `identifier` (required): Contact email or ID

**Example LLM prompt**: "Delete contact john@example.com"

---

### brevo_list_campaigns

**What it does**: Lists all email campaigns.

**When to use**: View campaign history, check status.

**Arguments**:
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all sent campaigns"

---

### brevo_get_campaign

**What it does**: Gets details for a specific campaign.

**When to use**: Check campaign stats, view content.

**Arguments**:
- `campaignId` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign 123"

---

### brevo_send_campaign

**What it does**: Sends an email campaign immediately.

**When to use**: Send transactional or marketing emails.

**Arguments**:
- `campaignId` (required): Campaign ID

**Example LLM prompt**: "Send campaign 123 now"

---

### brevo_list_sms_campaigns

**What it does**: Lists all SMS campaigns.

**When to use**: View SMS campaign history.

**Arguments**:
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all SMS campaigns"

---

### brevo_get_sms_campaign

**What it does**: Gets details for a specific SMS campaign.

**When to use**: Check SMS campaign stats.

**Arguments**:
- `campaignId` (required): Campaign ID

**Example LLM prompt**: "Get details for SMS campaign 456"

---

## Brevo API Notes

- **Contacts**: Can be identified by email or internal ID
- **Attributes**: Custom fields for storing additional data
- **Campaigns**: Email and SMS marketing campaigns
- **Lists**: Contacts can be organized into lists
- **API Key**: Found in Brevo account settings
