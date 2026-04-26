# Medallia Tools

Provider: `medallia` | Engine: `nango` | Auth: OAUTH2_CC via Nango (Client Credentials)

## Overview

These tools wrap the Medallia API. They allow AI agents to manage surveys, responses, employees, and analytics. **Requires Medallia Client Credentials.**

## Authentication

**Nango OAUTH2_CC (Client Credentials)**:
- Uses client credentials via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://${connectionConfig.apiHostUrl}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `medallia_list_surveys` | List all surveys | GET | /surveys |
| `medallia_get_survey` | Get survey details | GET | /surveys/{surveyId} |
| `medallia_list_responses` | List survey responses | GET | /responses |
| `medallia_get_response` | Get response details | GET | /responses/{responseId} |
| `medallia_list_employees` | List employees | GET | /employees |
| `medallia_get_employee` | Get employee details | GET | /employees/{employeeId} |
| `medallia_list_teams` | List teams | GET | /teams |
| `medallia_get_team` | Get team details | GET | /teams/{teamId} |
| `medallia_list_analytics` | Get analytics data | GET | /analytics |
| `medallia_list_invites` | List invitations | GET | /invites |

---

## Tool Details

### medallia_list_surveys

**What it does**: Lists all surveys in Medallia.

**When to use**: Browse surveys, find survey IDs.

**Arguments**:
- `status` (optional): Filter by status (active, closed, draft)
- `survey_type` (optional): Filter by type
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all active surveys"

---

### medallia_get_survey

**What it does**: Gets details of a specific survey.

**When to use**: Review survey configuration, check questions.

**Arguments**:
- `surveyId` (required): Survey ID

**Example LLM prompt**: "Get details for survey 12345"

---

### medallia_list_responses

**What it does**: Lists all survey responses in Medallia.

**When to use**: Analyze feedback, track completion rates.

**Arguments**:
- `survey_id` (optional): Filter by survey ID
- `from_date` (optional): From date (ISO 8601)
- `to_date` (optional): To date (ISO 8601)
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all responses to survey 12345 from last month"

---

### medallia_get_response

**What it does**: Gets details of a specific survey response.

**When to use**: Review individual feedback, investigate responses.

**Arguments**:
- `responseId` (required): Response ID

**Example LLM prompt**: "Get details for response 12345"

---

### medallia_list_employees

**What it does**: Lists all employees in Medallia.

**When to use**: Manage employee list, find employee IDs.

**Arguments**:
- `team_id` (optional): Filter by team ID
- `search` (optional): Search query
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all employees in Engineering team"

---

### medallia_get_employee

**What it does**: Gets details of a specific employee.

**When to use**: Check employee information, view team membership.

**Arguments**:
- `employeeId` (required): Employee ID

**Example LLM prompt**: "Get details for employee 12345"

---

### medallia_list_teams

**What it does**: Lists all teams in Medallia.

**When to use**: View team structure, manage team-based surveys.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all teams"

---

### medallia_get_team

**What it does**: Gets details of a specific team.

**When to use**: Check team composition, manage team surveys.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "Get details for team 12345"

---

### medallia_list_analytics

**What it does**: Gets analytics data from Medallia.

**When to use**: Generate reports, analyze survey results.

**Arguments**:
- `survey_id` (optional): Survey ID
- `from_date` (optional): From date
- `to_date` (optional): To date
- `metric` (optional): Specific metric to retrieve

**Example LLM prompt**: "Get NPS scores for survey 12345"

---

### medallia_list_invites

**What it does**: Lists all survey invitations.

**When to use**: Track invitation status, manage follow-ups.

**Arguments**:
- `survey_id` (optional): Filter by survey ID
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List pending invitations for survey 12345"

---

## Medallia Notes

- **Surveys**: Employee engagement and feedback surveys
- **Responses**: Individual survey submissions
- **Employees**: People management data
- **Teams**: Organizational teams
- **Analytics**: Aggregated survey metrics
