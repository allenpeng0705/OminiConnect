# Ramp-Sandbox Tools

Provider: `ramp-sandbox` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Ramp Sandbox is a testing environment for the Ramp spend management platform. These tools allow AI agents to test transaction, card, employee, and expense operations in a safe sandbox environment.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Ramp Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `transactions:read`, `cards:read`, `employees:read`, `departments:read`, `expenses:write`, `limits:read`, `balance:read`

**Important**: This is the sandbox environment. All operations are test operations and do not affect production data.

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ramp-sandbox_list_transactions` | List all transactions | GET | /v1/transactions |
| `ramp-sandbox_get_transaction` | Get transaction details | GET | /v1/transactions/{transactionId} |
| `ramp-sandbox_list_cards` | List company cards | GET | /v1/cards |
| `ramp-sandbox_get_card` | Get card details | GET | /v1/cards/{cardId} |
| `ramp-sandbox_list_employees` | List employees | GET | /v1/employees |
| `ramp-sandbox_get_employee` | Get employee details | GET | /v1/employees/{employeeId} |
| `ramp-sandbox_list_departments` | List departments | GET | /v1/departments |
| `ramp-sandbox_create_expense` | Create an expense | POST | /v1/expenses |
| `ramp-sandbox_list_limits` | List spending limits | GET | /v1/limits |
| `ramp-sandbox_get_balance` | Get account balance | GET | /v1/balance |

---

## Tool Details

### ramp-sandbox_list_transactions

**What it does**: Returns a paginated list of all transactions.

**When to use**: Test transaction retrieval and filtering.

**Arguments**:
- `limit` (optional): Number of transactions (default 50)
- `offset` (optional): Pagination offset
- `status` (optional): Filter by status (pending, posted, declined)

**Example LLM prompt**: "List all test transactions"

---

### ramp-sandbox_get_transaction

**What it does**: Gets details of a specific transaction.

**When to use**: Test transaction details retrieval.

**Arguments**:
- `transactionId` (required): The transaction ID

**Example LLM prompt**: "Get details for transaction txn_test123"

---

### ramp-sandbox_list_cards

**What it does**: Returns a list of all company cards.

**When to use**: Test card listing and filtering.

**Arguments**:
- `limit` (optional): Number of cards (default 50)
- `employeeId` (optional): Filter by employee

**Example LLM prompt**: "List all test cards"

---

### ramp-sandbox_get_card

**What it does**: Gets details of a specific card.

**When to use**: Test card details retrieval.

**Arguments**:
- `cardId` (required): The card ID

**Example LLM prompt**: "Get details for card card_test456"

---

### ramp-sandbox_list_employees

**What it does**: Returns a list of all employees.

**When to use**: Test employee listing.

**Arguments**:
- `limit` (optional): Number of employees (default 50)

**Example LLM prompt**: "List all test employees"

---

### ramp-sandbox_get_employee

**What it does**: Gets details of a specific employee.

**When to use**: Test employee details retrieval.

**Arguments**:
- `employeeId` (required): The employee ID

**Example LLM prompt**: "Get details for employee emp_test789"

---

### ramp-sandbox_list_departments

**What it does**: Returns a list of all departments.

**When to use**: Test department listing.

**Arguments**: None

**Example LLM prompt**: "List all test departments"

---

### ramp-sandbox_create_expense

**What it does**: Creates a new expense record in sandbox.

**When to use**: Test expense creation.

**Arguments**:
- `amount` (required): Expense amount
- `currency` (optional): Currency code (default USD)
- `merchantName` (required): Merchant name
- `description` (optional): Description
- `employeeId` (optional): Employee ID
- `date` (optional): Date (ISO 8601)

**Example LLM prompt**: "Create a test expense for $25 at Test Merchant"

---

### ramp-sandbox_list_limits

**What it does**: Returns spending limits for cards and employees.

**When to use**: Test limit retrieval.

**Arguments**:
- `cardId` (optional): Filter by card ID

**Example LLM prompt**: "List all test spending limits"

---

### ramp-sandbox_get_balance

**What it does**: Gets your Ramp sandbox account balance.

**When to use**: Test balance retrieval.

**Arguments**: None

**Example LLM prompt**: "What's my test balance?"

---

## Ramp Sandbox Notes

- This is a TEST environment - data does not affect production
- All IDs are sandbox-specific and not valid in production
- Use for testing integrations and workflows
- May have limited functionality compared to production
