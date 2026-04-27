# Mailgun Tools

Provider: `mailgun` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Mailgun API. They allow AI agents to manage domains, send messages, and handle email deliverability features like bounces and spam complaints. Mailgun is a developer-focused email service for sending, receiving, and tracking emails.

## Authentication

**Nango API Key**:
- User provides Mailgun API Key
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `domains`, `messages`, `bounces`, `complaints`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mailgun_list_domains` | List all verified domains | GET | /v3/domains |
| `mailgun_get_domain` | Get domain details and credentials | GET | /v3/domains/{domain} |
| `mailgun_add_domain` | Add and verify a new domain | POST | /v3/domains |
| `mailgun_send_message` | Send an email message | POST | /v3/{domain}/messages |
| `mailgun_list_messages` | List stored messages | GET | /v3/domains/{domain}/messages |
| `mailgun_get_message` | Get a specific stored message | GET | /v3/messages/{url} |
| `mailgun_list_bounces` | List bounced email addresses | GET | /v3/{domain}/bounces |
| `mailgun_get_bounce` | Get bounce details for an address | GET | /v3/{domain}/bounces/{address} |
| `mailgun_list_complaints` | List spam complaint addresses | GET | /v3/{domain}/complaints |
| `mailgun_get_complaint` | Get complaint details for an address | GET | /v3/{domain}/complaints/{address} |

---

## Tool Details

### mailgun_list_domains

**What it does**: Lists all domains associated with your Mailgun account including verification status and settings.

**When to use**: See available sending domains, check which domains are verified and configured.

**Arguments**:
- `limit` (optional): Maximum number of domains to return (default 100)
- `skip` (optional): Number of domains to skip for pagination

**Example LLM prompt**: "List all domains configured in our Mailgun account"

---

### mailgun_get_domain

**What it does**: Gets detailed information about a specific domain including DNS records and verification status.

**When to use**: Check domain verification status, review DNS configuration, or get SMTP credentials.

**Arguments**:
- `domain` (required): Domain name (e.g., example.com)

**Example LLM prompt**: "Get details for domain mail.example.com"

---

### mailgun_add_domain

**What it does**: Adds a new domain to your Mailgun account. Returns DNS records required for verification.

**When to use**: Set up a new sending domain for your application or business.

**Arguments**:
- `name` (required): Domain name to add
- `smtp_password` (optional): Password for SMTP authentication
- `web_scheme` (optional): Scheme for click tracking: http or https (default https)

**Example LLM prompt**: "Add domain newsletter.example.com to our Mailgun account"

---

### mailgun_send_message

**What it does**: Sends an email message to one or more recipients with support for HTML, text, attachments, and custom headers.

**When to use**: Send transactional emails, notifications, or any application-generated messages.

**Arguments**:
- `domain` (required): Domain to send from
- `to` (required): Recipient email addresses
- `from` (required): Sender address (format: 'Name <email@example.com>')
- `subject` (required): Email subject
- `text` (optional): Plain text body
- `html` (optional): HTML body
- `attachment` (optional): File attachments
- `header` (optional): Custom headers as key-value pairs

**Example LLM prompt**: "Send an email from 'My App <noreply@example.com>' to 'user@example.com' with subject 'Welcome' and body 'Hello!'"

---

### mailgun_list_messages

**What it does**: Lists messages stored via Mailgun's store-and-forward service. Requires message storage to be enabled.

**When to use**: Retrieve copies of sent messages, review message content, or audit outgoing emails.

**Arguments**:
- `domain` (required): Domain name
- `limit` (optional): Maximum messages to return (default 100, max 10000)
- `skip` (optional): Number of messages to skip

**Example LLM prompt**: "List the last 50 stored messages for domain example.com"

---

### mailgun_get_message

**What it does**: Retrieves a specific stored message by its URL. Message URLs are returned when listing stored messages.

**When to use**: Read the full content of a previously sent message, check headers, or review attachments.

**Arguments**:
- `url` (required): Full URL of the stored message

**Example LLM prompt**: "Get the stored message at URL https://smtp.mailgun.org/..."

---

### mailgun_list_bounces

**What it does**: Lists all email addresses that have bounced. Bounces are failed deliveries due to invalid addresses or mail servers.

**When to use**: Review bounce records, clean up invalid addresses, or improve deliverability.

**Arguments**:
- `domain` (required): Domain name
- `limit` (optional): Maximum results (default 100)
- `skip` (optional): Number of results to skip

**Example LLM prompt**: "List all bounced addresses for domain example.com"

---

### mailgun_get_bounce

**What it does**: Gets bounce details for a specific email address including bounce type, code, and timestamp.

**When to use**: Check if a specific address has bounced, understand delivery failures.

**Arguments**:
- `domain` (required): Domain name
- `address` (required): Email address to check

**Example LLM prompt**: "Get bounce details for user@example.com"

---

### mailgun_list_complaints

**What it does**: Lists all email addresses that marked your messages as spam. Complaints are reported via Feedback Loops.

**When to use**: Review spam complaints, identify unhappy recipients, or improve list quality.

**Arguments**:
- `domain` (required): Domain name
- `limit` (optional): Maximum results (default 100)
- `skip` (optional): Number of results to skip

**Example LLM prompt**: "List all spam complaints for domain example.com"

---

### mailgun_get_complaint

**What it does**: Gets complaint details for a specific email address including when the complaint was filed.

**When to use**: Check if a specific user marked you as spam, investigate complaint sources.

**Arguments**:
- `domain` (required): Domain name
- `address` (required): Email address to check

**Example LLM prompt**: "Get complaint details for user@example.com"

---

## Mailgun API Notes

- **Domain Verification**: DNS records must be added and verified before sending
- **Message Storage**: Enable storage to retain copies of sent messages
- **Bounces**: Hard bounces (invalid addresses) should be removed from lists; soft bounces are temporary
- **Feedback Loops**: Mailgun integrates with major ESPs for spam complaint reporting
- **Rate Limits**: Limits depend on your Mailgun plan and domain reputation
- **Click Tracking**: Enable per-domain for open/click tracking
