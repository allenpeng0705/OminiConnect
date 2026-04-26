# SparkPost Tools

Provider: `sparkpost` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the SparkPost API. They allow AI agents to send email messages, manage templates, configure webhooks for events, retrieve transmission metrics, and manage sending domains. SparkPost is an email delivery platform known for its flexibility and detailed analytics.

## Authentication

**Nango API Key**:
- User provides SparkPost API Key
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `transmissions`, `templates`, `webhooks`, `metrics`, `sending-domains`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sparkpost_list_messages` | List all transmission messages | GET | /v1/transmissions |
| `sparkpost_get_message` | Get message details by ID | GET | /v1/transmissions/{id} |
| `sparkpost_send_message` | Send an email message | POST | /v1/transmissions |
| `sparkpost_list_templates` | List all email templates | GET | /v1/templates |
| `sparkpost_get_template` | Get a specific template by ID | GET | /v1/templates/{id} |
| `sparkpost_create_template` | Create a new email template | POST | /v1/templates |
| `sparkpost_list_webhooks` | List all webhooks | GET | /v1/webhooks |
| `sparkpost_create_webhook` | Create a new webhook | POST | /v1/webhooks |
| `sparkpost_get_metrics` | Retrieve aggregate metrics | GET | /v1/metrics |
| `sparkpost_list_domains` | List all sending domains | GET | /v1/sending-domains |

---

## Tool Details

### sparkpost_list_messages

**What it does**: Retrieves all transmission messages from your SparkPost account with pagination and filtering.

**When to use**: Browse your sending history, find specific transmissions, or prepare transmission reports.

**Arguments**:
- `limit` (optional): Maximum number of transmissions to return
- `offset` (optional): Offset for pagination
- `campaign_id` (optional): Filter by campaign ID

**Example LLM prompt**: "List the last 50 transmissions in our SparkPost account"

---

### sparkpost_get_message

**What it does**: Retrieves details of a specific transmission message by its ID including recipient info and status.

**When to use**: Check the status of a specific email transmission or retrieve details about a sent message.

**Arguments**:
- `id` (required): Transmission ID to retrieve

**Example LLM prompt**: "Get details for transmission ID abc123"

---

### sparkpost_send_message

**What it does**: Sends an email message via SparkPost transmission API with support for HTML, plain text, and metadata.

**When to use**: Send transactional emails, marketing campaigns, or automated notifications.

**Arguments**:
- `recipients` (required): List of recipient email addresses
- `from` (required): Sender email address
- `subject` (required): Email subject line
- `html` (optional): HTML email body
- `text` (optional): Plain text email body
- `campaign_id` (optional): Campaign ID for tracking
- `metadata` (optional): Custom metadata key-value pairs

**Example LLM prompt**: "Send an email from sender@company.com to recipient@example.com with subject 'Your monthly report' and HTML body containing the report"

---

### sparkpost_list_templates

**What it does**: Retrieves all email templates from your SparkPost account with pagination.

**When to use**: Browse available templates, select a template for a transmission, or review template designs.

**Arguments**:
- `limit` (optional): Maximum number of templates to return
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all email templates in our SparkPost account"

---

### sparkpost_get_template

**What it does**: Retrieves a specific email template by ID including its content and configuration.

**When to use**: Review template content, check template variables, or prepare personalized transmissions.

**Arguments**:
- `id` (required): Template ID to retrieve

**Example LLM prompt**: "Get details for template with ID welcome-email"

---

### sparkpost_create_template

**What it does**: Creates a new email template in SparkPost with support for handlebar syntax in content.

**When to use**: Add new transactional or marketing email templates to your account.

**Arguments**:
- `id` (required): Unique ID for the template
- `name` (required): Template name
- `html` (required): HTML body with handlebar syntax
- `description` (optional): Template description
- `text` (optional): Plain text body
- `subject` (optional): Default subject line

**Example LLM prompt**: "Create a template with ID 'order-confirmation', name 'Order Confirmation', and HTML body with handlebar tokens for order details"

---

### sparkpost_list_webhooks

**What it does**: Retrieves all configured webhooks for event streaming in your SparkPost account.

**When to use**: Review existing webhook configurations or check which events are being tracked.

**Arguments**:
- `limit` (optional): Maximum number of webhooks to return

**Example LLM prompt**: "List all webhooks configured in our SparkPost account"

---

### sparkpost_create_webhook

**What it does**: Creates a new webhook for receiving SparkPost event notifications at your endpoint.

**When to use**: Set up event tracking for analytics, build email dashboards, or trigger custom workflows based on email events.

**Arguments**:
- `name` (required): Webhook name
- `target` (required): URL to receive webhook events
- `events` (required): List of event types to subscribe to
- `auth_token` (optional): Bearer token for webhook verification

**Example LLM prompt**: "Create a webhook named 'email-analytics' that posts to https://api.example.com/sparkpost-webhook for delivery and bounce events"

---

### sparkpost_get_metrics

**What it does**: Retrieves aggregate metrics for your SparkPost transmissions including delivery and engagement data.

**When to use**: Analyze email performance, prepare reports on delivery rates, or monitor engagement metrics.

**Arguments**:
- `metrics` (optional): List of metric types to retrieve
- `from` (optional): Start date (YYYY-MM-DD)
- `to` (optional): End date (YYYY-MM-DD)
- `timezone` (optional): Timezone for results (e.g., America/New_York)

**Example LLM prompt**: "Get email metrics for the last 30 days including open and click rates"

---

### sparkpost_list_domains

**What it does**: Retrieves all verified sending domains in your SparkPost account.

**When to use**: Review your sending domains, check domain verification status, or prepare domain-based sending reports.

**Arguments**:
- `limit` (optional): Maximum number of domains to return
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all sending domains configured in our SparkPost account"

---

## SparkPost API Notes

- **Transmission-Based**: SparkPost uses "transmissions" as the core concept for sent messages
- **Template Support**: Templates use handlebar syntax for dynamic content (e.g., `{{name}}`)
- **Campaign Tracking**: Use campaign_id to group and track related transmissions
- **Webhook Events**: Available events include delivery, bounce, complaint, open, click, and more
- **Metrics**: SparkPost provides detailed engagement metrics including opens, clicks, and unsubscribes
- **Sending Domains**: Verify domain ownership before sending to improve deliverability
- **IP Pools**: SparkPost supports IP pool management for dedicated sending infrastructure
