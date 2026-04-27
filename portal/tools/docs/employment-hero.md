# Employment Hero Tools

Provider: `employment-hero` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Employment Hero API. They allow AI agents to manage employees, leave requests, departments, and timesheets. Employment Hero is an HR platform for small and medium businesses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Employment Hero
- Token stored in Nango, accessed via `connection_ref`
- Scopes depend on the OAuth app configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `employment_hero_list_employees` | List employees | GET | /api/v1/employees |
| `employment_hero_get_employee` | Get employee details | GET | /api/v1/employees/{id} |
| `employment_hero_list_leaves` | List leave requests | GET | /api/v1/leaves |
| `employment_hero_get_leave` | Get leave details | GET | /api/v1/leaves/{id} |
| `employment_hero_approve_leave` | Approve leave request | POST | /api/v1/leaves/{id}/approve |
| `employment_hero_list_departments` | List departments | GET | /api/v1/departments |
| `employment_hero_list_jobs` | List job postings | GET | /api/v1/jobs |
| `employment_hero_get_company` | Get company info | GET | /api/v1/company |
| `employment_hero_list_awards` | List awards | GET | /api/v1/awards |
| `employment_hero_list_timesheets` | List timesheets | GET | /api/v1/timesheets |

---

## Tool Details

### employment_hero_list_employees

**What it does**: Lists all employees in the organization with pagination.

**When to use**: Browse employee directory, find specific employees.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all employees in the company"

---

### employment_hero_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: Get employee details, contact info, job role.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get details for employee 123"

---

### employment_hero_list_leaves

**What it does**: Lists leave requests with optional filtering.

**When to use**: Review pending leaves, check employee availability.

**Arguments**:
- `status` (optional): Filter by status (pending, approved, rejected)
- `employee_id` (optional): Filter by employee
- `page` (optional): Page number

**Example LLM prompt**: "List all pending leave requests"

---

### employment_hero_get_leave

**What it does**: Gets details of a specific leave request.

**When to use**: Review leave details before approval.

**Arguments**:
- `id` (required): Leave ID

**Example LLM prompt**: "Get details for leave request 456"

---

### employment_hero_approve_leave

**What it does**: Approves a pending leave request.

**When to use**: Manager approval of employee time off.

**Arguments**:
- `id` (required): Leave ID

**Example LLM prompt**: "Approve leave request 456"

---

### employment_hero_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### employment_hero_list_jobs

**What it does**: Lists all job postings.

**When to use**: View open positions, track recruiting pipeline.

**Arguments**:
- `status` (optional): Filter by status (open, closed)

**Example LLM prompt**: "List all open job postings"

---

### employment_hero_get_company

**What it does**: Gets company information.

**When to use**: Get company details, organizational info.

**Arguments**: None

**Example LLM prompt**: "Get company information"

---

### employment_hero_list_awards

**What it does**: Lists awards and recognitions.

**When to use**: View employee recognition history.

**Arguments**:
- `employee_id` (optional): Filter by employee

**Example LLM prompt**: "List awards for employee 123"

---

### employment_hero_list_timesheets

**What it does**: Lists timesheet entries with date filtering.

**When to use**: Review time tracking data, approve hours.

**Arguments**:
- `employee_id` (optional): Filter by employee
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "List timesheets for the last week"

---

## Employment Hero API Notes

- **Employee IDs**: Unique identifiers for employees
- **Leave statuses**: pending, approved, rejected
- **Departments**: Organizational units for grouping employees
- **Timesheets**: Track employee work hours
- **Awards**: Employee recognition and achievements
