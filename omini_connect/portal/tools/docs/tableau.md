# Tableau Tools

Provider: `tableau` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Tableau REST API. They allow AI agents to list and retrieve workbooks, views, datasources, projects, and users. Tableau is a leading visual analytics platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Tableau Cloud
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `TableauOAuth`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tableau_list_workbooks` | List workbooks | GET | /api/api-version/workbooks |
| `tableau_get_workbook` | Get workbook details | GET | /api/api-version/workbooks/{workbookId} |
| `tableau_list_datasources` | List datasources | GET | /api/api-version/datasources |
| `tableau_get_datasource` | Get datasource details | GET | /api/api-version/datasources/{datasourceId} |
| `tableau_list_projects` | List projects | GET | /api/api-version/projects |
| `tableau_get_project` | Get project details | GET | /api/api-version/projects/{projectId} |
| `tableau_list_views` | List views | GET | /api/api-version/views |
| `tableau_get_view` | Get view details | GET | /api/api-version/views/{viewId} |
| `tableau_list_users` | List users | GET | /api/api-version/users |
| `tableau_get_user` | Get user details | GET | /api/api-version/users/{userId} |

---

## Tool Details

### tableau_list_workbooks

**What it does**: Lists all workbooks the user has access to, with optional filtering by site, project, or owner.

**When to use**: Discover available workbooks before retrieving specific content.

**Arguments**:
- `siteId` (optional): Filter by site ID
- `projectId` (optional): Filter by project ID
- `ownerId` (optional): Filter by owner ID

**Example LLM prompt**: "List all workbooks in the Sales site"

---

### tableau_get_workbook

**What it does**: Gets detailed information about a specific workbook including name, description, project, owner, and views.

**When to use**: Read workbook metadata or prepare to extract data.

**Arguments**:
- `workbookId` (required): Workbook ID

**Example LLM prompt**: "Get details for workbook abc123"

---

### tableau_list_datasources

**What it does**: Lists all datasources the user has access to.

**When to use**: Discover available data sources for analysis.

**Arguments**:
- `siteId` (optional): Filter by site ID

**Example LLM prompt**: "List all datasources available to me"

---

### tableau_get_datasource

**What it does**: Gets detailed information about a specific datasource including connection details and ownership.

**When to use**: Understand data source configuration before using it.

**Arguments**:
- `datasourceId` (required): Datasource ID

**Example LLM prompt**: "Get details for datasource def789"

---

### tableau_list_projects

**What it does**: Lists all projects the user has access to, with optional filtering by site or parent project.

**When to use**: Browse project hierarchy for organizing content.

**Arguments**:
- `siteId` (optional): Filter by site ID
- `parentProjectId` (optional): Filter by parent project ID

**Example LLM prompt**: "List all projects in the Analytics site"

---

### tableau_get_project

**What it does**: Gets detailed information about a specific project including name, description, parent project, and permissions.

**When to use**: Understand project structure and settings.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "Get details for project proj123"

---

### tableau_list_views

**What it does**: Lists all views, either within a specific workbook or across a site.

**When to use**: Find views to visualize or embed.

**Arguments**:
- `workbookId` (optional): Filter by workbook ID
- `siteId` (optional): Filter by site ID

**Example LLM prompt**: "List all views in the Quarterly Reports workbook"

---

### tableau_get_view

**What it does**: Gets detailed information about a specific view.

**When to use**: Get view metadata, permissions, or thumbnail info.

**Arguments**:
- `viewId` (required): View ID

**Example LLM prompt**: "Get details for view xyz456"

---

### tableau_list_users

**What it does**: Lists all users the user has access to. Filter by site or role.

**When to use**: Find users, check site membership, or look up user roles.

**Arguments**:
- `siteId` (optional): Filter by site ID
- `role` (optional): Filter by site role (Viewer, Editor, SiteAdmin, etc.)

**Example LLM prompt**: "List all users in the Analytics site"

---

### tableau_get_user

**What it does**: Gets detailed information about a specific user including their site roles and permissions.

**When to use**: Check user details, verify permissions, find contact information.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user jane.doe"

---

## Tableau API Notes

- **API Version**: Tableau uses versioned API endpoints (e.g., `api-version` is replaced with actual version like `3.0`)
- **Site scoping**: Most resources are scoped to a site - include `siteId` when filtering
- **Workbook content**: Workbooks contain views - use `workbookId` to scope views
- **Pagination**: Default pagination is used - consider filtering to reduce results
- **Permissions**: Results are filtered by what the authenticated user can access
- **User roles**: Site roles include Viewer, Editor, SiteAdmin, and more
