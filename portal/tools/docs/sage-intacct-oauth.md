# Sage-intacct-oauth Tools

Provider: `sage-intacct-oauth` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Sage Intacct with OAuth2 authentication. These tools allow AI agents to manage locations, departments, employees, projects, GL accounts, journals, and financial reports.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Sage Intacct using OAuth2
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `locations:read`, `departments:read`, `employees:read`, `projects:read`, `gl-accounts:read`, `journals:read`, `reports:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sage-intacct-oauth_list_locations` | List all locations | GET | /v3/locations |
| `sage-intacct-oauth_get_location` | Get location details | GET | /v3/locations/{locationId} |
| `sage-intacct-oauth_list_departments` | List departments | GET | /v3/departments |
| `sage-intacct-oauth_list_employees` | List employees | GET | /v3/employees |
| `sage-intacct-oauth_get_employee` | Get employee details | GET | /v3/employees/{employeeId} |
| `sage-intacct-oauth_list_projects` | List projects | GET | /v3/projects |
| `sage-intacct-oauth_get_project` | Get project details | GET | /v3/projects/{projectId} |
| `sage-intacct-oauth_list_GL_accounts` | List GL accounts | GET | /v3/gl-accounts |
| `sage-intacct-oauth_list_journals` | List journal entries | GET | /v3/journals |
| `sage-intacct-oauth_get_reports` | Get financial reports | GET | /v3/reports |

---

## Tool Details

### sage-intacct-oauth_list_locations

**What it does**: Returns a list of all locations.

**When to use**: View organizational locations.

**Arguments**:
- `limit` (optional): Number of locations (default 50)

**Example LLM prompt**: "List all locations"

---

### sage-intacct-oauth_get_location

**What it does**: Gets details of a specific location.

**When to use**: Get location information.

**Arguments**:
- `locationId` (required): The location ID

**Example LLM prompt**: "Get details for location loc_abc123"

---

### sage-intacct-oauth_list_departments

**What it does**: Returns a list of all departments.

**When to use**: View department structure.

**Arguments**:
- `limit` (optional): Number of departments (default 50)

**Example LLM prompt**: "List all departments"

---

### sage-intacct-oauth_list_employees

**What it does**: Returns a list of all employees.

**When to use**: View employee roster.

**Arguments**:
- `limit` (optional): Number of employees (default 50)

**Example LLM prompt**: "List all employees"

---

### sage-intacct-oauth_get_employee

**What it does**: Gets details of a specific employee.

**When to use**: Get employee information.

**Arguments**:
- `employeeId` (required): The employee ID

**Example LLM prompt**: "Get details for employee emp_xyz789"

---

### sage-intacct-oauth_list_projects

**What it does**: Returns a list of all projects.

**When to use**: View project portfolio.

**Arguments**:
- `limit` (optional): Number of projects (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active projects"

---

### sage-intacct-oauth_get_project

**What it does**: Gets details of a specific project.

**When to use**: Get project financial details.

**Arguments**:
- `projectId` (required): The project ID

**Example LLM prompt**: "Get details for project prj_123"

---

### sage-intacct-oauth_list_GL_accounts

**What it does**: Returns a list of GL accounts.

**When to use**: View chart of accounts.

**Arguments**:
- `limit` (optional): Number of accounts (default 50)

**Example LLM prompt**: "List all GL accounts"

---

### sage-intacct-oauth_list_journals

**What it does**: Returns journal entries.

**When to use**: View accounting transactions.

**Arguments**:
- `limit` (optional): Number of entries (default 50)

**Example LLM prompt**: "List recent journal entries"

---

### sage-intacct-oauth_get_reports

**What it does**: Returns financial reports.

**When to use**: Get financial statements.

**Arguments**:
- `reportId` (required): Report ID
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Get P&L report for Q1"

---

## Sage Intacct OAuth Notes

- Same API as standard Sage Intacct - only auth differs
- OAuth2 provides secure token-based authentication
- All other functionality remains identical
