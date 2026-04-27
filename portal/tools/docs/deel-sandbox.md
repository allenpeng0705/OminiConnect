# Deel Sandbox Tools

Provider: `deel-sandbox` | Engine: `nango` | Auth: OAuth via Nango (Sandbox)

## Overview

These tools wrap the Deel API sandbox environment. They allow AI agents to manage contracts, employees, payments, invoices, and time-off requests. Deel is a global HR and payroll platform.

## Authentication

**Nango OAuth (Sandbox)**:
- User authenticates via Nango Connect with Deel Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Uses sandbox environment (demo.deel.com)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `deel_sandbox_list_contracts` | List contracts | GET | /v1/contracts |
| `deel_sandbox_get_contract` | Get contract details | GET | /v1/contracts/{contract_id} |
| `deel_sandbox_list_employees` | List employees/contractors | GET | /v1/employees |
| `deel_sandbox_get_employee` | Get employee details | GET | /v1/employees/{employee_id} |
| `deel_sandbox_list_payments` | List payments | GET | /v1/payments |
| `deel_sandbox_get_payment` | Get payment details | GET | /v1/payments/{payment_id} |
| `deel_sandbox_list_invoices` | List invoices | GET | /v1/invoices |
| `deel_sandbox_get_invoice` | Get invoice details | GET | /v1/invoices/{invoice_id} |
| `deel_sandbox_list_time_off` | List time off requests | GET | /v1/time-off |
| `deel_sandbox_get_time_off` | Get time off details | GET | /v1/time-off/{time_off_id} |

---

## Tool Details

### deel_sandbox_list_contracts

**What it does**: Lists all contracts in the sandbox environment.

**When to use**: Browse contracts, find active agreements, review contract terms.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `status` (optional): Filter by active, ended, or draft

**Example LLM prompt**: "List all active contracts"

---

### deel_sandbox_get_contract

**What it does**: Gets detailed contract information.

**When to use**: Review contract details, check terms, verify agreement specifics.

**Arguments**:
- `contract_id` (required): Contract ID

**Example LLM prompt**: "Get details for contract c-123"

---

### deel_sandbox_list_employees

**What it does**: Lists all employees and contractors.

**When to use**: View workforce, find specific workers, manage team members.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `type` (optional): Filter by employee or contractor

**Example LLM prompt**: "List all contractors"

---

### deel_sandbox_get_employee

**What it does**: Gets detailed employee/contractor information.

**When to use**: Review worker details, check employment terms, verify payment setup.

**Arguments**:
- `employee_id` (required): Employee ID

**Example LLM prompt**: "Get details for employee e-456"

---

### deel_sandbox_list_payments

**What it does**: Lists all payments in the sandbox.

**When to use**: View payment history, track pending payments, verify payroll.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `status` (optional): Filter by pending, processed, or failed

**Example LLM prompt**: "List all pending payments"

---

### deel_sandbox_get_payment

**What it does**: Gets detailed payment information.

**When to use**: Check payment details, verify transaction, track specific payment.

**Arguments**:
- `payment_id` (required): Payment ID

**Example LLM prompt**: "Get details for payment p-789"

---

### deel_sandbox_list_invoices

**What it does**: Lists all invoices.

**When to use**: Browse invoices, find invoices by status, manage billing.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `status` (optional): Filter by draft, sent, or paid

**Example LLM prompt**: "List all unpaid invoices"

---

### deel_sandbox_get_invoice

**What it does**: Gets detailed invoice information.

**When to use**: Review invoice details, check line items, verify payment terms.

**Arguments**:
- `invoice_id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice i-101"

---

### deel_sandbox_list_time_off

**What it does**: Lists all time off requests.

**When to use**: View PTO requests, track approvals, manage leave.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `status` (optional): Filter by pending, approved, or rejected

**Example LLM prompt**: "List all pending time off requests"

---

### deel_sandbox_get_time_off

**What it does**: Gets detailed time off request information.

**When to use**: Review leave details, check dates, verify coverage.

**Arguments**:
- `time_off_id` (required): Time Off ID

**Example LLM prompt**: "Get details for time off t-202"

---

## Deel Sandbox API Notes

- **Sandbox Only**: This environment is for testing and development
- **Same API Structure**: Identical endpoints to production Deel API
- **Sandbox URLs**: Uses api-sandbox.demo.deel.com
- **Mock Data**: Contains synthetic or test data
- **Contracts**: Employment or service agreements
- **Employees**: Both direct employees and contractors
- **Payments**: Payroll and contractor payments
- **Time Off**: PTO and leave management
