# Freshteam Tools

Provider: `freshteam` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Freshteam HR API. They allow AI agents to manage employees, departments, job postings, holidays, and time-off requests. **Requires Freshteam API key.**

## Authentication

**Nango API_KEY**:
- User provides their Freshteam API key and account name
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{accountName}.freshteam.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `freshteam_list_employees` | List employees | GET | /employees |
| `freshteam_get_employee` | Get employee details | GET | /employees/{id} |
| `freshteam_create_employee` | Create an employee | POST | /employees |
| `freshteam_update_employee` | Update an employee | PUT | /employees/{id} |
| `freshteam_list_departments` | List departments | GET | /departments |
| `freshteam_get_department` | Get department details | GET | /departments/{id} |
| `freshteam_list_job_postings` | List job postings | GET | /job_postings |
| `freshteam_get_job_posting` | Get job posting details | GET | /job_postings/{id} |
| `freshteam_list_holidays` | List holidays | GET | /holidays |
| `freshteam_list_time_off_requests` | List time off requests | GET | /time_off_requests |

---

## Tool Details

### freshteam_list_employees

**What it does**: Lists all employees in the organization with optional filters.

**When to use**: Find employees, search by department, or get an overview of the workforce.

**Arguments**:
- `department_id` (optional): Filter by department ID
- `status` (optional): Filter by status (active, inactive, all)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all employees in the engineering department"

---

### freshteam_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: Get employee details like contact info, department, or job title.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get details for employee with ID 12345"

---

### freshteam_create_employee

**What it does**: Creates a new employee in Freshteam.

**When to use**: Add new hires to the HR system.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (required): Work email
- `department_id` (optional): Department ID
- `designation` (optional): Job title
- `date_of_joining` (optional): Join date (YYYY-MM-DD)

**Example LLM prompt**: "Create a new employee named John Doe with email john@company.com in the engineering department"

---

### freshteam_update_employee

**What it does**: Updates an existing employee's information.

**When to use**: Modify employee details when roles or information change.

**Arguments**:
- `id` (required): Employee ID
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `department_id` (optional): Department ID
- `designation` (optional): Job title

**Example LLM prompt**: "Update the department for employee 12345 to engineering"

---

### freshteam_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: Get an overview of organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments in the company"

---

### freshteam_get_department

**What it does**: Gets detailed information about a specific department.

**When to use**: Get department head, employee count, or department details.

**Arguments**:
- `id` (required): Department ID

**Example LLM prompt**: "Get details for department ID 5"

---

### freshteam_list_job_postings

**What it does**: Lists all job postings with optional status filter.

**When to use**: View open positions or track recruiting pipeline.

**Arguments**:
- `status` (optional): Filter by status (open, closed, all)

**Example LLM prompt**: "List all open job postings"

---

### freshteam_get_job_posting

**What it does**: Gets detailed information about a specific job posting.

**When to use**: Get job description, requirements, or posting status.

**Arguments**:
- `id` (required): Job posting ID

**Example LLM prompt**: "Get details for job posting 456"

---

### freshteam_list_holidays

**What it does**: Lists all holidays configured in the organization.

**When to use**: Plan around holidays, calculate working days.

**Arguments**:
- `year` (optional): Year (defaults to current year)

**Example LLM prompt**: "What holidays are observed this year?"

---

### freshteam_list_time_off_requests

**What it does**: Lists all time off requests with optional filters.

**When to use**: Review PTO requests, track approval status.

**Arguments**:
- `status` (optional): Filter by status (pending, approved, rejected)
- `employee_id` (optional): Filter by employee ID

**Example LLM prompt**: "List all pending time off requests"

---

## Freshteam API Notes

- **Employee IDs**: Numeric IDs assigned by Freshteam
- **Department structure**: Departments can be nested
- **Job postings**: Linked to departments and hiring managers
- **Time off**: Requests have approval workflows
