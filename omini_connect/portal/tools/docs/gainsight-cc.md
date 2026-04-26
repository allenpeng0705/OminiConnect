# Gainsight CC Tools

Provider: `gainsight-cc` | Engine: `nango` | Auth: OAUTH2_CC via Nango

## Overview

These tools wrap the Gainsight Customer Success API. They allow AI agents to manage companies, users, health scores, activities, NPS surveys, and playbooks. **Requires Gainsight OAuth2 Client Credentials.**

## Authentication

**Nango OAUTH2_CC (Client Credentials)**:
- Uses client_id and client_secret to obtain access token
- Token stored in Nango, accessed via `connection_ref`
- Region-based base URL: `https://api2-{region}.insided.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gainsight_list_companies` | List companies | GET | /v1/companies |
| `gainsight_get_company` | Get company details | GET | /v1/companies/{id} |
| `gainsight_list_users` | List users | GET | /v1/users |
| `gainsight_get_user` | Get user details | GET | /v1/users/{id} |
| `gainsight_list_csr_tasks` | List CSR tasks | GET | /v1/csrtasks |
| `gainsight_list_health_scores` | List health scores | GET | /v1/healthscores |
| `gainsight_list_activities` | List activities | GET | /v1/activities |
| `gainsight_list_nps_scores` | List NPS scores | GET | /v1/nps |
| `gainsight_list_survey_responses` | List survey responses | GET | /v1/surveyresponses |
| `gainsight_list_playbooks` | List playbooks | GET | /v1/playbooks |

---

## Tool Details

### gainsight_list_companies

**What it does**: Lists all companies in Gainsight.

**When to use**: Get an overview of all customer companies.

**Arguments**:
- `page` (optional): Page number (default 1)
- `limit` (optional): Results per page (default 20)

**Example LLM prompt**: "List all companies in Gainsight"

---

### gainsight_get_company

**What it does**: Gets detailed information about a specific company.

**When to use**: Get company details, CSM, health score, etc.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get details for company abc123"

---

### gainsight_list_users

**What it does**: Lists all users in Gainsight.

**When to use**: Find users, filter by company.

**Arguments**:
- `company_id` (optional): Filter by company ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all users at company abc123"

---

### gainsight_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user details, role, company association.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user john@company.com"

---

### gainsight_list_csr_tasks

**What it does**: Lists customer success tasks and action items.

**When to use**: Track CSM activities and to-dos.

**Arguments**:
- `status` (optional): Filter by status
- `company_id` (optional): Filter by company ID

**Example LLM prompt**: "List all open CSR tasks"

---

### gainsight_list_health_scores

**What it does**: Lists health scores for companies.

**When to use**: Monitor customer health and engagement.

**Arguments**:
- `company_id` (optional): Filter by company ID

**Example LLM prompt**: "List health scores for all companies"

---

### gainsight_list_activities

**What it does**: Lists activities and interactions.

**When to use**: Track customer engagement history.

**Arguments**:
- `company_id` (optional): Filter by company ID
- `type` (optional): Activity type filter

**Example LLM prompt**: "List recent activities for company abc123"

---

### gainsight_list_nps_scores

**What it does**: Lists NPS survey scores and responses.

**When to use**: Monitor customer satisfaction.

**Arguments**:
- `company_id` (optional): Filter by company ID

**Example LLM prompt**: "List NPS scores for this quarter"

---

### gainsight_list_survey_responses

**What it does**: Lists survey responses and results.

**When to use**: Analyze survey data and feedback.

**Arguments**:
- `company_id` (optional): Filter by company ID

**Example LLM prompt**: "List survey responses for company abc123"

---

### gainsight_list_playbooks

**What it does**: Lists available playbooks and templates.

**When to use**: View automation templates for customer success.

**Arguments**: None

**Example LLM prompt**: "List available playbooks"

---

## Gainsight CC API Notes

- **Region configuration**: Required for base URL construction
- **Health scores**: Composite metrics from multiple data points
- **CSR Tasks**: Customer Success Manager action items
- **Playbooks**: Automated workflows for common scenarios
