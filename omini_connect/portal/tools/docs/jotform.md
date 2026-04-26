# Jotform Tools

Provider: `jotform` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Jotform API. They allow AI agents to manage forms, submissions, questions, and reports. **Requires Jotform API key.**

## Authentication

**API Key via Nango**:
- User provides their Jotform API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.jotform.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `jotform_list_forms` | List forms | GET | /user/forms |
| `jotform_get_form` | Get form details | GET | /form/{form_id} |
| `jotform_list_submissions` | List form submissions | GET | /form/{form_id}/submissions |
| `jotform_get_submission` | Get submission details | GET | /submission/{submission_id} |
| `jotform_create_form` | Create a form | POST | /user/forms |
| `jotform_list_users` | List users | GET | /user |
| `jotform_list_questions` | List form questions | GET | /form/{form_id}/questions |
| `jotform_list_reports` | List reports | GET | /user/reports |
| `jotform_get_report` | Get report details | GET | /report/{report_id} |
| `jotform_list_subusers` | List subusers | GET | /user/subusers |

---

## Tool Details

### jotform_list_forms

**What it does**: Lists all forms.

**When to use**: Find forms, view form list.

**Arguments**:
- `limit` (optional): Max results (default: 20)
- `offset` (optional): Offset for pagination (default: 0)

**Example LLM prompt**: "List all forms in Jotform"

---

### jotform_get_form

**What it does**: Gets details for a specific form.

**When to use**: Get form information, view form settings.

**Arguments**:
- `form_id` (required): Form ID

**Example LLM prompt**: "Get details for form abc123"

---

### jotform_list_submissions

**What it does**: Lists submissions for a form.

**When to use**: View form responses, analyze data.

**Arguments**:
- `form_id` (required): Form ID
- `limit` (optional): Max results (default: 20)
- `offset` (optional): Offset for pagination (default: 0)

**Example LLM prompt**: "List submissions for form abc123"

---

### jotform_get_submission

**What it does**: Gets details for a specific submission.

**When to use**: Get submission information, view response details.

**Arguments**:
- `submission_id` (required): Submission ID

**Example LLM prompt**: "Get details for submission xyz789"

---

### jotform_create_form

**What it does**: Creates a new form.

**When to use**: Create new forms, set up surveys.

**Arguments**:
- `title` (required): Form title
- `form_fields` (optional): Form fields JSON

**Example LLM prompt**: "Create a form titled Contact Us"

---

### jotform_list_users

**What it does**: Lists user account details.

**When to use**: View account information.

**Arguments**: None

**Example LLM prompt**: "Get my Jotform account info"

---

### jotform_list_questions

**What it does**: Lists questions for a form.

**When to use**: View form structure, list fields.

**Arguments**:
- `form_id` (required): Form ID

**Example LLM prompt**: "List questions for form abc123"

---

### jotform_list_reports

**What it does**: Lists all reports.

**When to use**: View reports, analyze data.

**Arguments**:
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all reports in Jotform"

---

### jotform_get_report

**What it does**: Gets details for a specific report.

**When to use**: Get report information.

**Arguments**:
- `report_id` (required): Report ID

**Example LLM prompt**: "Get details for report r1"

---

### jotform_list_subusers

**What it does**: Lists all subusers.

**When to use**: View subusers, manage permissions.

**Arguments**: None

**Example LLM prompt**: "List all subusers in Jotform"

---

## Jotform API Notes

- **IDs**: String IDs for forms, submissions, reports
- **Pagination**: Use limit and offset parameters
- **Submissions**: Responses to forms
- **Reports**: Pre-built or custom reports for form data
