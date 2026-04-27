# Resend Tools

Provider: `resend` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Resend is a modern email API platform for developers. These tools allow AI agents to send transactional emails, manage audiences and contacts, and administer sending domains.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Resend
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `emails:read`, `emails:write`, `audiences:read`, `audiences:write`, `contacts:read`, `contacts:write`, `audit:read`, `domains:read`, `domains:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `resend_list_emails` | List sent emails | GET | /emails |
| `resend_get_email` | Get email details | GET | /emails/{emailId} |
| `resend_send_email` | Send an email | POST | /emails |
| `resend_list_audiences` | List audiences | GET | /audiences |
| `resend_create_audience` | Create an audience | POST | /audiences |
| `resend_add_contact` | Add contact to audience | POST | /audiences/{audienceId}/contacts |
| `resend_list_contacts` | List audience contacts | GET | /audiences/{audienceId}/contacts |
| `resend_get_audit_log` | Get audit log | GET | /audit-logs |
| `resend_list_domains` | List verified domains | GET | /domains |
| `resend_create_domain` | Add a domain | POST | /domains |

---

## Tool Details

### resend_list_emails

**What it does**: Returns a list of all sent emails.

**When to use**: View email sending history.

**Arguments**:
- `limit` (optional): Number of emails (default 50)
- `from` (optional): Filter by sender

**Example LLM prompt**: "List recent emails sent through Resend"

---

### resend_get_email

**What it does**: Gets details of a specific email.

**When to use**: Check email delivery status.

**Arguments**:
- `emailId` (required): The email ID

**Example LLM prompt**: "Get status for email em_abc123"

---

### resend_send_email

**What it does**: Sends a new transactional email.

**When to use**: Send notifications, alerts, or personalized emails.

**Arguments**:
- `from` (required): Sender email (must be verified)
- `to` (required): Array of recipient emails
- `subject` (required): Email subject
- `html` (optional): HTML body
- `text` (optional): Plain text body

**Example LLM prompt**: "Send an email from no-reply@example.com to user@example.com with subject 'Welcome'"

---

### resend_list_audiences

**What it does**: Returns a list of all audiences.

**When to use**: View your contact lists for marketing.

**Arguments**:
- `limit` (optional): Number of audiences (default 50)

**Example LLM prompt**: "List all audiences"

---

### resend_create_audience

**What it does**: Creates a new audience.

**When to use**: Create a new mailing list.

**Arguments**:
- `name` (required): Audience name
- `description` (optional): Description

**Example LLM prompt**: "Create an audience called 'Newsletter Subscribers'"

---

### resend_add_contact

**What it does**: Adds a contact to an audience.

**When to use**: Subscribe users to your email list.

**Arguments**:
- `audienceId` (required): Audience ID
- `email` (required): Contact email
- `firstName` (optional): First name
- `lastName` (optional): Last name

**Example LLM prompt**: "Add user@example.com to audience aud_123"

---

### resend_list_contacts

**What it does**: Returns contacts in an audience.

**When to use**: View audience subscribers.

**Arguments**:
- `audienceId` (required): Audience ID
- `limit` (optional): Number of contacts (default 100)

**Example LLM prompt**: "List contacts in audience aud_123"

---

### resend_get_audit_log

**What it does**: Returns audit log entries.

**When to use**: Monitor account activity for security.

**Arguments**:
- `limit` (optional): Number of entries (default 50)

**Example LLM prompt**: "Get recent audit log entries"

---

### resend_list_domains

**What it does**: Returns all verified sending domains.

**When to use**: View your verified domains.

**Arguments**: None

**Example LLM prompt**: "List all verified domains"

---

### resend_create_domain

**What it does**: Adds and verifies a new domain.

**When to use**: Set up a new sending domain.

**Arguments**:
- `name` (required): Domain name
- `region` (optional): Region (us-east-1, eu-west-1)

**Example LLM prompt**: "Add domain mycompany.com"

---

## Resend Notes

- Emails can only be sent from verified domains
- Audiences are mailing lists for marketing campaigns
- Contacts can be added individually or in bulk
- Audit logs track account activity for compliance
