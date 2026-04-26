# SendGrid Tools

Provider: `sendgrid` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the SendGrid API. They allow AI agents to send transactional emails, manage contacts, organize contact lists, handle templates, and create marketing campaigns. SendGrid is a leading email delivery platform for developers and marketers.

## Authentication

**Nango API Key**:
- User provides SendGrid API Key
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `mail.send`, `marketing.contacts`, `marketing.lists`, `templates`, `marketing.campaigns`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sendgrid_send_email` | Send a transactional email | POST | /mail/send |
| `sendgrid_list_contacts` | List all contacts | GET | /marketing/contacts |
| `sendgrid_get_contact` | Get a specific contact by email | GET | /marketing/contacts/{email} |
| `sendgrid_add_contact` | Add a new contact | PUT | /marketing/contacts |
| `sendgrid_list_lists` | List all contact lists | GET | /marketing/lists |
| `sendgrid_get_list` | Get a specific list by ID | GET | /marketing/lists/{list_id} |
| `sendgrid_add_contact_to_list` | Add a contact to a list | POST | /marketing/lists/{list_id}/contacts |
| `sendgrid_list_templates` | List email templates | GET | /templates |
| `sendgrid_get_template` | Get a specific template by ID | GET | /templates/{template_id} |
| `sendgrid_create_campaign` | Create a marketing campaign | POST | /marketing/campaigns |

---

## Tool Details

### sendgrid_send_email

**What it does**: Sends a transactional email to one or more recipients. Supports HTML content, attachments, and custom headers.

**When to use**: Send order confirmations, password resets, notifications, or any automated email from your application.

**Arguments**:
- `to` (required): Array of recipient objects with email and name
- `from` (required): Sender object with email and name
- `subject` (required): Email subject line
- `text` (optional): Plain text body content
- `html` (optional): HTML body content
- `reply_to` (optional): Reply-to address object

**Example LLM prompt**: "Send an email from noreply@company.com to user@example.com with subject 'Welcome' and body 'Hello, welcome to our service!'"

---

### sendgrid_list_contacts

**What it does**: Retrieves all contacts in your SendGrid marketing lists with pagination.

**When to use**: Browse your contact database, find subscribers, or prepare bulk operations.

**Arguments**:
- `page_size` (optional): Number of results per page (max 100, default 100)
- `page_token` (optional): Pagination token from previous response

**Example LLM prompt**: "List the first 50 contacts in our SendGrid account"

---

### sendgrid_get_contact

**What it does**: Retrieves a specific contact by their email address, including all stored information and metadata.

**When to use**: Look up individual contact details, verify a subscriber exists, or check contact information.

**Arguments**:
- `email` (required): Contact email address (URL encoded)

**Example LLM prompt**: "Get details for contact john@example.com"

---

### sendgrid_add_contact

**What it does**: Adds a new contact to your SendGrid marketing lists with custom field data.

**When to use**: Add new subscribers, capture signups, or import contacts from other systems.

**Arguments**:
- `email` (required): Contact email address
- `first_name` (optional): Contact first name
- `last_name` (optional): Contact last name
- `custom_fields` (optional): Custom field key-value pairs

**Example LLM prompt**: "Add a new contact with email sarah@example.com, first name Sarah, and last name Smith"

---

### sendgrid_list_lists

**What it does**: Retrieves all contact lists in your SendGrid account. Lists organize contacts for targeted campaigns.

**When to use**: See available lists, check audience size, or prepare campaigns.

**Arguments**:
- `page_size` (optional): Number of results per page (max 100, default 100)
- `page_token` (optional): Pagination token from previous response

**Example LLM prompt**: "List all our contact lists in SendGrid"

---

### sendgrid_get_list

**What it does**: Gets details of a specific contact list including member count and settings.

**When to use**: Check list details before adding contacts or sending campaigns.

**Arguments**:
- `list_id` (required): List ID

**Example LLM prompt**: "Get details for list with ID abc123"

---

### sendgrid_add_contact_to_list

**What it does**: Adds an existing contact to a contact list by their email address.

**When to use**: Subscribe contacts to specific lists, organize contacts by interest or segment.

**Arguments**:
- `list_id` (required): List ID to add contact to
- `emails` (required): Array of contact emails to add

**Example LLM prompt**: "Add contact john@example.com to list abc123"

---

### sendgrid_list_templates

**What it does**: Retrieves all email templates in your SendGrid account. Templates can be dynamic with Handlebars syntax.

**When to use**: Browse available templates, select a template for a campaign, or review template designs.

**Arguments**:
- `generation` (optional): Filter by generation: `dynamic` or `raw`

**Example LLM prompt**: "List all dynamic email templates"

---

### sendgrid_get_template

**What it does**: Gets a specific email template by ID including all versions and settings for rendering.

**When to use**: Review template content, check template variables, or prepare personalized campaigns.

**Arguments**:
- `template_id` (required): Template ID

**Example LLM prompt**: "Get details for template abc123"

---

### sendgrid_create_campaign

**What it does**: Creates a new marketing email campaign with recipients, subject, content, and send settings.

**When to use**: Set up newsletters, promotional campaigns, or automated email sequences.

**Arguments**:
- `name` (required): Campaign name
- `send_at` (optional): ISO 8601 date to schedule the campaign
- `subject` (optional): Email subject line
- `sender_id` (optional): Sender identity ID
- `list_ids` (optional): Array of contact list IDs to send to
- `template_id` (optional): Template ID to use

**Example LLM prompt**: "Create a campaign called 'Spring Sale' using template abc123 to list xyz789"

---

## SendGrid API Notes

- **Transactional vs Marketing**: Use `send_email` for transactional messages, campaigns for marketing emails
- **Contact Database**: Contacts are stored in the unified marketing contacts database
- **Lists vs Segments**: Lists are static collections; SendGrid also supports segments (dynamic groups)
- **Templates**: Dynamic templates support personalization with Handlebars syntax
- **Sender Identity**: Verify sender identities in your SendGrid settings before sending
- **Rate Limits**: Rate limits apply based on your SendGrid plan tier
