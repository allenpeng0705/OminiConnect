# Chattermill Tools

Provider: `chattermill` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Chattermill API. Chattermill is a customer feedback analytics platform that helps businesses understand and act on customer feedback. **Requires Chattermill API key.**

## Authentication

**Nango API_KEY**:
- User provides their Chattermill API key
- Token passed via `Authorization: Bearer` header
- Base URL: `https://${connectionConfig.subdomain}.chattermill.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `chattermill_list_projects` | List projects | GET | /v1/projects |
| `chattermill_get_project` | Get project details | GET | /v1/projects/{id} |
| `chattermill_list_reviews` | List reviews | GET | /v1/reviews |
| `chattermill_get_review` | Get review details | GET | /v1/reviews/{id} |
| `chattermill_list_themes` | List themes | GET | /v1/projects/{projectId}/themes |
| `chattermill_get_theme` | Get theme details | GET | /v1/themes/{id} |
| `chattermill_list_reports` | List reports | GET | /v1/projects/{projectId}/reports |
| `chattermill_get_report` | Get report details | GET | /v1/reports/{id} |
| `chattermill_list_comments` | List comments | GET | /v1/reviews/{reviewId}/comments |
| `chattermill_create_comment` | Create a comment | POST | /v1/reviews/{reviewId}/comments |

---

## Tool Details

### chattermill_list_projects

**What it does**: Lists all projects in the account.

**When to use**: Find projects for viewing feedback.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List my Chattermill projects"

---

### chattermill_get_project

**What it does**: Gets details of a specific project.

**When to use**: View project settings and summary.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get project 123 details"

---

### chattermill_list_reviews

**What it does**: Lists all reviews in a project.

**When to use**: Browse customer feedback.

**Arguments**:
- `project_id` (required): Project ID
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List reviews for project 123"

---

### chattermill_get_review

**What it does**: Gets details of a specific review.

**When to use**: View individual feedback and scores.

**Arguments**:
- `id` (required): Review ID

**Example LLM prompt**: "Get review 456 details"

---

### chattermill_list_themes

**What it does**: Lists all themes in a project.

**When to use**: View organized topic categories.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "List themes for project 123"

---

### chattermill_get_theme

**What it does**: Gets details of a specific theme.

**When to use**: View theme summary and related reviews.

**Arguments**:
- `id` (required): Theme ID

**Example LLM prompt**: "Get theme 789 details"

---

### chattermill_list_reports

**What it does**: Lists all reports in a project.

**When to use**: View analytics reports.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "List reports for project 123"

---

### chattermill_get_report

**What it does**: Gets details of a specific report.

**When to use**: View detailed analytics.

**Arguments**:
- `id` (required): Report ID

**Example LLM prompt**: "Get report 101 details"

---

### chattermill_list_comments

**What it does**: Lists all comments on a review.

**When to use**: View collaboration on feedback.

**Arguments**:
- `reviewId` (required): Review ID

**Example LLM prompt**: "List comments for review 456"

---

### chattermill_create_comment

**What it does**: Creates a new comment on a review.

**When to use**: Add annotations to customer feedback.

**Arguments**:
- `reviewId` (required): Review ID
- `content` (required): Comment text

**Example LLM prompt**: "Add a comment to review 456"

---

## Chattermill API Notes

- **Subdomain**: Required configuration for API access
- **Projects**: Top-level containers for feedback data
- **Reviews**: Individual customer feedback entries
- **Themes**: Categorized topics extracted from reviews
- **Reports**: Analytics summaries of feedback data
