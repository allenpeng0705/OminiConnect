# Google Forms Tools

Provider: `google-forms` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Forms API. They allow AI agents to manage forms, questions, responses, and watches. **Requires Google OAuth2 with Forms permissions.**

## Authentication

**Nango OAUTH2 (Google Forms)**:
- User authenticates via OAuth2 with Forms scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://forms.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_forms_list_forms` | List forms | GET | /forms/v1/forms |
| `google_forms_get_form` | Get form details | GET | /forms/v1/forms/{formId} |
| `google_forms_create_form` | Create form | POST | /forms/v1/forms |
| `google_forms_update_form` | Update form | PATCH | /forms/v1/forms/{formId} |
| `google_forms_list_questions` | List questions | GET | /forms/v1/forms/{formId}/questions |
| `google_forms_create_question` | Create question | POST | /forms/v1/forms/{formId}/questions |
| `google_forms_list_responses` | List responses | GET | /forms/v1/forms/{formId}/responses |
| `google_forms_get_response` | Get response details | GET | /forms/v1/forms/{formId}/responses/{responseId} |
| `google_forms_list_watches` | List watches | GET | /forms/v1/forms/{formId}/watches |
| `google_forms_create_watch` | Create watch | POST | /forms/v1/forms/{formId}/watches |

---

## Tool Details

### google_forms_list_forms

**What it does**: Lists forms in Google Forms.

**When to use**: Browse user's forms.

**Arguments**: None

**Example LLM prompt**: "List my Google Forms"

---

### google_forms_get_form

**What it does**: Gets detailed information about a form.

**When to use**: Get form settings and structure.

**Arguments**:
- `formId` (required): Form ID

**Example LLM prompt**: "Get details for form abc123"

---

### google_forms_create_form

**What it does**: Creates a new Google Form.

**When to use**: Create new surveys or forms.

**Arguments**:
- `title` (required): Form title

**Example LLM prompt**: "Create a new form titled 'Customer Survey'"

---

### google_forms_update_form

**What it does**: Updates a form's settings.

**When to use**: Modify form title or settings.

**Arguments**:
- `formId` (required): Form ID
- `title` (optional): New title

**Example LLM prompt**: "Update form abc123 with new title"

---

### google_forms_list_questions

**What it does**: Lists questions in a form.

**When to use**: See form structure.

**Arguments**:
- `formId` (required): Form ID

**Example LLM prompt**: "List questions in form abc123"

---

### google_forms_create_question

**What it does**: Creates a question in a form.

**When to use**: Add questions to forms.

**Arguments**:
- `formId` (required): Form ID
- `question` (required): Question object

**Example LLM prompt**: "Add a text question to form abc123"

---

### google_forms_list_responses

**What it does**: Lists responses to a form.

**When to use**: View form submissions.

**Arguments**:
- `formId` (required): Form ID

**Example LLM prompt**: "List responses for form abc123"

---

### google_forms_get_response

**What it does**: Gets details of a form response.

**When to use**: View individual submission.

**Arguments**:
- `formId` (required): Form ID
- `responseId` (required): Response ID

**Example LLM prompt**: "Get response xyz789 for form abc123"

---

### google_forms_list_watches

**What it does**: Lists watches on a form.

**When to use**: See configured webhooks.

**Arguments**:
- `formId` (required): Form ID

**Example LLM prompt**: "List watches for form abc123"

---

### google_forms_create_watch

**What it does**: Creates a watch (webhook) on a form.

**When to use**: Get notified of new responses.

**Arguments**:
- `formId` (required): Form ID
- `watch` (required): Watch object with target and event type

**Example LLM prompt**: "Create a watch for form abc123 to notify my webhook"

---

## Google Forms API Notes

- **Form ID**: Found in the form URL
- **Questions**: RADIO, CHECKBOX, TEXT, DATE, etc.
- **Responses**: Individual form submissions
- **Watches**: Webhook notifications for new responses
- **Scopes**: forms.body for editing, forms.responses.readonly for reading responses
