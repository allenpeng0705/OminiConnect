# Mode Tools

Provider: `mode` | Engine: `nango` | Auth: API Key (via Nango)

## Overview

These tools wrap the Mode API. They allow AI agents to manage workspaces, reports, dashboards, queries, and users. Mode is a collaborative analytics platform for SQL and Python.

## Authentication

**Nango API Key**:
- User provides Mode API token via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `api_key`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mode_list_reports` | List reports in a workspace | GET | /api/workspaces/{workspaceId}/reports |
| `mode_get_report` | Get report details | GET | /api/workspaces/{workspaceId}/reports/{reportId} |
| `mode_create_report` | Create a new report | POST | /api/workspaces/{workspaceId}/reports |
| `mode_list_dashboards` | List dashboards in a workspace | GET | /api/workspaces/{workspaceId}/dashboards |
| `mode_get_dashboard` | Get dashboard details | GET | /api/workspaces/{workspaceId}/dashboards/{dashboardId} |
| `mode_run_query` | Run a SQL query | POST | /api/workspaces/{workspaceId}/query |
| `mode_list_queries` | List saved queries | GET | /api/workspaces/{workspaceId}/queries |
| `mode_get_query` | Get query details | GET | /api/workspaces/{workspaceId}/queries/{queryId} |
| `mode_list_users` | List workspace users | GET | /api/workspaces/{workspaceId}/members |
| `mode_get_user` | Get user details | GET | /api/workspaces/{workspaceId}/members/{userId} |

---

## Tool Details

### mode_list_reports

**What it does**: Lists all reports in a Mode workspace with pagination. Returns report names, descriptions, and creation dates.

**When to use**: Discover available analyses and visualizations in a workspace.

**Arguments**:
- `workspaceId` (required): Workspace ID
- `limit` (optional): Maximum number of reports to return (default 25)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all reports in the Analytics workspace"

---

### mode_get_report

**What it does**: Returns detailed information about a specific report including the query, visualization, and parameters.

**When to use**: Understand report composition before modifying or sharing.

**Arguments**:
- `workspaceId` (required): Workspace ID
- `reportId` (required): Report ID

**Example LLM prompt**: "Get details for report rpt456 in workspace abc123"

---

### mode_create_report

**What it does**: Creates a new report in a Mode workspace. Specify the query, name, and visualization settings.

**When to use**: Save a new analysis with visualizations for the team.

**Arguments**:
- `workspaceId` (required): Workspace ID
- `name` (required): Report name
- `query` (required): Query definition including SQL and data source
- `visualization` (optional): Visualization type and options

**Example LLM prompt**: "Create a report called 'Revenue by Region' using the sales query"

---

### mode_list_dashboards

**What it does**: Lists all dashboards in a Mode workspace. Returns dashboard names, descriptions, and publication status.

**When to use**: Discover available dashboards for sharing or analysis.

**Arguments**:
- `workspaceId` (required): Workspace ID
- `limit` (optional): Maximum number of dashboards to return (default 25)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "Show me all dashboards in the Analytics workspace"

---

### mode_get_dashboard

**What it does**: Returns detailed information about a specific dashboard including its reports, filters, and layout.

**When to use**: Understand dashboard composition before sharing or modifying.

**Arguments**:
- `workspaceId` (required): Workspace ID
- `dashboardId` (required): Dashboard ID

**Example LLM prompt**: "Get details for dashboard dash789 in workspace abc123"

---

### mode_run_query

**What it does**: Executes a SQL query in Mode and returns results. The query runs against the specified data source.

**When to use**: Execute ad-hoc analysis or retrieve specific data.

**Arguments**:
- `workspaceId` (required): Workspace ID
- `sql` (required): SQL query string
- `dataSourceId` (optional): Data source ID to query against

**Example LLM prompt**: "Run a query to get total revenue by product from the analytics workspace"

---

### mode_list_queries

**What it does**: Lists all saved queries in a Mode workspace. Returns query names, SQL, and execution history.

**When to use**: Discover reusable SQL snippets for common analyses.

**Arguments**:
- `workspaceId` (required): Workspace ID
- `limit` (optional): Maximum number of queries to return (default 25)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all saved queries in the Analytics workspace"

---

### mode_get_query

**What it does**: Returns detailed information about a saved query including the SQL, parameters, and last execution results.

**When to use**: Understand query structure before running or modifying.

**Arguments**:
- `workspaceId` (required): Workspace ID
- `queryId` (required): Query ID

**Example LLM prompt**: "Get details for query qry123 in workspace abc123"

---

### mode_list_users

**What it does**: Lists all users in a Mode workspace. Returns user names, emails, and roles.

**When to use**: Discover workspace members and their permissions.

**Arguments**:
- `workspaceId` (required): Workspace ID
- `limit` (optional): Maximum number of users to return (default 25)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all users in the Analytics workspace"

---

### mode_get_user

**What it does**: Returns detailed information about a specific workspace user including their role and permissions.

**When to use**: Check user permissions or role in the workspace.

**Arguments**:
- `workspaceId` (required): Workspace ID
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user john@example.com in workspace abc123"

---

## Mode API Notes

- **Workspace-first**: All resources are scoped to a workspace - always specify `workspaceId`
- **Reports**: Reports contain queries and visualizations
- **Dashboards**: Collections of reports with shared filters
- **Queries**: Saved SQL queries with optional parameters
- **Data sources**: Must be configured in Mode before they can be queried
- **Members**: Can have different roles (viewer, editor, admin) within a workspace
- **Pagination**: Default limit is 25 for lists
