# Typeform Tools

Provider: `typeform` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Typeform API. They allow AI agents to manage forms, responses, fields, and webhooks. Typeform is a popular platform for creating interactive forms and surveys.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Typeform
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read_forms`, `write_forms`, `read_responses`, `read_workspaces`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `typeform_list_forms` | List forms | GET | /forms |
| `typeform_get_form` | Get form details | GET | /forms/{form_id} |
| `typeform_create_form` | Create a form | POST | /forms |
| `typeform_list_responses` | List form responses | GET | /forms/{form_id}/responses |
| `typeform_get_response` | Get response details | GET | /forms/{form_id}/responses/{response_id} |
| `typeform_list_fields` | List form fields | GET | /forms/{form_id}/fields |
| `typeform_get_field` | Get field details | GET | /forms/{form_id}/fields/{field_id} |
| `typeform_create_webhook` | Create a webhook | POST | /forms/{form_id}/webhooks |
| `typeform_list_webhooks` | List webhooks | GET | /forms/{form_id}/webhooks |
| `typeform_calculate_form` | Calculate form logic | POST | /forms/{form_id}/calculate |

---

## Tool Details

### typeform_list_forms

**What it does**: Lists all forms in the Typeform account.

**When to use**: Find forms, browse available surveys.

**Arguments**:
- `page_count` (optional): Number of forms per page (default 200)
- `page` (optional): Page number
- `workspace_id` (optional): Filter by workspace ID

**Example LLM prompt**: "List all my Typeform surveys"

---

### typeform_get_form

**What it does**: Gets detailed information about a specific form including fields and settings.

**When to use**: Get form structure, questions, settings.

**Arguments**:
- `form_id` (required): Form ID

**Example LLM prompt**: "Get the customer feedback form structure"

---

### typeform_create_form

**What it does**: Creates a new form with the specified title and fields.

**When to use**: Create a new survey or form.

**Arguments**:
- `title` (required): Form title
- `fields` (optional): Array of field definitions
- `settings` (optional): Form settings (language, progress bar, etc.)

**Example LLM prompt**: "Create a new form called Customer Satisfaction Survey"

---

### typeform_list_responses

**What it does**: Lists all responses for a specific form.

**When to use**: Gather survey results, analyze feedback.

**Arguments**:
- `form_id` (required): Form ID
- `page_count` (optional): Number of responses per page (default 100)
- `after` (optional): Filter responses after this submission time
- `before` (optional): Filter responses before this submission time

**Example LLM prompt**: "Get all responses from last week's customer survey"

---

### typeform_get_response

**What it does**: Gets detailed information about a specific response including all answers.

**When to use**: View individual response with all answers.

**Arguments**:
- `form_id` (required): Form ID
- `response_id` (required): Response ID

**Example LLM prompt**: "Get response details for submission abc123"

---

### typeform_list_fields

**What it does**: Lists all fields (questions) in a specific form.

**When to use**: Get form structure, question types.

**Arguments**:
- `form_id` (required): Form ID

**Example LLM prompt**: "What questions are in the NPS survey?"

---

### typeform_get_field

**What it does**: Gets detailed information about a specific field including properties and validation.

**When to use**: Get field details, validation rules, choices.

**Arguments**:
- `form_id` (required): Form ID
- `field_id` (required): Field ID

**Example LLM prompt**: "Get details for the rating field in the feedback form"

---

### typeform_create_webhook

**What it does**: Creates a webhook for a form to receive response notifications.

**When to use**: Set up real-time notifications when forms are submitted.

**Arguments**:
- `form_id` (required): Form ID
- `url` (required): Webhook endpoint URL
- `enabled` (optional): Enable the webhook immediately (default true)
- `secret` (optional): Secret for verifying webhook signatures

**Example LLM prompt**: "Add a webhook to notify our Slack channel when a form is submitted"

---

### typeform_list_webhooks

**What it does**: Lists all webhooks configured for a form.

**When to use**: Check existing webhook integrations.

**Arguments**:
- `form_id` (required): Form ID

**Example LLM prompt**: "What webhooks are set up for this form?"

---

### typeform_calculate_form

**What it does**: Calculates form logic and gets expected output for given answers.

**When to use**: Test form logic, get conditional path results.

**Arguments**:
- `form_id` (required): Form ID
- `answers` (required): Answers to calculate logic against

**Example LLM prompt**: "Calculate the form output for these answers: rating=9, feedback='Great service'"

---

## Typeform API Notes

- **Pagination**: Use `page_count` and `page` for large result sets
- **Date filtering**: Use `after` and `before` with ISO 8601 timestamps
- **Field IDs**: Unique identifiers for each question in a form
- **Webhooks**: Must be enabled and point to a valid HTTPS endpoint
- **Form calculation**: Useful for testing logic branching before deployment
