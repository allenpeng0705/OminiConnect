# Paycor Sandbox Tools

Provider: `paycor-sandbox` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Paycor Sandbox API. They allow AI agents to manage employees, payroll runs, time entries, departments, jobs, benefits, and pay groups in a sandbox environment. **Requires Paycor Sandbox OAuth2 authentication.**

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Paycor Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://apis-sandbox.paycor.com
- Requires subscription key in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `paycor_sandbox_list_employees` | List employees | GET | /v1/employees |
| `paycor_sandbox_get_employee` | Get employee details | GET | /v1/employees/{employeeId} |
| `paycor_sandbox_list_payroll_runs` | List payroll runs | GET | /v1/payroll/runs |
| `paycor_sandbox_get_payroll_run` | Get payroll run details | GET | /v1/payroll/runs/{runId} |
| `paycor_sandbox_list_time_entries` | List time entries | GET | /v1/time/entries |
| `paycor_sandbox_list_departments` | List departments | GET | /v1/departments |
| `paycor_sandbox_list_jobs` | List jobs/positions | GET | /v1/jobs |
| `paycor_sandbox_list_benefits` | List benefit plans | GET | /v1/benefits |
| `paycor_sandbox_list_pay_groups` | List pay groups | GET | /v1/pay-groups |
| `paycor_sandbox_get_company_info` | Get company information | GET | /v1/company |

---

## Tool Details

### paycor_sandbox_list_employees

**What it does**: Lists all employees in the organization.

**When to use**: Browse employee directory, find employees.

**Arguments**:
- `departmentId` (optional): Filter by department
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "List all active employees"

---

### paycor_sandbox_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: Get employee details, employment information.

**Arguments**:
- `employeeId` (required): Employee ID

**Example LLM prompt**: "Get details for employee 12345"

---

### paycor_sandbox_list_payroll_runs

**What it does**: Lists all payroll runs for the organization.

**When to use**: Review payroll history, check status.

**Arguments**:
- `status` (optional): Filter by status
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show payroll runs from this month"

---

### paycor_sandbox_get_payroll_run

**What it does**: Gets detailed information about a specific payroll run.

**When to use**: Review payroll details, totals.

**Arguments**:
- `runId` (required): Payroll run ID

**Example LLM prompt**: "Get details for payroll run 67890"

---

### paycor_sandbox_list_time_entries

**What it does**: Lists time entries for employees.

**When to use**: Review time tracking, timesheet approval.

**Arguments**:
- `employeeId` (optional): Filter by employee
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show time entries for last week"

---

### paycor_sandbox_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: Browse organizational structure.

**Arguments**: None

**Example LLM prompt**: "What departments do we have?"

---

### paycor_sandbox_list_jobs

**What it does**: Lists all jobs/positions in the organization.

**When to use**: View job listings, positions.

**Arguments**: None

**Example LLM prompt**: "List all open positions"

---

### paycor_sandbox_list_benefits

**What it does**: Lists all benefit plans available in the organization.

**When to use**: Review benefits offerings.

**Arguments**:
- `benefitType` (optional): Filter by benefit type

**Example LLM prompt**: "What health plans are offered?"

---

### paycor_sandbox_list_pay_groups

**What it does**: Lists all pay groups configured in the organization.

**When to use**: Understand pay scheduling.

**Arguments**: None

**Example LLM prompt**: "What pay groups are configured?"

---

### paycor_sandbox_get_company_info

**What it does**: Gets company/organization information.

**When to use**: Get company details, settings.

**Arguments**: None

**Example LLM prompt**: "Get our company information"

---

## Paycor Sandbox API Notes

- **Sandbox Environment**: Uses sandbox APIs for testing
- **OAuth2**: Requires user authentication via OAuth flow
- **Subscription Key**: Required in connection configuration
- **Rate limits**: Apply to API calls
- **Date formats**: Use YYYY-MM-DD format
