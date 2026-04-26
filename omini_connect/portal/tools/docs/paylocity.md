# Paylocity Tools

Provider: `paylocity` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Paylocity Weblink API. They allow AI agents to manage employees, payroll runs, time entries, departments, jobs, benefits, and pay groups. **Requires Paylocity OAuth2 Client Credentials authentication.**

## Authentication

**Nango OAuth2 CC**:
- Uses client_id and client_secret for client credentials flow
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://{environment}.paylocity.com
- Requires environment and companyId in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `paylocity_list_employees` | List employees | GET | /v1/employees |
| `paylocity_get_employee` | Get employee details | GET | /v1/employees/{employeeId} |
| `paylocity_list_payroll_runs` | List payroll runs | GET | /v1/payroll/runs |
| `paylocity_get_payroll_run` | Get payroll run details | GET | /v1/payroll/runs/{runId} |
| `paylocity_list_time_entries` | List time entries | GET | /v1/time/entries |
| `paylocity_list_departments` | List departments | GET | /v1/departments |
| `paylocity_list_jobs` | List jobs/positions | GET | /v1/jobs |
| `paylocity_list_benefits` | List benefit plans | GET | /v1/benefits |
| `paylocity_list_pay_groups` | List pay groups | GET | /v1/pay-groups |
| `paylocity_get_company_info` | Get company information | GET | /v1/company |

---

## Tool Details

### paylocity_list_employees

**What it does**: Lists all employees in the organization.

**When to use**: Browse employee directory, find employees.

**Arguments**:
- `departmentId` (optional): Filter by department
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "List all active employees"

---

### paylocity_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: Get employee details, employment information.

**Arguments**:
- `employeeId` (required): Employee ID

**Example LLM prompt**: "Get details for employee 12345"

---

### paylocity_list_payroll_runs

**What it does**: Lists all payroll runs for the organization.

**When to use**: Review payroll history, check status.

**Arguments**:
- `status` (optional): Filter by status
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show payroll runs from this month"

---

### paylocity_get_payroll_run

**What it does**: Gets detailed information about a specific payroll run.

**When to use**: Review payroll details, totals.

**Arguments**:
- `runId` (required): Payroll run ID

**Example LLM prompt**: "Get details for payroll run 67890"

---

### paylocity_list_time_entries

**What it does**: Lists time entries for employees.

**When to use**: Review time tracking, timesheet approval.

**Arguments**:
- `employeeId` (optional): Filter by employee
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show time entries for last week"

---

### paylocity_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: Browse organizational structure.

**Arguments**: None

**Example LLM prompt**: "What departments do we have?"

---

### paylocity_list_jobs

**What it does**: Lists all jobs/positions in the organization.

**When to use**: View job listings, positions.

**Arguments**: None

**Example LLM prompt**: "List all open positions"

---

### paylocity_list_benefits

**What it does**: Lists all benefit plans available in the organization.

**When to use**: Review benefits offerings.

**Arguments**:
- `benefitType` (optional): Filter by benefit type

**Example LLM prompt**: "What health plans are offered?"

---

### paylocity_list_pay_groups

**What it does**: Lists all pay groups configured in the organization.

**When to use**: Understand pay scheduling.

**Arguments**: None

**Example LLM prompt**: "What pay groups are configured?"

---

### paylocity_get_company_info

**What it does**: Gets company/organization information.

**When to use**: Get company details, settings.

**Arguments**: None

**Example LLM prompt**: "Get our company information"

---

## Paylocity API Notes

- **OAuth2 Client Credentials**: Uses client_id and client_secret for machine-to-machine auth
- **Environment**: Configurable environment (dc1demogw, api)
- **Rate limits**: Apply to API calls
- **Date formats**: Use YYYY-MM-DD format
