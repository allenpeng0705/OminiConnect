# ClickSend Tools

Provider: `clicksend` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the ClickSend API. ClickSend is a communication platform for sending SMS and email campaigns. **Requires ClickSend credentials (username and API key).**

## Authentication

**Nango BASIC**:
- User provides their ClickSend username and API key
- Credentials passed via HTTP Basic Authentication
- Base URL: `https://rest.clicksend.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `clicksend_list_contacts` | List contacts | GET | /v3/contact |
| `clicksend_get_contact` | Get contact details | GET | /v3/contact/{id} |
| `clicksend_create_contact` | Create a contact | POST | /v3/contact |
| `clicksend_list_sms_messages` | List SMS messages | GET | /v3/sms |
| `clicksend_send_sms` | Send an SMS | POST | /v3/sms/send |
| `clicksend_list_sms_campaigns` | List SMS campaigns | GET | /v3/sms-campaign |
| `clicksend_get_sms_campaign` | Get SMS campaign details | GET | /v3/sms-campaign/{id} |
| `clicksend_list_email_campaigns` | List email campaigns | GET | /v3/email-campaign |
| `clicksend_get_email_campaign` | Get email campaign details | GET | /v3/email-campaign/{id} |
| `clicksend_get_account` | Get account info | GET | /v3/account |

---

## Tool Details

### clicksend_list_contacts

**What it does**: Lists all contacts in your account.

**When to use**: Find contacts for messaging.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all ClickSend contacts"

---

### clicksend_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: View contact information.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact 123 details"

---

### clicksend_create_contact

**What it does**: Creates a new contact.

**When to use**: Add a new contact to your list.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number

**Example LLM prompt**: "Create a contact John Doe with phone +1234567890"

---

### clicksend_list_sms_messages

**What it does**: Lists all SMS messages.

**When to use**: View SMS history.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List recent SMS messages"

---

### clicksend_send_sms

**What it does**: Sends an SMS message.

**When to use**: Send a text message to a recipient.

**Arguments**:
- `to` (required): Recipient phone number
- `body` (required): SMS message body

**Example LLM prompt**: "Send an SMS to +1234567890 saying 'Hello World'"

---

### clicksend_list_sms_campaigns

**What it does**: Lists all SMS campaigns.

**When to use**: View campaign history.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all SMS campaigns"

---

### clicksend_get_sms_campaign

**What it does**: Gets details of a specific SMS campaign.

**When to use**: View campaign status and statistics.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get SMS campaign 123 details"

---

### clicksend_list_email_campaigns

**What it does**: Lists all email campaigns.

**When to use**: View email campaign history.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all email campaigns"

---

### clicksend_get_email_campaign

**What it does**: Gets details of a specific email campaign.

**When to use**: View campaign statistics.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get email campaign 456 details"

---

### clicksend_get_account

**What it does**: Gets account information.

**When to use**: View account balance and settings.

**Arguments**: None

**Example LLM prompt**: "Get my ClickSend account info"

---

## ClickSend API Notes

- **BASIC Auth**: Username and API key via HTTP Basic Auth
- **SMS**: Text messaging to mobile numbers
- **Email Campaigns**: Bulk email marketing
- **Contacts**: Recipient address book
