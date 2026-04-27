# Microsoft Power BI Tools

Provider: `microsoft-power-bi` | Engine: `nango` | Auth: OAuth2 via Nango (alias: microsoft)

## Overview

These tools wrap the Power BI REST API. They allow AI agents to manage workspaces, reports, datasets, and capacities. **Requires Power BI OAuth2.**

## Authentication

**Nango OAUTH2 (Microsoft Power BI)**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.powerbi.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `powerbi_list_workspaces` | List workspaces | GET | /v1.0/myorg/groups |
| `powerbi_get_workspace` | Get workspace details | GET | /v1.0/myorg/groups/{groupId} |
| `powerbi_list_reports` | List reports | GET | /v1.0/myorg/groups/{groupId}/reports |
| `powerbi_get_report` | Get report details | GET | /v1.0/myorg/reports/{reportId} |
| `powerbi_list_datasets` | List datasets | GET | /v1.0/myorg/groups/{groupId}/datasets |
| `powerbi_get_dataset` | Get dataset details | GET | /v1.0/myorg/datasets/{datasetId} |
| `powerbi_list_capacities` | List capacities | GET | /v1.0/myorg/capacities |
| `powerbi_get_capacity` | Get capacity details | GET | /v1.0/myorg/capacities/{capacityId} |
| `powerbi_list_groups` | List groups | GET | /v1.0/myorg/groups |
| `powerbi_get_group` | Get group details | GET | /v1.0/myorg/groups/{groupId} |

---

## Tool Details

### powerbi_list_workspaces

**What it does**: Lists all Power BI workspaces accessible to the user.

**When to use**: Browse workspaces, find workspace IDs.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all my Power BI workspaces"

---

### powerbi_get_workspace

**What it does**: Gets details of a specific Power BI workspace.

**When to use**: Check workspace settings, view members.

**Arguments**:
- `groupId` (required): Workspace (Group) ID

**Example LLM prompt**: "Get details for workspace 12345678-1234-1234-1234-123456789012"

---

### powerbi_list_reports

**What it does**: Lists all Power BI reports in a workspace.

**When to use**: Browse reports, find report IDs.

**Arguments**:
- `groupId` (required): Workspace ID

**Example LLM prompt**: "List all reports in workspace 12345678-1234-1234-1234-123456789012"

---

### powerbi_get_report

**What it does**: Gets details of a specific Power BI report.

**When to use**: Check report details, embed URLs.

**Arguments**:
- `reportId` (required): Report ID

**Example LLM prompt**: "Get details for report 12345678-1234-1234-1234-123456789012"

---

### powerbi_list_datasets

**What it does**: Lists all Power BI datasets in a workspace.

**When to use**: Manage datasets, refresh data.

**Arguments**:
- `groupId` (required): Workspace ID

**Example LLM prompt**: "List all datasets in workspace 12345678-1234-1234-1234-123456789012"

---

### powerbi_get_dataset

**What it does**: Gets details of a specific Power BI dataset.

**When to use**: Check dataset settings, table info.

**Arguments**:
- `datasetId` (required): Dataset ID

**Example LLM prompt**: "Get details for dataset 12345678-1234-1234-1234-123456789012"

---

### powerbi_list_capacities

**What it does**: Lists all Power BI capacities (dedicated capacities).

**When to use**: Manage capacity resources.

**Arguments**: None

**Example LLM prompt**: "List all Power BI capacities"

---

### powerbi_get_capacity

**What it does**: Gets details of a specific Power BI capacity.

**When to use**: Check capacity status, state.

**Arguments**:
- `capacityId` (required): Capacity ID

**Example LLM prompt**: "Get details for capacity 12345678-1234-1234-1234-123456789012"

---

### powerbi_list_groups

**What it does**: Lists all Power BI app workspaces (groups).

**When to use**: Find workspaces, organization browsing.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all workspaces in the organization"

---

### powerbi_get_group

**What it does**: Gets details of a specific Power BI app workspace.

**When to use**: Check workspace info, manage members.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "Get details for workspace 12345678-1234-1234-1234-123456789012"

---

## Power BI Notes

- **Workspaces**: Containers for reports and datasets
- **Reports**: Visualizations and dashboards
- **Datasets**: Data sources for reports
- **Capacities**: Dedicated Power BI resources
- **Admin APIs**: Some operations require admin permissions
