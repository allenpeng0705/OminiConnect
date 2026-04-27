# Namely (PAT) Tools

Provider: `namely-pat` | Engine: `nango` | Auth: API_KEY (Personal Access Token) via Nango

## Overview

These tools wrap the Namely HR platform API using a Personal Access Token. They allow AI agents to manage employees, departments, job postings, time off requests, benefits, and payroll. **Requires Namely PAT authentication.**

## Authentication

**Personal Access Token (API_KEY)**:
- User provides a Personal Access Token from Namely
- Token passed via `Authorization: Bearer` header
- Same API endpoints as OAuth2 variant

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `namely_list_employees` | List employees | GET | /v1/employees |
| `namely_get_employee` | Get employee details | GET | /v1/employees/{id} |
| `namely_list_departments` | List departments | GET | /v1/departments |
| `namely_list_jobs` | List job postings | GET | /v1/jobs |
| `namely_list_time_off` | List time off requests | GET | /v1/time_off_requests |
| `namely_create_time_off` | Create time off request | POST | /v1/time_off_requests |
| `namely_list_benefits` | List benefits enrollment | GET | /v1/benefits_enrollments |
| `namely_list_payroll` | List payroll data | GET | /v1/payroll |
| `namely_list_company` | Get company info | GET | /v1/company |
| `namely_list_profiles` | List employee profiles | GET | /v1/profiles |

---

## Tool Details

### namely_list_employees

**What it does**: Lists all employees in the organization.

**When to use**: Browse employee directory, find employees by department.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Items per page (default 20, max 100)

**Example LLM prompt**: "List all employees in the engineering department"

---

### namely_get_employee

**What it does**: Gets detailed information for a specific employee.

**When to use**: Get employee details, contact information, job title.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get details for employee with ID 12345"

---

### namely_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: View organizational structure, find department IDs.

**Arguments**: None

**Example LLM prompt**: "List all departments in the company"

---

### namely_list_jobs

**What it does**: Lists all job postings.

**When to use**: View open positions, recruiting management.

**Arguments**:
- `status` (optional): Filter by status (open, closed)

**Example LLM prompt**: "List all open job postings"

---

### namely_list_time_off

**What it does**: Lists time off requests for employees.

**When to use**: Review PTO requests, manage time off calendar.

**Arguments**:
- `employee_id` (optional): Filter by employee ID
- `status` (optional): Filter by status (pending, approved, denied)

**Example LLM prompt**: "Show all pending time off requests"

---

### namely_create_time_off

**What it does**: Creates a new time off request.

**When to use**: Submit PTO requests, schedule time off.

**Arguments**:
- `employee_id` (required): Employee ID
- `start_date` (required): Start date (YYYY-MM-DD)
- `end_date` (required): End date (YYYY-MM-DD)
- `type` (required): Time off type (vacation, sick, personal)

**Example LLM prompt**: "Create a vacation request for employee 123 from June 1-5"

---

### namely_list_benefits

**What it does**: Lists benefits enrollment for employees.

**When to use**: Review health insurance, retirement plans.

**Arguments**:
- `employee_id` (optional): Filter by employee ID

**Example LLM prompt**: "List benefits for employee 12345"

---

### namely_list_payroll

**What it does**: Lists payroll data for employees.

**When to use**: View salary information, pay history.

**Arguments**:
- `employee_id` (optional): Filter by employee ID
- `pay_period_start` (optional): Pay period start date

**Example LLM prompt**: "Show payroll data for the current month"

---

### namely_list_company

**What it does**: Gets company information and settings.

**When to use**: View company details, organizational settings.

**Arguments**: None

**Example LLM prompt**: "Get company information"

---

### namely_list_profiles

**What it does**: Lists employee profiles with detailed information.

**When to use**: Browse complete employee directory with profile details.

**Arguments**:
- `department_id` (optional): Filter by department ID

**Example LLM prompt**: "List all profiles in the sales department"

---

## Namely API Notes

- **Authentication**: Uses Personal Access Token instead of OAuth
- **Date format**: ISO 8601 (YYYY-MM-DD) for all dates
- **Pagination**: Use page and per_page parameters for large result sets
- **Employee IDs**: Numeric IDs used throughout the API
