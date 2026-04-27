# Paychex Tools

Provider: `paychex` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Paychex API. They allow AI agents to manage employees, payroll runs, time entries, departments, locations, benefits, and pay groups. **Requires Paychex OAuth2 Client Credentials authentication.**

## Authentication

**Nango OAuth2 CC**:
- Uses client_id and client_secret for client credentials flow
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.paychex.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `paychex_list_employees` | List employees | GET | /v1/employees |
| `paychex_get_employee` | Get employee details | GET | /v1/employees/{employeeId} |
| `paychex_list_payroll_runs` | List payroll runs | GET | /v1/payrollRuns |
| `paychex_get_payroll_run` | Get payroll run details | GET | /v1/payrollRuns/{runId} |
| `paychex_list_time_entries` | List time entries | GET | /v1/timeEntries |
| `paychex_list_departments` | List departments | GET | /v1/departments |
| `paychex_list_locations` | List locations | GET | /v1/locations |
| `paychex_list_benefits` | List benefit plans | GET | /v1/benefits |
| `paychex_list_pay_groups` | List pay groups | GET | /v1/payGroups |
| `paychex_get_company_info` | Get company information | GET | /v1/company |

---

## Tool Details

### paychex_list_employees

**What it does**: Lists all employees in the organization.

**When to use**: Browse employee directory, find employees by criteria.

**Arguments**:
- `page` (optional): Page number (default 1)
- `perPage` (optional): Results per page (default 50)

**Example LLM prompt**: "List all employees in the HR system"

---

### paychex_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: Get employee details, employment status, compensation info.

**Arguments**:
- `employeeId` (required): Employee ID

**Example LLM prompt**: "Get details for employee EMP12345"

---

### paychex_list_payroll_runs

**What it does**: Lists all payroll runs for the organization.

**When to use**: Review payroll history, check payroll status.

**Arguments**:
- `status` (optional): Filter by status (PROCESSED, PENDING)
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show recent payroll runs this month"

---

### paychex_get_payroll_run

**What it does**: Gets detailed information about a specific payroll run.

**When to use**: Review specific payroll details, totals, deductions.

**Arguments**:
- `runId` (required): Payroll run ID

**Example LLM prompt**: "Get details for payroll run PR12345"

---

### paychex_list_time_entries

**What it does**: Lists time entries for employees.

**When to use**: Review employee time tracking, timesheet approval.

**Arguments**:
- `employeeId` (optional): Filter by employee ID
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show time entries for last week"

---

### paychex_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: Browse organizational structure.

**Arguments**: None

**Example LLM prompt**: "What departments are in our organization?"

---

### paychex_list_locations

**What it does**: Lists all work locations for the organization.

**When to use**: View company locations, manage workplace settings.

**Arguments**: None

**Example LLM prompt**: "List all company locations"

---

### paychex_list_benefits

**What it does**: Lists all benefit plans available in the organization.

**When to use**: Review benefits offerings, employee enrollment.

**Arguments**:
- `benefitType` (optional): Filter by type (HEALTH, DENTAL, etc.)

**Example LLM prompt**: "What health benefit plans are offered?"

---

### paychex_list_pay_groups

**What it does**: Lists all pay groups configured in the organization.

**When to use**: Understand pay scheduling, payroll configuration.

**Arguments**: None

**Example LLM prompt**: "What pay groups are configured?"

---

### paychex_get_company_info

**What it does**: Gets company/organization information.

**When to use**: Get company details, settings, configuration.

**Arguments**: None

**Example LLM prompt**: "Get our company information"

---

## Paychex API Notes

- **OAuth2 Client Credentials**: Uses client_id and client_secret for machine-to-machine auth
- **Rate limits**: Apply to API calls — implement backoff for heavy use
- **Date formats**: Use YYYY-MM-DD format for date parameters
