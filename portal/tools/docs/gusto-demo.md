# Gusto Demo Tools

Provider: `gusto-demo` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Gusto API (Demo environment). They allow AI agents to manage employees, companies, payrolls, and time off requests. Gusto is a human resources and payroll platform.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Gusto (Demo)
- Token stored in Nango, accessed via `connection_ref`
- Authorization URL: https://api.gusto-demo.com/oauth/authorize
- Token URL: https://api.gusto-demo.com/oauth/token

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gusto_list_employees` | List employees | GET | /v1/employees |
| `gusto_get_employee` | Get employee details | GET | /v1/employees/{id} |
| `gusto_list_companies` | List companies | GET | /v1/companies |
| `gusto_get_company` | Get company details | GET | /v1/companies/{id} |
| `gusto_list_payrolls` | List payrolls | GET | /v1/payrolls |
| `gusto_get_payroll` | Get payroll details | GET | /v1/payrolls/{id} |
| `gusto_list_time_off` | List time off requests | GET | /v1/time_off_requests |
| `gusto_get_time_off` | Get time off details | GET | /v1/time_off_requests/{id} |
| `gusto_list_departments` | List departments | GET | /v1/departments |
| `gusto_list_locations` | List locations | GET | /v1/locations |

---

## Tool Details

### gusto_list_employees

**What it does**: Lists all employees in your Gusto account.

**When to use**: Browse employee roster, find specific employees.

**Arguments**:
- `company_id` (optional): Filter by company ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all employees"

---

### gusto_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: View employee profile and compensation details.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get employee with ID 123"

---

### gusto_list_companies

**What it does**: Lists all companies in your Gusto account.

**When to use**: View company structure.

**Arguments**: None

**Example LLM prompt**: "List all companies"

---

### gusto_get_company

**What it does**: Gets detailed information about a specific company.

**When to use**: View company settings and details.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get company with ID 456"

---

### gusto_list_payrolls

**What it does**: Lists all payrolls in your Gusto account.

**When to use**: View payroll history.

**Arguments**:
- `company_id` (optional): Filter by company ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all payrolls"

---

### gusto_get_payroll

**What it does**: Gets detailed information about a specific payroll.

**When to use**: View payroll details and compensation.

**Arguments**:
- `id` (required): Payroll ID

**Example LLM prompt**: "Get payroll with ID 789"

---

### gusto_list_time_off

**What it does**: Lists all time off requests in your Gusto account.

**When to use**: View time off requests and statuses.

**Arguments**:
- `employee_id` (optional): Filter by employee ID
- `status` (optional): Filter by status (`approved`, `denied`, `pending`)

**Example LLM prompt**: "List all pending time off requests"

---

### gusto_get_time_off

**What it does**: Gets detailed information about a specific time off request.

**When to use**: View time off details and approval status.

**Arguments**:
- `id` (required): Time off request ID

**Example LLM prompt**: "Get time off request with ID 101"

---

### gusto_list_departments

**What it does**: Lists all departments in your Gusto account.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### gusto_list_locations

**What it does**: Lists all locations in your Gusto account.

**When to use**: View company locations.

**Arguments**: None

**Example LLM prompt**: "List all locations"

---

## Gusto Demo API Notes

- **API Base URL**: https://api.gusto-demo.com
- **Auth Mode**: OAuth2
- **Environment**: Demo/Sandbox environment
- **Employees**: Full-time and part-time employees
- **Companies**: Employer companies using Gusto
- **Payrolls**: Payroll runs with compensation details
- **Time Off**: PTO, sick leave, and other time off requests
- **Departments**: Organizational units within companies
- **Locations**: Physical work locations
