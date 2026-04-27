# Looker Tools

Provider: `looker` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Looker API. They allow AI agents to explore looks, dashboards, models, users, and queries. Looker is a modern business intelligence platform for data exploration and visualization.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Looker
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `LookerOAuth`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `looker_list_looks` | List looks | GET | /looks |
| `looker_get_look` | Get look details | GET | /looks/{lookId} |
| `looker_list_dashboards` | List dashboards | GET | /dashboards |
| `looker_get_dashboard` | Get dashboard details | GET | /dashboards/{dashboardId} |
| `looker_list_models` | List models | GET | /models |
| `looker_get_model` | Get model details | GET | /models/{modelName} |
| `looker_list_users` | List users | GET | /users |
| `looker_get_user` | Get user details | GET | /users/{userId} |
| `looker_run_look` | Run a look and get results | GET | /looks/{lookId}/run |
| `looker_get_query` | Get query details | GET | /queries/{queryId} |

---

## Tool Details

### looker_list_looks

**What it does**: Lists all looks the user has access to.

**When to use**: Find saved analysis and reports.

**Arguments**:
- `limit` (optional): Number of results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all looks I have access to"

---

### looker_get_look

**What it does**: Gets detailed information about a specific look including the underlying query and visualization configuration.

**When to use**: Get the query definition and configuration of a look.

**Arguments**:
- `lookId` (required): Look ID

**Example LLM prompt**: "Get details for look monthly_revenue"

---

### looker_list_dashboards

**What it does**: Lists all dashboards the user has access to with pagination support.

**When to use**: Discover available dashboards before drilling into specific ones.

**Arguments**:
- `limit` (optional): Number of results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all dashboards available to me"

---

### looker_get_dashboard

**What it does**: Gets detailed information about a specific dashboard including tiles, filters, and layout.

**When to use**: Understand dashboard composition before extracting data.

**Arguments**:
- `dashboardId` (required): Dashboard ID or slug

**Example LLM prompt**: "Get details for dashboard sales_overview"

---

### looker_list_models

**What it does**: Lists all Looker models (containing views/explores).

**When to use**: Discover available data models for querying.

**Arguments**:
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "What models are available in Looker"

---

### looker_get_model

**What it does**: Gets detailed information about a specific model including available views and explores.

**When to use**: Understand the structure of a model before writing queries.

**Arguments**:
- `modelName` (required): Model name

**Example LLM prompt**: "Get details for the ecommerce model"

---

### looker_list_users

**What it does**: Lists all users in the Looker instance.

**When to use**: Find users, check permissions, or list team members.

**Arguments**:
- `limit` (optional): Number of results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all users in the Looker instance"

---

### looker_get_user

**What it does**: Gets detailed information about a specific user including roles and permissions.

**When to use**: Check user details, verify permissions, or find contact information.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user 42"

---

### looker_run_look

**What it does**: Runs a Look look and returns the results in the specified format.

**When to use**: Execute a saved look to get fresh data.

**Arguments**:
- `lookId` (required): Look ID
- `format` (optional): Output format: json, csv, html (default json)

**Example LLM prompt**: "Run look monthly_reports to get the latest data"

---

### looker_get_query

**What it does**: Gets details about a specific saved query including the query definition.

**When to use**: Understand the structure of a query before running it.

**Arguments**:
- `queryId` (required): Query ID

**Example LLM prompt**: "Get details for query 123"

---

## Looker API Notes

- **Models and Views**: Looker organizes data into models (containing views/explores)
- **Looks vs Dashboards**: Looks are individual saved visualizations; dashboards combine multiple looks
- **Fields**: Can be referenced as `model.view.field` or just `field`
- **Filters**: Use Looker filter syntax (e.g., `status=active`, `created_date>30 days ago`)
- **Pagination**: Use limit and offset for paginated results
- **Running looks**: Returns fresh data based on the look's underlying query
