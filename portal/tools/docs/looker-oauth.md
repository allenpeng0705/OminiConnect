# Looker OAuth Tools

Provider: `looker-oauth` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Looker API. They allow AI agents to manage dashboards, looks, queries, and analytics. **Requires Looker OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with Looker
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{apiHostname}/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `looker_oauth_list_dashboards` | List dashboards | GET | /dashboards |
| `looker_oauth_get_dashboard` | Get dashboard details | GET | /dashboards/{dashboard_id} |
| `looker_oauth_list_tiles` | List dashboard tiles | GET | /dashboards/{dashboard_id}/tiles |
| `looker_oauth_list_looks` | List looks | GET | /looks |
| `looker_oauth_get_look` | Get look details | GET | /looks/{look_id} |
| `looker_oauth_list_queries` | List saved queries | GET | /queries |
| `looker_oauth_run_query` | Run a query | GET | /queries/{query_id}/run |
| `looker_oauth_list_users` | List users | GET | /users |
| `looker_oauth_get_user` | Get user details | GET | /users/{user_id} |
| `looker_oauth_list_spaces` | List spaces | GET | /spaces |

---

## Tool Details

### looker_oauth_list_dashboards

**What it does**: Lists all dashboards.

**When to use**: Find dashboards, view analytics.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all dashboards in Looker"

---

### looker_oauth_get_dashboard

**What it does**: Gets details for a specific dashboard.

**When to use**: Get dashboard information.

**Arguments**:
- `dashboard_id` (required): Dashboard ID

**Example LLM prompt**: "Get details for dashboard 12345"

---

### looker_oauth_list_tiles

**What it does**: Lists all tiles in a dashboard.

**When to use**: View dashboard components.

**Arguments**:
- `dashboard_id` (required): Dashboard ID

**Example LLM prompt**: "List tiles for dashboard 12345"

---

### looker_oauth_list_looks

**What it does**: Lists all looks.

**When to use**: View saved visualizations.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all looks in Looker"

---

### looker_oauth_get_look

**What it does**: Gets details for a specific look.

**When to use**: Get look information.

**Arguments**:
- `look_id` (required): Look ID

**Example LLM prompt**: "Get details for look 67890"

---

### looker_oauth_list_queries

**What it does**: Lists all saved queries.

**When to use**: View available queries.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all queries in Looker"

---

### looker_oauth_run_query

**What it does**: Runs a saved query.

**When to use**: Execute analytics queries.

**Arguments**:
- `query_id` (required): Query ID
- `format` (optional): Output format (json, csv, html - default: json)

**Example LLM prompt**: "Run query 111 with JSON output"

---

### looker_oauth_list_users

**What it does**: Lists all users.

**When to use**: View users, find team members.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all users in Looker"

---

### looker_oauth_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user 1"

---

### looker_oauth_list_spaces

**What it does**: Lists all spaces.

**When to use**: View content organization.

**Arguments**: None

**Example LLM prompt**: "List all spaces in Looker"

---

## Looker API Notes

- **Business Intelligence**: Analytics and BI platform
- **Dashboards**: Collections of visualizations
- **Looks**: Saved visualizations
- **Queries**: Saved SQL queries
- **Spaces**: Content organization folders
- **Users**: Platform users and permissions
