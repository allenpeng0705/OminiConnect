# Postmark Tools

Provider: `postmark` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Postmark API. They allow AI agents to send transactional emails, manage email templates, track bounces, configure webhooks, and organize email tags. Postmark is a focused email delivery service built for developers who prioritize reliability and speed.

## Authentication

**Nango API Key**:
- User provides Postmark Server API Token
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `email`, `templates`, `bounces`, `webhooks`, `tags`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `postmark_send_email` | Send an email message | POST | /v3/email |
| `postmark_list_templates` | List all email templates | GET | /v3/templates |
| `postmark_get_template` | Get a specific template by ID | GET | /v3/templates/{template_id} |
| `postmark_create_template` | Create a new email template | POST | /v3/templates |
| `postmark_list_bounces` | List all bounces | GET | /v3/bounces |
| `postmark_get_bounce` | Get bounce details by ID | GET | /v3/bounces/{bounce_id} |
| `postmark_list_webhooks` | List all webhooks | GET | /v3/webhooks |
| `postmark_create_webhook` | Create a new webhook | POST | /v3/webhooks |
| `postmark_list_tags` | List all email tags | GET | /v3/tags |
| `postmark_get_tag` | Get tag statistics | GET | /v3/tags/{tag_name} |

---

## Tool Details

### postmark_send_email

**What it does**: Sends a transactional email message via Postmark. Supports HTML, plain text, tracking, and tagging.

**When to use**: Send order confirmations, password resets, notifications, or any transactional email from your application.

**Arguments**:
- `To` (required): Recipient email address
- `From` (required): Sender email address
- `Subject` (required): Email subject line
- `HtmlBody` (optional): HTML email body
- `TextBody` (optional): Plain text email body
- `Tag` (optional): Tag for tracking and filtering
- `TrackOpens` (optional): Enable open tracking

**Example LLM prompt**: "Send an email from noreply@company.com to user@example.com with subject 'Your order is confirmed' and HTML body '<h1>Thank you!</h1>'"

---

### postmark_list_templates

**What it does**: Retrieves all email templates in your Postmark account with pagination support.

**When to use**: Browse available templates, select a template for sending, or review template designs.

**Arguments**:
- `count` (optional): Number of templates to return
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all email templates in our Postmark account"

---

### postmark_get_template

**What it does**: Retrieves a specific email template by ID including its content and settings.

**When to use**: Review template content, check template variables, or prepare personalized emails.

**Arguments**:
- `template_id` (required): ID of the template to retrieve

**Example LLM prompt**: "Get details for template with ID 123456"

---

### postmark_create_template

**What it does**: Creates a new email template in Postmark with HTML and plain text versions.

**When to use**: Add new transactional or marketing email templates to your account.

**Arguments**:
- `Name` (required): Name of the template
- `Subject` (required): Template subject line
- `HtmlBody` (required): HTML body with handlebar tokens
- `TextBody` (optional): Plain text body

**Example LLM prompt**: "Create a template named 'Welcome Email' with subject 'Welcome to Our Service!' and HTML body containing handlebar tokens for the user's name"

---

### postmark_list_bounces

**What it does**: Retrieves all bounces (failed deliveries) for your server with filtering options.

**When to use**: Monitor deliverability issues, clean your email list, or investigate delivery failures.

**Arguments**:
- `count` (optional): Number of bounces to return
- `offset` (optional): Offset for pagination
- `type` (optional): Filter by bounce type
- `inactive` (optional): Filter by inactive status

**Example LLM prompt**: "List all bounces in our Postmark account from the last 30 days"

---

### postmark_get_bounce

**What it does**: Retrieves details about a specific bounce by its ID including the bounce type and timestamp.

**When to use**: Investigate a specific delivery failure or get details about why an email bounced.

**Arguments**:
- `bounce_id` (required): ID of the bounce to retrieve

**Example LLM prompt**: "Get details for bounce with ID 789"

---

### postmark_list_webhooks

**What it does**: Retrieves all webhooks configured for your Postmark server.

**When to use**: Review existing webhook configurations, check which events are being tracked.

**Arguments**:
- `server_id` (required): Server ID to list webhooks for

**Example LLM prompt**: "List all webhooks configured for our Postmark server"

---

### postmark_create_webhook

**What it does**: Creates a new webhook for tracking email events such as bounces, spam complaints, and clicks.

**When to use**: Set up event tracking for analytics, build email activity dashboards, or trigger custom workflows.

**Arguments**:
- `ServerId` (required): Server ID to attach webhook to
- `URL` (required): Webhook callback URL
- `Token` (optional): Secret token for webhook verification
- `Triggers` (optional): Event types to trigger on (bounce, spam, etc.)
- `BasicAuth` (optional): Basic authentication credentials

**Example LLM prompt**: "Create a webhook on server 1234 that posts to https://api.example.com/postmark-webhook for bounce and spam events"

---

### postmark_list_tags

**What it does**: Retrieves all tags used for email tracking in your Postmark account.

**When to use**: Review email tagging patterns, prepare tag-based analytics, or find specific campaign tags.

**Arguments**:
- `count` (optional): Number of tags to return
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all email tags we've used in Postmark"

---

### postmark_get_tag

**What it does**: Retrieves statistics and details for a specific tag including delivery metrics over a date range.

**When to use**: Analyze campaign performance, compare email tag effectiveness, or investigate specific email flows.

**Arguments**:
- `tag_name` (required): Name of the tag to retrieve
- `from_date` (optional): Start date for statistics (YYYY-MM-DD)
- `to_date` (optional): End date for statistics (YYYY-MM-DD)

**Example LLM prompt**: "Get statistics for the 'welcome-email' tag for the last 7 days"

---

## Postmark API Notes

- **Transactional Focus**: Postmark is designed for transactional emails (order confirmations, password resets) rather than marketing
- **Template Tokens**: Use handlebar tokens like `{{FirstName}}` for dynamic content in templates
- **Server-Based**: Postmark organizes resources by Server; ensure you're using the correct Server ID
- **Bounce Handling**: Postmark automatically suppresses bouncing addresses after multiple failures
- **Webhook Events**: Available events include bounce, spam complaint, click, open, and delivery
- **Tagging**: Use tags to categorize and track different types of emails in your analytics
