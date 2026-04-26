# Ramp Tools

Provider: `ramp` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Ramp is a spend management platform that provides corporate cards, expense management, and financial automation. These tools allow AI agents to manage transactions, cards, employees, and expenses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Ramp
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `transactions:read`, `cards:read`, `employees:read`, `departments:read`, `expenses:write`, `limits:read`, `balance:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ramp_list_transactions` | List all transactions | GET | /v1/transactions |
| `ramp_get_transaction` | Get transaction details | GET | /v1/transactions/{transactionId} |
| `ramp_list_cards` | List company cards | GET | /v1/cards |
| `ramp_get_card` | Get card details | GET | /v1/cards/{cardId} |
| `ramp_list_employees` | List employees | GET | /v1/employees |
| `ramp_get_employee` | Get employee details | GET | /v1/employees/{employeeId} |
| `ramp_list_departments` | List departments | GET | /v1/departments |
| `ramp_create_expense` | Create an expense | POST | /v1/expenses |
| `ramp_list_limits` | List spending limits | GET | /v1/limits |
| `ramp_get_balance` | Get account balance | GET | /v1/balance |

---

## Tool Details

### ramp_list_transactions

**What it does**: Returns a paginated list of all transactions.

**When to use**: Review spending activity, find transactions.

**Arguments**:
- `limit` (optional): Number of transactions (default 50)
- `offset` (optional): Pagination offset
- `status` (optional): Filter by status (pending, posted, declined)

**Example LLM prompt**: "List all transactions from this month"

---

### ramp_get_transaction

**What it does**: Gets details of a specific transaction.

**When to use**: Get transaction details, merchant info, approval status.

**Arguments**:
- `transactionId` (required): The transaction ID

**Example LLM prompt**: "Get details for transaction txn_abc123"

---

### ramp_list_cards

**What it does**: Returns a list of all company cards.

**When to use**: View all issued cards, find cards by employee.

**Arguments**:
- `limit` (optional): Number of cards (default 50)
- `employeeId` (optional): Filter by employee

**Example LLM prompt**: "List all cards assigned to John"

---

### ramp_get_card

**What it does**: Gets details of a specific card.

**When to use**: Check card status, spending limit, associated employee.

**Arguments**:
- `cardId` (required): The card ID

**Example LLM prompt**: "Get details for card card_xyz789"

---

### ramp_list_employees

**What it does**: Returns a list of all employees.

**When to use**: View employee roster, find employee IDs.

**Arguments**:
- `limit` (optional): Number of employees (default 50)

**Example LLM prompt**: "List all employees in Ramp"

---

### ramp_get_employee

**What it does**: Gets details of a specific employee.

**When to use**: Get employee info, department, spending limit.

**Arguments**:
- `employeeId` (required): The employee ID

**Example LLM prompt**: "Get details for employee emp_123"

---

### ramp_list_departments

**What it does**: Returns a list of all departments.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### ramp_create_expense

**What it does**: Creates a new expense record.

**When to use**: Log expenses, submit receipts.

**Arguments**:
- `amount` (required): Expense amount
- `currency` (optional): Currency code (default USD)
- `merchantName` (required): Merchant name
- `description` (optional): Description
- `employeeId` (optional): Employee ID
- `date` (optional): Date (ISO 8601)

**Example LLM prompt**: "Create an expense for $50 at Amazon"

---

### ramp_list_limits

**What it does**: Returns spending limits for cards and employees.

**When to use**: Check available spending limits.

**Arguments**:
- `cardId` (optional): Filter by card ID

**Example LLM prompt**: "List all spending limits"

---

### ramp_get_balance

**What it does**: Gets your Ramp account balance.

**When to use**: Check available funds.

**Arguments**: None

**Example LLM prompt**: "What's my current Ramp balance?"

---

## Ramp API Notes

- Transactions include merchant info, amount, date, and status
- Cards are assigned to employees with spending limits
- Departments organize employees for budget tracking
- All amounts are in the currency specified (default USD)
