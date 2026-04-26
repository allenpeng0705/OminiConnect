# Remote Tools

Provider: `remote` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Remote API. They allow AI agents to manage contractors, employees, payrolls, and time off. Remote is a global HR platform for onboarding, paying, and managing employees worldwide.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Remote
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `contractors:read`, `contractors:write`, `employees:read`, `employees:write`, `payrolls:read`, `time_off:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `remote_list_contractors` | List contractors | GET | /v1/contractors |
| `remote_get_contractor` | Get a specific contractor | GET | /v1/contractors/{contractor_id} |
| `remote_create_contractor` | Create a new contractor | POST | /v1/contractors |
| `remote_list_employees` | List employees | GET | /v1/employees |
| `remote_get_employee` | Get a specific employee | GET | /v1/employees/{employee_id} |
| `remote_create_employee` | Create a new employee | POST | /v1/employees |
| `remote_list_payrolls` | List payroll runs | GET | /v1/payrolls |
| `remote_get_payroll` | Get a specific payroll run | GET | /v1/payrolls/{payroll_id} |
| `remote_list_time_off` | List time off requests | GET | /v1/time_off |
| `remote_get_time_off_request` | Get a specific time off request | GET | /v1/time_off/{request_id} |

---

## Tool Details

### remote_list_contractors

**What it does**: Lists all contractors in the Remote platform with profiles, contract terms, and payment rates.

**When to use**: Manage the contractor workforce and track independent relationships.

**Arguments**:
- `status` (optional): Filter by status (active, inactive)
- `country` (optional): Filter by country code
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all active contractors based in India"

---

### remote_get_contractor

**What it does**: Gets details of a specific contractor including profile, contract terms, and payment details.

**When to use**: Review contractor details before approving payments.

**Arguments**:
- `contractor_id` (required): Contractor ID

**Example LLM prompt**: "Get details for contractor con-123"

---

### remote_create_contractor

**What it does**: Creates a new contractor record with personal details, contract terms, and payment preferences.

**When to use**: Onboard new contractors to the platform.

**Arguments**:
- `first_name` (required): Contractor first name
- `last_name` (required): Contractor last name
- `email` (required): Contractor email address
- `country` (required): Country code (e.g., US, GB, DE)
- `rate` (optional): Contractor rate (hourly or fixed amount)
- `rate_type` (optional): Rate type (hourly, fixed)
- `currency` (optional): Payment currency code (default USD)
- `start_date` (optional): Contract start date (YYYY-MM-DD)

**Example LLM prompt**: "Add a new contractor for Jane Doe in Canada at $75/hour"

---

### remote_list_employees

**What it does**: Lists all employees in the Remote platform with profiles, status, location, and benefits info.

**When to use**: Manage the full-time workforce across global operations.

**Arguments**:
- `status` (optional): Filter by status (active, inactive, onboarding)
- `country` (optional): Filter by country code
- `department` (optional): Filter by department
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all active employees on the Engineering team"

---

### remote_get_employee

**What it does**: Gets details of a specific employee including profile, employment terms, payroll, and benefits.

**When to use**: Review employee details before making changes.

**Arguments**:
- `employee_id` (required): Employee ID

**Example LLM prompt**: "Get details for employee emp-456"

---

### remote_create_employee

**What it does**: Creates a new employee record with personal details, employment terms, location, and payroll config.

**When to use**: Onboard new full-time employees globally.

**Arguments**:
- `first_name` (required): Employee first name
- `last_name` (required): Employee last name
- `email` (required): Employee email address
- `country` (required): Country code (e.g., US, GB, DE)
- `employment_type` (optional): Employment type (full_time, part_time)
- `start_date` (optional): Employment start date (YYYY-MM-DD)
- `department` (optional): Department name
- `compensation` (optional): Compensation details

**Example LLM prompt**: "Create a new employee for Maria Garcia in Spain on the Finance team"

---

### remote_list_payrolls

**What it does**: Lists all payroll runs with periods, status, totals, and worker counts.

**When to use**: Track payroll cycles, monitor payments, and manage global payroll operations.

**Arguments**:
- `status` (optional): Filter by status (draft, processing, completed, failed)
- `period_start` (optional): Payroll period start date (YYYY-MM-DD)
- `period_end` (optional): Payroll period end date (YYYY-MM-DD)
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "Show all completed payrolls from the last month"

---

### remote_get_payroll

**What it does**: Gets details of a specific payroll run including period, total amounts, worker breakdown, and status.

**When to use**: Review payroll details and verify earnings and deductions.

**Arguments**:
- `payroll_id` (required): Payroll run ID

**Example LLM prompt**: "Get details for payroll pr-789"

---

### remote_list_time_off

**What it does**: Lists all time off requests with details, leave types, status, and employee info.

**When to use**: Track leave, manage approvals, and monitor time off balances.

**Arguments**:
- `status` (optional): Filter by status (pending, approved, rejected)
- `employee_id` (optional): Filter by employee ID
- `leave_type` (optional): Filter by type (vacation, sick, personal)
- `start_date` (optional): Start date filter (YYYY-MM-DD)
- `end_date` (optional): End date filter (YYYY-MM-DD)
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all pending time off requests for vacation"

---

### remote_get_time_off_request

**What it does**: Gets details of a specific time off request including employee, dates, leave type, status, and balance impact.

**When to use**: Review time off details before approving or rejecting.

**Arguments**:
- `request_id` (required): Time off request ID

**Example LLM prompt**: "Get details for time off request req-101"

---

## Remote API Notes

- **Global Workforce**: Remote supports employees and contractors in 180+ countries
- **Contractor Rate Types**: hourly (time-based) or fixed (project-based)
- **Employee Types**: full_time, part_time with different compliance requirements
- **Location**: Employee location determines payroll, tax, and compliance requirements
- **Time Off Types**: vacation, sick, personal, parental, etc.
- **Payroll Processing**: Typically run monthly, with support for multiple pay frequencies
