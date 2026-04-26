# Paycom Tools

Provider: `paycom` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Paycom API. They allow AI agents to manage employees, payroll runs, time off requests, departments, jobs, benefits, and pay groups. **Requires Paycom Basic Authentication (SID + API Token).**

## Authentication

**Nango Basic Auth**:
- Uses SID as username and API Token as password
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.paycomonline.net

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `paycom_list_employees` | List employees | GET | /api/v1/employees |
| `paycom_get_employee` | Get employee details | GET | /api/v1/employees/{employeeId} |
| `paycom_list_payroll_runs` | List payroll runs | GET | /api/v1/payroll/runs |
| `paycom_get_payroll_run` | Get payroll run details | GET | /api/v1/payroll/runs/{runId} |
| `paycom_list_time_off` | List time off requests | GET | /api/v1/time-off/requests |
| `paycom_list_departments` | List departments | GET | /api/v1/departments |
| `paycom_list_jobs` | List jobs/positions | GET | /api/v1/jobs |
| `paycom_list_pay_groups` | List pay groups | GET | /api/v1/pay-groups |
| `paycom_list_benefits` | List benefit plans | GET | /api/v1/benefits |
| `paycom_get_company_info` | Get company information | GET | /api/v1/company |

---

## Tool Details

### paycom_list_employees

**What it does**: Lists all employees in the organization.

**When to use**: Browse employee directory, find employees.

**Arguments**:
- `departmentId` (optional): Filter by department
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "List all active employees"

---

### paycom_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: Get employee details, employment information.

**Arguments**:
- `employeeId` (required): Employee ID

**Example LLM prompt**: "Get details for employee 12345"

---

### paycom_list_payroll_runs

**What it does**: Lists all payroll runs for the organization.

**When to use**: Review payroll history, check status.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show payroll runs from this month"

---

### paycom_get_payroll_run

**What it does**: Gets detailed information about a specific payroll run.

**When to use**: Review payroll details, totals.

**Arguments**:
- `runId` (required): Payroll run ID

**Example LLM prompt**: "Get details for payroll run 67890"

---

### paycom_list_time_off

**What it does**: Lists time off requests for employees.

**When to use**: Review time off requests, approvals.

**Arguments**:
- `employeeId` (optional): Filter by employee
- `status` (optional): Filter by status (pending, approved, denied)

**Example LLM prompt**: "Show pending time off requests"

---

### paycom_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: Browse organizational structure.

**Arguments**: None

**Example LLM prompt**: "What departments do we have?"

---

### paycom_list_jobs

**What it does**: Lists all jobs/positions in the organization.

**When to use**: View job listings, positions.

**Arguments**: None

**Example LLM prompt**: "List all open positions"

---

### paycom_list_pay_groups

**What it does**: Lists all pay groups configured in the organization.

**When to use**: Understand pay scheduling.

**Arguments**: None

**Example LLM prompt**: "What pay groups are configured?"

---

### paycom_list_benefits

**What it does**: Lists all benefit plans available in the organization.

**When to use**: Review benefits offerings.

**Arguments**:
- `benefitType` (optional): Filter by benefit type

**Example LLM prompt**: "What health plans are offered?"

---

### paycom_get_company_info

**What it does**: Gets company/organization information.

**When to use**: Get company details, settings.

**Arguments**: None

**Example LLM prompt**: "Get our company information"

---

## Paycom API Notes

- **Basic Auth**: Uses SID and API Token for authentication
- **Rate limits**: Apply to API calls
- **Date formats**: Use YYYY-MM-DD format
