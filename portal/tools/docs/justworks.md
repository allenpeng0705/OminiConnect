# Justworks Tools

Provider: `justworks` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Justworks API. They allow AI agents to manage employees, companies, payrolls, time off, and benefits. **Requires Justworks OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with Justworks
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://public-api.justworks.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `justworks_list_employees` | List employees | GET | /public/v1/employees |
| `justworks_get_employee` | Get employee details | GET | /public/v1/employees/{employee_id} |
| `justworks_list_companies` | List companies | GET | /public/v1/companies |
| `justworks_get_company` | Get company details | GET | /public/v1/companies/{company_id} |
| `justworks_list_payrolls` | List payrolls | GET | /public/v1/payrolls |
| `justworks_get_payroll` | Get payroll details | GET | /public/v1/payrolls/{payroll_id} |
| `justworks_list_time_off` | List time off requests | GET | /public/v1/time_off_requests |
| `justworks_get_time_off` | Get time off details | GET | /public/v1/time_off_requests/{time_off_id} |
| `justworks_list_benefits` | List benefits | GET | /public/v1/benefits |
| `justworks_list_jobs` | List jobs | GET | /public/v1/jobs |

---

## Tool Details

### justworks_list_employees

**What it does**: Lists all employees in the organization.

**When to use**: Find employees, view employee list.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all employees in Justworks"

---

### justworks_get_employee

**What it does**: Gets details for a specific employee.

**When to use**: Get employee information, view employee profile.

**Arguments**:
- `employee_id` (required): Employee ID

**Example LLM prompt**: "Get details for employee 12345"

---

### justworks_list_companies

**What it does**: Lists all companies.

**When to use**: View companies, find company info.

**Arguments**: None

**Example LLM prompt**: "List all companies in Justworks"

---

### justworks_get_company

**What it does**: Gets details for a specific company.

**When to use**: Get company information.

**Arguments**:
- `company_id` (required): Company ID

**Example LLM prompt**: "Get details for company 1"

---

### justworks_list_payrolls

**What it does**: Lists all payrolls.

**When to use**: View payroll history, track payments.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all payrolls in Justworks"

---

### justworks_get_payroll

**What it does**: Gets details for a specific payroll.

**When to use**: Get payroll information.

**Arguments**:
- `payroll_id` (required): Payroll ID

**Example LLM prompt**: "Get details for payroll 67890"

---

### justworks_list_time_off

**What it does**: Lists all time off requests.

**When to use**: View time off, track PTO.

**Arguments**:
- `status` (optional): Filter by status
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all time off requests"

---

### justworks_get_time_off

**What it does**: Gets details for a specific time off request.

**When to use**: Get time off information.

**Arguments**:
- `time_off_id` (required): Time Off Request ID

**Example LLM prompt**: "Get details for time off request 111"

---

### justworks_list_benefits

**What it does**: Lists all benefits.

**When to use**: View benefits, find benefit information.

**Arguments**: None

**Example LLM prompt**: "List all benefits in Justworks"

---

### justworks_list_jobs

**What it does**: Lists all jobs (positions).

**When to use**: View open positions, list job titles.

**Arguments**: None

**Example LLM prompt**: "List all jobs in Justworks"

---

## Justworks API Notes

- **IDs**: Numeric IDs for employees, companies, payrolls, time off
- **Pagination**: Use page and per_page parameters
- **HR Management**: Covers payroll, benefits, time off, and employee management
