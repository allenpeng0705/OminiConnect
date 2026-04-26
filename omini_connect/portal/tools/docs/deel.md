# Deel Tools

Provider: `deel` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Deel API. They allow AI agents to manage contracts, employees, time off, and invoices. Deel is a global payroll and HR platform for managing contractors and employees worldwide.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Deel
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `contracts:read`, `contracts:write`, `employees:read`, `employees:write`, `time_off:read`, `time_off:write`, `invoices:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `deel_list_contracts` | List contracts | GET | /v1/contracts |
| `deem_get_contract` | Get a specific contract | GET | /v1/contracts/{contract_id} |
| `deel_create_contract` | Create a new contract | POST | /v1/contracts |
| `deel_list_employees` | List employees | GET | /v1/employees |
| `deel_get_employee` | Get a specific employee | GET | /v1/employees/{employee_id} |
| `deel_create_employee` | Create a new employee | POST | /v1/employees |
| `deel_list_time_off` | List time off requests | GET | /v1/time_off |
| `deel_get_time_off_request` | Get a specific time off request | GET | /v1/time_off/{request_id} |
| `deel_approve_time_off` | Approve a time off request | POST | /v1/time_off/{request_id}/approve |
| `deel_list_invoices` | List invoices | GET | /v1/invoices |

---

## Tool Details

### deel_list_contracts

**What it does**: Lists all contracts in the Deel platform with types (full-time, contractor) and status.

**When to use**: Manage the workforce and track contractual relationships.

**Arguments**:
- `status` (optional): Filter by status (active, ended, draft)
- `type` (optional): Filter by type (full_time, contractor)
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all active contractor contracts"

---

### deem_get_contract

**What it does**: Gets details of a specific contract including terms, parties, payment details, and status.

**When to use**: Review contract terms before taking action.

**Arguments**:
- `contract_id` (required): Contract ID

**Example LLM prompt**: "Get details for contract con-123"

---

### deel_create_contract

**What it does**: Creates a new contract for a worker with compensation, type, and terms.

**When to use**: Onboard new contractors or create employment agreements.

**Arguments**:
- `worker_id` (required): Worker ID or candidate ID
- `contract_type` (required): Contract type (full_time, contractor)
- `title` (optional): Job title or contract title
- `compensation` (optional): Compensation details
- `start_date` (optional): Contract start date (YYYY-MM-DD)
- `end_date` (optional): Contract end date (YYYY-MM-DD)

**Example LLM prompt**: "Create a contractor contract for Jane Doe starting next week"

---

### deel_list_employees

**What it does**: Lists all employees in the Deel platform with profiles, employment status, and contracts.

**When to use**: Manage the HR workforce and track employee records.

**Arguments**:
- `status` (optional): Filter by status (active, inactive)
- `employment_type` (optional): Filter by type (full_time, contractor)
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all active full-time employees"

---

### deel_get_employee

**What it does**: Gets details of a specific employee including profile, employment details, and payroll info.

**When to use**: Review employee details before making changes.

**Arguments**:
- `employee_id` (required): Employee ID

**Example LLM prompt**: "Get details for employee emp-456"

---

### deel_create_employee

**What it does**: Creates a new employee record with personal details, employment terms, and payroll setup.

**When to use**: Onboard new employees to the platform.

**Arguments**:
- `first_name` (required): Employee first name
- `last_name` (required): Employee last name
- `email` (required): Employee email address
- `employment_type` (optional): Employment type (full_time, contractor)
- `start_date` (optional): Employment start date (YYYY-MM-DD)
- `compensation` (optional): Compensation details

**Example LLM prompt**: "Add a new employee for John Smith with john@example.com"

---

### deel_list_time_off

**What it does**: Lists all time off requests with types (vacation, sick leave), status, and dates.

**When to use**: Track leave balances and manage time off approvals.

**Arguments**:
- `status` (optional): Filter by status (pending, approved, rejected)
- `employee_id` (optional): Filter by employee ID
- `start_date` (optional): Start date filter (YYYY-MM-DD)
- `end_date` (optional): End date filter (YYYY-MM-DD)
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "Show all pending time off requests this month"

---

### deel_get_time_off_request

**What it does**: Gets details of a specific time off request including employee, dates, leave type, and status.

**When to use**: Review time off request details before approving.

**Arguments**:
- `request_id` (required): Time off request ID

**Example LLM prompt**: "Get details for time off request req-789"

---

### deel_approve_time_off

**What it does**: Approves a pending time off request.

**When to use**: Manage leave approvals from the HR workflow.

**Arguments**:
- `request_id` (required): Time off request ID
- `notes` (optional): Approval notes

**Example LLM prompt**: "Approve time off request req-789"

---

### deel_list_invoices

**What it does**: Lists all invoices with details, amounts, status, and worker information.

**When to use**: Track payments, manage billing, and monitor contractor payments.

**Arguments**:
- `status` (optional): Filter by status (draft, sent, paid, overdue)
- `contract_id` (optional): Filter by contract ID
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all overdue invoices"

---

## Deel API Notes

- **Contract Types**: full_time (employees), contractor (independent workers)
- **Worker ID**: Required for contract creation - can be contractor or employee
- **Time Off Status**: pending -> approved/rejected workflow
- **Invoice Status**: draft -> sent -> paid (or overdue)
- **Global Payroll**: Deel supports payroll in 150+ countries
- **Compensation**: Can include amount, currency, and pay frequency
