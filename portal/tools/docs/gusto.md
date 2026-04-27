# Gusto Tools

Provider: `gusto` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Gusto API. They allow AI agents to manage employees, contractors, payrolls, and time off in the Gusto payroll and HR platform. Gusto is a modern payroll and HR platform for small businesses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Gusto
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `employees:read`, `employees:write`, `contractors:read`, `payrolls:read`, `payrolls:write`, `time_off:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gusto_list_employees` | List all employees | GET | /employees |
| `gusto_get_employee` | Get employee details | GET | /employees/{employee_id} |
| `gusto_create_employee` | Create new employee | POST | /employees |
| `gusto_list_contractors` | List contractors | GET | /contractors |
| `gusto_get_contractor` | Get contractor details | GET | /contractors/{contractor_id} |
| `gusto_list_payrolls` | List payroll runs | GET | /payrolls |
| `gusto_get_payroll` | Get payroll details | GET | /payrolls/{payroll_id} |
| `gusto_run_payroll` | Execute payroll run | POST | /payrolls |
| `gusto_list_time_off` | List time off policies/requests | GET | /time_off |
| `gusto_get_time_off_request` | Get time off request details | GET | /time_off/{request_id} |

---

## Tool Details

### gusto_list_employees

**What it does**: Lists all employees with optional filters for status, department, or location. Returns names, titles, departments, employment status, and compensation info.

**When to use**: Browse employee roster, find employees by department, check active workforce.

**Arguments**:
- `status` (optional): Filter by status: active, inactive, all (default: active)
- `department` (optional): Filter by department ID
- `location` (optional): Filter by location/region
- `limit` (optional): Number of results
- `page` (optional): Page number

**Example LLM prompt**: "List all active employees in the Engineering department"

---

### gusto_get_employee

**What it does**: Gets detailed employee information including personal details, job profile, compensation, tax withholdings, direct deposit info, and employment history.

**When to use**: Get full employee profile, review pay details and tax information.

**Arguments**:
- `employee_id` (required): Employee ID

**Example LLM prompt**: "Get details for employee 123 to review their compensation and tax info"

---

### gusto_create_employee

**What it does**: Creates a new employee record with personal info, compensation, and tax forms. New employees can be onboarded with direct deposit and tax withholding setup.

**When to use**: Add new hires to payroll, onboard employees.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (required): Email address
- `hire_date` (required): Hire date YYYY-MM-DD
- `pay_type` (optional): Pay type: salary, hourly
- `pay_amount` (optional): Pay amount based on pay_type
- `department` (optional): Department name
- `job_title` (optional): Job title

**Example LLM prompt**: "Create a new employee for John Doe starting March 1st with salary pay type"

---

### gusto_list_contractors

**What it does**: Lists all contractors with optional filters for status or company. Returns contractor names, companies, rates, and payment status.

**When to use**: Browse contractor list, track contractor relationships, check payment status.

**Arguments**:
- `status` (optional): Filter by status: active, inactive, all (default: active)
- `limit` (optional): Number of results
- `page` (optional): Page number

**Example LLM prompt**: "List all active contractors"

---

### gusto_get_contractor

**What it does**: Gets detailed contractor information including company, contact details, payment rates, tax status, and recent payment history.

**When to use**: Review contractor details, check payment rates and tax classification.

**Arguments**:
- `contractor_id` (required): Contractor ID

**Example LLM prompt**: "Get details for contractor 456 to review their payment rate"

---

### gusto_list_payrolls

**What it does**: Lists all payroll runs with optional filters for status, employee, or date range. Returns payroll totals, status, approval info, and payment dates.

**When to use**: Track payroll history, find specific payroll runs, review payroll expenses.

**Arguments**:
- `status` (optional): Filter by status: draft, pending, running, processed, failed
- `employee_id` (optional): Filter by employee ID
- `start_date` (optional): Start date YYYY-MM-DD
- `end_date` (optional): End date YYYY-MM-DD
- `limit` (optional): Number of results
- `page` (optional): Page number

**Example LLM prompt**: "List all payrolls from the last quarter"

---

### gusto_get_payroll

**What it does**: Gets detailed information about a specific payroll run including employee breakdown, tax withholdings, deductions, and payment totals.

**When to use**: Review payroll details, verify compensation and deductions before finalizing.

**Arguments**:
- `payroll_id` (required): Payroll ID

**Example LLM prompt**: "Get details for payroll 789 to see the breakdown of taxes and deductions"

---

### gusto_run_payroll

**What it does**: Executes a payroll run for the specified pay period. This processes employee compensation, taxes, deductions, and sets up direct deposits. Must be used after reviewing and approving the payroll preview.

**When to use**: Process payroll for pay period, pay employees.

**Arguments**:
- `pay_period_start` (required): Pay period start date YYYY-MM-DD
- `pay_period_end` (required): Pay period end date YYYY-MM-DD
- `employees` (optional): Array of employee IDs to include (default all active)

**Example LLM prompt**: "Run payroll for January 1-15, 2024 for all active employees"

---

### gusto_list_time_off

**What it does**: Lists time off policies and/or requests with optional filters. Returns time off types, accrual rates, balances, and pending requests.

**When to use**: Review time off balances, track PTO usage, find pending requests.

**Arguments**:
- `employee_id` (optional): Filter by employee ID
- `status` (optional): Filter by request status: pending, approved, denied
- `policy_type` (optional): Filter by policy type: vacation, sick, personal
- `start_date` (optional): Filter requests from this date
- `end_date` (optional): Filter requests until this date

**Example LLM prompt**: "List all pending time off requests for this week"

---

### gusto_get_time_off_request

**What it does**: Gets detailed information about a specific time off request including dates, hours, type, approval status, and notes.

**When to use**: Review specific time off request details, check approval status.

**Arguments**:
- `request_id` (required): Time off request ID

**Example LLM prompt**: "Get details for time off request 321 to see the approval history"

---

## Gusto API Notes

- **Employee Status**: active employees are in the current workforce, inactive are former employees
- **Pay Types**: salary (fixed amount per pay period) or hourly (based on hours worked)
- **Contractors**: Independent contractors who receive 1099 forms - separate from employees
- **Payroll Status**: draft -> pending -> running -> processed (or failed) - check status before retrying failed payrolls
- **Time Off**: Includes vacation, sick, and personal time off with accrual-based or unlimited policies
- **Direct Deposit**: Employees can have multiple bank accounts for direct deposit distribution
- **Tax Withholding**: Federal and state tax withholdings are calculated based on W-4 information
