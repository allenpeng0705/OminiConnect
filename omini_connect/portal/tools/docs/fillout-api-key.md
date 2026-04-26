# Fillout API Key Tools

Provider: `fillout-api-key` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Fillout API using API Key authentication. They allow AI agents to manage forms, responses, and questions. Fillout is a form building and survey platform.

## Authentication

**Nango API_KEY**:
- User provides their Fillout API key via Nango Connect
- Key is passed in the Authorization header as Bearer token
- User also provides the hostname for their Fillout instance
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fillout_api_key_list_forms` | List forms | GET | /v1/api/forms |
| `fillout_api_key_get_form` | Get form details | GET | /v1/api/forms/{id} |
| `fillout_api_key_list_responses` | List responses | GET | /v1/api/forms/{formId}/responses |
| `fillout_api_key_get_response` | Get response details | GET | /v1/api/forms/{formId}/responses/{responseId} |
| `fillout_api_key_list_questions` | List questions | GET | /v1/api/forms/{formId}/questions |
| `fillout_api_key_get_question` | Get question details | GET | /v1/api/forms/{formId}/questions/{questionId} |
| `fillout_api_key_list_answers` | List answers | GET | /v1/api/forms/{formId}/questions/{questionId}/answers |
| `fillout_api_key_get_answer` | Get answer details | GET | /v1/api/forms/{formId}/questions/{questionId}/answers/{answerId} |
| `fillout_api_key_list_captures` | List file captures | GET | /v1/api/forms/{formId}/captures |
| `fillout_api_key_get_workflow` | Get workflow status | GET | /v1/api/forms/{formId}/workflow |

---

## Tool Details

### fillout_api_key_list_forms

**What it does**: Lists all forms.

**When to use**: Browse available forms, find form IDs.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all my forms"

---

### fillout_api_key_get_form

**What it does**: Gets details of a specific form.

**When to use**: View form configuration, question list.

**Arguments**:
- `id` (required): Form ID

**Example LLM prompt**: "Get details for form abc123"

---

### fillout_api_key_list_responses

**What it does**: Lists all responses to a form.

**When to use**: Browse collected data, find responses.

**Arguments**:
- `formId` (required): Form ID
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List responses for form abc123"

---

### fillout_api_key_get_response

**What it does**: Gets details of a specific response.

**When to use**: View individual response data.

**Arguments**:
- `formId` (required): Form ID
- `responseId` (required): Response ID

**Example LLM prompt**: "Get response xyz789 from form abc123"

---

### fillout_api_key_list_questions

**What it does**: Lists all questions in a form.

**When to use**: View form structure.

**Arguments**:
- `formId` (required): Form ID

**Example LLM prompt**: "List questions for form abc123"

---

### fillout_api_key_get_question

**What it does**: Gets details of a specific question.

**When to use**: View question configuration.

**Arguments**:
- `formId` (required): Form ID
- `questionId` (required): Question ID

**Example LLM prompt**: "Get question def456 from form abc123"

---

### fillout_api_key_list_answers

**What it does**: Lists all answers to a specific question.

**When to use**: View responses to a specific question.

**Arguments**:
- `formId` (required): Form ID
- `questionId` (required): Question ID

**Example LLM prompt**: "List answers for question def456"

---

### fillout_api_key_get_answer

**What it does**: Gets a specific answer.

**When to use**: View individual answer.

**Arguments**:
- `formId` (required): Form ID
- `questionId` (required): Question ID
- `answerId` (required): Answer ID

**Example LLM prompt**: "Get answer ghi789"

---

### fillout_api_key_list_captures

**What it does**: Lists file captures from form responses.

**When to use**: Find uploaded files.

**Arguments**:
- `formId` (required): Form ID

**Example LLM prompt**: "List file captures for form abc123"

---

### fillout_api_key_get_workflow

**What it does**: Gets workflow status for a form.

**When to use**: Check automation status.

**Arguments**:
- `formId` (required): Form ID

**Example LLM prompt**: "Get workflow status for form abc123"

---

## Fillout API Key Notes

- **API Key vs OAuth**: Same API, different auth methods
- **Forms**: Surveys and data collection forms
- **Responses**: Individual form submissions
- **Questions**: Form fields and prompts
- **Answers**: Responses to questions
- **Workflows**: Automation triggered by form submissions
