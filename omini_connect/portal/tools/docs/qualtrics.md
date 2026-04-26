# Qualtrics Tools

Provider: `qualtrics` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Qualtrics API. They allow AI agents to manage surveys, responses, distributions, users, and exports. Qualtrics is an enterprise survey and experience management platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Qualtrics
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `qualtrics_list_surveys` | List all surveys | GET | /surveys |
| `qualtrics_get_survey` | Get survey details | GET | /surveys/{survey_id} |
| `qualtrics_list_responses` | List survey responses | GET | /surveys/{survey_id}/responses |
| `qualtrics_get_response` | Get response details | GET | /surveys/{survey_id}/responses/{response_id} |
| `qualtrics_distribute_survey` | Distribute a survey | POST | /surveys/{survey_id}/distributions |
| `qualtrics_get_distribution` | Get distribution details | GET | /distributions/{distribution_id} |
| `qualtrics_list_users` | List users | GET | /users |
| `qualtrics_get_user` | Get user details | GET | /users/{user_id} |
| `qualtrics_get_report` | Get survey report | GET | /surveys/{survey_id}/reports |
| `qualtrics_list_exports` | List data exports | GET | /surveys/{survey_id}/exports |

---

## Tool Details

### qualtrics_list_surveys

**What it does**: Lists all surveys in the Qualtrics account.

**When to use**: Find surveys, browse available research instruments.

**Arguments**:
- `limit` (optional): Max results (default 50)
- `active` (optional): Filter to only active surveys (default true)

**Example LLM prompt**: "List all active surveys in our account"

---

### qualtrics_get_survey

**What it does**: Gets detailed information about a specific survey including questions.

**When to use**: Get survey structure, questions, settings.

**Arguments**:
- `survey_id` (required): Survey ID

**Example LLM prompt**: "Get the customer satisfaction survey structure"

---

### qualtrics_list_responses

**What it does**: Lists all responses for a specific survey.

**When to use**: Gather survey results, analyze feedback.

**Arguments**:
- `survey_id` (required): Survey ID
- `limit` (optional): Max results (default 50)
- `start_date` (optional): Filter from date (YYYY-MM-DD)
- `end_date` (optional): Filter to date (YYYY-MM-DD)

**Example LLM prompt**: "Get all responses from last month's NPS survey"

---

### qualtrics_get_response

**What it does**: Gets detailed information about a specific survey response.

**When to use**: View individual response with all answers and metadata.

**Arguments**:
- `survey_id` (required): Survey ID
- `response_id` (required): Response ID

**Example LLM prompt**: "Get details for response ABC123"

---

### qualtrics_distribute_survey

**What it does**: Distributes a survey via email, link, or social media.

**When to use**: Send surveys to recipients, generate sharing links.

**Arguments**:
- `survey_id` (required): Survey ID
- `type` (required): Distribution type: email, link, generic
- `recipients` (optional): Array of recipient emails for email distribution
- `message` (optional): Email message content
- `from_name` (optional): Sender name

**Example LLM prompt**: "Send the NPS survey to all customers who purchased last quarter"

---

### qualtrics_get_distribution

**What it does**: Gets details about a specific survey distribution.

**When to use**: Check distribution status, recipient progress.

**Arguments**:
- `distribution_id` (required): Distribution ID

**Example LLM prompt**: "Get status of the March newsletter survey distribution"

---

### qualtrics_list_users

**What it does**: Lists all users in the Qualtrics account.

**When to use**: Find team members, check user roles.

**Arguments**:
- `limit` (optional): Max results (default 50)
- `organization_id` (optional): Filter by organization ID

**Example LLM prompt**: "List all users in our Qualtrics account"

---

### qualtrics_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user profile, role, permissions.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user john@example.com"

---

### qualtrics_get_report

**What it does**: Gets survey report with response statistics and analytics.

**When to use**: View survey analytics, response summaries.

**Arguments**:
- `survey_id` (required): Survey ID
- `report_type` (optional): Report type: default, crosstab, banner (default default)

**Example LLM prompt**: "Get the quarterly report for the product feedback survey"

---

### qualtrics_list_exports

**What it does**: Lists all data exports for a survey.

**When to use**: Find existing exports, check export status.

**Arguments**:
- `survey_id` (required): Survey ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all data exports for the customer survey"

---

## Qualtrics API Notes

- **Survey IDs**: Format like `SV_1234567890abcdef`
- **Response IDs**: Unique identifiers for each survey response
- **Distribution types**: email (individual recipients), link (shareable URL), generic (multi-channel)
- **Date filtering**: Use start_date and end_date in YYYY-MM-DD format
- **Pagination**: Use limit parameter to control result size
- **Reports**: Different report types provide varying levels of analysis detail
