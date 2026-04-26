# Papaya Global Tools

Provider: `papaya_global` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Papaya Global API. They allow AI agents to manage employees, payrolls, benefits, and time off policies. Papaya Global is a global HR and payroll platform for managing distributed workforces.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Papaya Global
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `employees:read`, `employees:write`, `payrolls:read`, `payrolls:write`, `benefits:read`, `time_off:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `papaya_list_employees` | List employees | GET | /v1/employees |
| `papaya_get_employee` | Get a specific employee | GET | /v1/employees/{employee_id} |
| `papaya_create_employee` | Create a new employee | POST | /v1/employees |
| `papaya_list_payrolls` | List payroll runs | GET | /v1/payrolls |
| `papaya_get_payroll` | Get a specific payroll run | GET | /v1/payrolls/{payroll_id} |
| `papaya_run_payroll` | Execute a payroll run | POST | /v1/payrolls/run |
| `papaya_list_benefits` | List benefits | GET | /v1/benefits |
| `papaya_get_benefit` | Get a specific benefit | GET | /v1/benefits/{benefit_id} |
| `papaya_list_time_off` | List time off policies | GET | /v1/time_off |
| `papaya_get_time_off_policy` | Get a specific time off policy | GET | /v1/time_off/{policy_id} |

---

## Tool Details

### papaya_list_employees

**What it does**: Lists all employees in the Papaya Global platform with profiles, employment status, and location.

**When to use**: Manage a global workforce and track employee records across countries.

**Arguments**:
- `status` (optional): Filter by status (active, inactive, onboarding)
- `country` (optional): Filter by country code
- `department` (optional): Filter by department
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all active employees in Germany"

---

### papaya_get_employee

**What it does**: Gets details of a specific employee including profile, employment terms, payroll info, and benefits enrollment.

**When to use**: Review full employee details before making changes.

**Arguments**:
- `employee_id` (required): Employee ID

**Example LLM prompt**: "Get details for employee emp-123"

---

### papaya_create_employee

**What it does**: Creates a new employee record with personal details, employment terms, location, and payroll config.

**When to use**: Onboard new employees across multiple countries.

**Arguments**:
- `first_name` (required): Employee first name
- `last_name` (required): Employee last name
- `email` (required): Employee email address
- `country` (required): Country code (e.g., US, GB, DE)
- `employment_type` (optional): Employment type (full_time, part_time, contractor)
- `start_date` (optional): Employment start date (YYYY-MM-DD)
- `department` (optional): Department name

**Example LLM prompt**: "Create a new employee for John Smith in the UK on the Engineering team"

---

### papaya_list_payrolls

**What it does**: Lists all payroll runs with periods, status, total amounts, and worker counts.

**When to use**: Track payroll cycles and manage global payroll operations.

**Arguments**:
- `status` (optional): Filter by status (draft, processing, completed)
- `period_start` (optional): Payroll period start date (YYYY-MM-DD)
- `period_end` (optional): Payroll period end date (YYYY-MM-DD)
- `country` (optional): Filter by country code
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "Show all completed payrolls from Q1 2024"

---

### papaya_get_payroll

**What it does**: Gets details of a specific payroll run including period, total amounts, worker breakdown, and status.

**When to use**: Review payroll details and verify earnings and deductions.

**Arguments**:
- `payroll_id` (required): Payroll run ID

**Example LLM prompt**: "Get details for payroll pr-456"

---

### papaya_run_payroll

**What it does**: Initiates or processes a payroll run for a specific period.

**When to use**: Execute payroll for a pay period.

**Arguments**:
- `period_start` (required): Pay period start date (YYYY-MM-DD)
- `period_end` (required): Pay period end date (YYYY-MM-DD)
- `country` (required): Country code for payroll processing
- `employee_ids` (optional): List of employee IDs to include
- `execute` (optional): Execute the payroll (default false for preview)

**Example LLM prompt**: "Run payroll for February 2024 for UK employees"

---

### papaya_list_benefits

**What it does**: Lists all benefits and benefit plans with types, coverage details, and enrollment info.

**When to use**: Manage employee benefits across global operations.

**Arguments**:
- `employee_id` (optional): Filter by employee ID to see enrolled benefits
- `benefit_type` (optional): Filter by type (health, dental, vision, life)
- `country` (optional): Filter by country code
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all health benefit plans available in the US"

---

### papaya_get_benefit

**What it does**: Gets details of a specific benefit plan including coverage, costs, and enrollment rules.

**When to use**: Review benefit details and check coverage.

**Arguments**:
- `benefit_id` (required): Benefit plan ID

**Example LLM prompt**: "Get details for benefit ben-789"

---

### papaya_list_time_off

**What it does**: Lists all time off policies with details, leave types, accrual rules, and entitlements.

**When to use**: Understand time off accruals and manage leave policies.

**Arguments**:
- `employee_id` (optional): Filter by employee ID
- `leave_type` (optional): Filter by type (vacation, sick, personal)
- `country` (optional): Filter by country code
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all vacation time off policies in the US"

---

### papaya_get_time_off_policy

**What it does**: Gets details of a specific time off policy including rules, accrual rates, and entitlements.

**When to use**: Review policy details before making changes.

**Arguments**:
- `policy_id` (required): Time off policy ID

**Example LLM prompt**: "Get details for time off policy pol-101"

---

## Papaya Global API Notes

- **Global Workforce**: Papaya Global supports employees in 160+ countries
- **Location**: Employee location determines payroll, tax, and compliance requirements
- **Benefits**: Company-level benefit plans that employees can enroll in
- **Time Off Types**: vacation, sick, personal, parental, etc.
- **Payroll Processing**: Typically run monthly, with support for multiple pay frequencies
- **Compliance**: Papaya handles local tax deductions and compliance automatically
