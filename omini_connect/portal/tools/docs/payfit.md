# Payfit Tools

Provider: `payfit` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Payfit API. They allow AI agents to manage employees, payroll runs, time off requests, departments, contracts, benefits, and pay conditions. **Requires Payfit OAuth2 authentication.**

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Payfit
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://partner-api.payfit.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `payfit_list_employees` | List employees | GET | /employees |
| `payfit_get_employee` | Get employee details | GET | /employees/{employeeId} |
| `payfit_list_payroll_runs` | List payroll runs | GET | /payroll/runs |
| `payfit_get_payroll_run` | Get payroll run details | GET | /payroll/runs/{runId} |
| `payfit_list_time_off` | List time off requests | GET | /time-off/requests |
| `payfit_list_departments` | List departments | GET | /departments |
| `payfit_list_contracts` | List employment contracts | GET | /contracts |
| `payfit_list_benefits` | List benefit plans | GET | /benefits |
| `payfit_list_pay_conditions` | List pay conditions | GET | /pay-conditions |
| `payfit_get_company_info` | Get company information | GET | /company |

---

## Tool Details

### payfit_list_employees

**What it does**: Lists all employees in the organization.

**When to use**: Browse employee directory, find employees.

**Arguments**:
- `departmentId` (optional): Filter by department
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "List all active employees"

---

### payfit_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: Get employee details, employment information.

**Arguments**:
- `employeeId` (required): Employee ID

**Example LLM prompt**: "Get details for employee 12345"

---

### payfit_list_payroll_runs

**What it does**: Lists all payroll runs for the organization.

**When to use**: Review payroll history, check status.

**Arguments**:
- `status` (optional): Filter by status
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show payroll runs from this month"

---

### payfit_get_payroll_run

**What it does**: Gets detailed information about a specific payroll run.

**When to use**: Review payroll details, totals.

**Arguments**:
- `runId` (required): Payroll run ID

**Example LLM prompt**: "Get details for payroll run 67890"

---

### payfit_list_time_off

**What it does**: Lists time off requests for employees.

**When to use**: Review time off requests, approvals.

**Arguments**:
- `employeeId` (optional): Filter by employee
- `status` (optional): Filter by status (pending, approved, denied)

**Example LLM prompt**: "Show pending time off requests"

---

### payfit_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: Browse organizational structure.

**Arguments**: None

**Example LLM prompt**: "What departments do we have?"

---

### payfit_list_contracts

**What it does**: Lists all employment contracts in the organization.

**When to use**: Review employment terms, contract details.

**Arguments**:
- `employeeId` (optional): Filter by employee

**Example LLM prompt**: "Show contracts for employees"

---

### payfit_list_benefits

**What it does**: Lists all benefit plans available in the organization.

**When to use**: Review benefits offerings.

**Arguments**:
- `benefitType` (optional): Filter by benefit type

**Example LLM prompt**: "What health plans are offered?"

---

### payfit_list_pay_conditions

**What it does**: Lists all pay conditions (salary, hourly rates, etc.).

**When to use**: Review compensation structure.

**Arguments**: None

**Example LLM prompt**: "What pay conditions are configured?"

---

### payfit_get_company_info

**What it does**: Gets company/organization information.

**When to use**: Get company details, settings.

**Arguments**: None

**Example LLM prompt**: "Get our company information"

---

## Payfit API Notes

- **OAuth2**: Requires user authentication via OAuth flow
- **Rate limits**: Apply to API calls
- **Date formats**: Use YYYY-MM-DD format
