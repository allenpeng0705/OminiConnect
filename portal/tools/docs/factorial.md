# Factorial Tools

Provider: `factorial` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Factorial HR API. They allow AI agents to manage employees, leaves, departments, and attendance. Factorial is an HR platform for small and medium businesses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Factorial
- Token stored in Nango, accessed via `connection_ref`
- OAuth2 without PKCE

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `factorial_list_employees` | List employees | GET | /api/v1/employees |
| `factorial_get_employee` | Get employee details | GET | /api/v1/employees/{id} |
| `factorial_list_leaves` | List leave requests | GET | /api/v1/leaves |
| `factorial_get_leave` | Get leave details | GET | /api/v1/leaves/{id} |
| `factorial_list_departments` | List departments | GET | /api/v1/departments |
| `factorial_list_positions` | List positions | GET | /api/v1/positions |
| `factorial_list_companies` | List companies | GET | /api/v1/companies |
| `factorial_get_company` | Get company details | GET | /api/v1/companies/{id} |
| `factorial_list_contracts` | List contracts | GET | /api/v1/contracts |
| `factorial_list_attendances` | List attendances | GET | /api/v1/attendances |

---

## Tool Details

### factorial_list_employees

**What it does**: Lists all employees in the organization.

**When to use**: Browse employee directory, find specific employees.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all employees"

---

### factorial_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: Get employee details, contact info.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get details for employee 123"

---

### factorial_list_leaves

**What it does**: Lists leave requests with filtering.

**When to use**: Review time off requests, track absences.

**Arguments**:
- `status` (optional): Filter by status (pending, approved, rejected)
- `employee_id` (optional): Filter by employee

**Example LLM prompt**: "List all pending leave requests"

---

### factorial_get_leave

**What it does**: Gets details of a specific leave request.

**When to use**: Review leave details.

**Arguments**:
- `id` (required): Leave ID

**Example LLM prompt**: "Get details for leave 456"

---

### factorial_list_departments

**What it does**: Lists all departments.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### factorial_list_positions

**What it does**: Lists all positions/job titles.

**When to use**: See job roles in the company.

**Arguments**: None

**Example LLM prompt**: "List all positions"

---

### factorial_list_companies

**What it does**: Lists all companies.

**When to use**: View company structure.

**Arguments**: None

**Example LLM prompt**: "List my companies"

---

### factorial_get_company

**What it does**: Gets details of a specific company.

**When to use**: Get company information.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get details for company abc123"

---

### factorial_list_contracts

**What it does**: Lists employment contracts.

**When to use**: View contract details, track employment terms.

**Arguments**:
- `employee_id` (optional): Filter by employee

**Example LLM prompt**: "List contracts for employee 123"

---

### factorial_list_attendances

**What it does**: Lists attendance records.

**When to use**: Track work hours, verify attendance.

**Arguments**:
- `employee_id` (optional): Filter by employee
- `date_from` (optional): Start date (YYYY-MM-DD)
- `date_to` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "List attendances for the last week"

---

## Factorial API Notes

- **Employees**: Core HR entity with contact and job info
- **Leaves**: Time off requests with approval workflow
- **Departments**: Organizational units
- **Positions**: Job titles and roles
- **Contracts**: Employment terms and conditions
- **Attendances**: Clock-in/out records and work hours
