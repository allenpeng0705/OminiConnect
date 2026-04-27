# Sap-concur Tools

Provider: `sap-concur` | Engine: `nango` | Auth: OAuth via Nango

## Overview

SAP Concur is a travel and expense management platform. These tools allow AI agents to manage expenses, expense reports, approvals, and expense policies.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with SAP Concur
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `expenses:read`, `expenses:write`, `reports:read`, `reports:write`, `users:read`, `approvals:read`, `policies:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sap-concur_list_expenses` | List all expenses | GET | /v1/expenses |
| `sap-concur_get_expense` | Get expense details | GET | /v1/expenses/{expenseId} |
| `sap-concur_create_expense` | Create an expense | POST | /v1/expenses |
| `sap-concur_list_reports` | List expense reports | GET | /v1/reports |
| `sap-concur_get_report` | Get report details | GET | /v1/reports/{reportId} |
| `sap-concur_create_report` | Create an expense report | POST | /v1/reports |
| `sap-concur_submit_report` | Submit a report for approval | POST | /v1/reports/{reportId}/submit |
| `sap-concur_list_users` | List employees | GET | /v1/users |
| `sap-concur_get_approval_chain` | Get approval chain | GET | /v1/reports/{reportId}/approvals |
| `sap-concur_get_policy` | Get expense policy | GET | /v1/policies |

---

## Tool Details

### sap-concur_list_expenses

**What it does**: Returns a list of all expenses.

**When to use**: View expense history.

**Arguments**:
- `limit` (optional): Number of expenses (default 50)
- `userId` (optional): Filter by user

**Example LLM prompt**: "List all my expenses"

---

### sap-concur_get_expense

**What it does**: Gets details of a specific expense.

**When to use**: Check expense details.

**Arguments**:
- `expenseId` (required): The expense ID

**Example LLM prompt**: "Get details for expense exp_abc123"

---

### sap-concur_create_expense

**What it does**: Creates a new expense entry.

**When to use**: Log a new expense.

**Arguments**:
- `expenseType` (required): Expense type
- `amount` (required): Expense amount
- `currency` (optional): Currency code
- `description` (optional): Description
- `date` (optional): Expense date

**Example LLM prompt**: "Create an expense of $50 for meals"

---

### sap-concur_list_reports

**What it does**: Returns a list of expense reports.

**When to use**: View expense reports.

**Arguments**:
- `limit` (optional): Number of reports (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending reports"

---

### sap-concur_get_report

**What it does**: Gets details of a specific report.

**When to use**: Get report details and expenses.

**Arguments**:
- `reportId` (required): The report ID

**Example LLM prompt**: "Get details for report rpt_xyz789"

---

### sap-concur_create_report

**What it does**: Creates a new expense report.

**When to use**: Start a new expense report.

**Arguments**:
- `name` (required): Report name
- `submittedDate` (optional): Submission date

**Example LLM prompt**: "Create a report called 'Business Trip March'"

---

### sap-concur_submit_report

**What it does**: Submits a report for approval.

**When to use**: Send report for review.

**Arguments**:
- `reportId` (required): The report ID to submit

**Example LLM prompt**: "Submit report rpt_123 for approval"

---

### sap-concur_list_users

**What it does**: Returns a list of users.

**When to use**: View employee list.

**Arguments**:
- `limit` (optional): Number of users (default 50)

**Example LLM prompt**: "List all users"

---

### sap-concur_get_approval_chain

**What it does**: Returns approval chain for a report.

**When to use**: See who will approve a report.

**Arguments**:
- `reportId` (required): The report ID

**Example LLM prompt**: "Get approval chain for report rpt_123"

---

### sap-concur_get_policy

**What it does**: Returns expense policy information.

**When to use**: Understand expense rules.

**Arguments**:
- `policyId` (optional): Policy ID

**Example LLM prompt**: "Get expense policy details"

---

## SAP Concur Notes

- Expenses are individual line items
- Reports group expenses together
- Approval chains route reports for review
- Policies define expense rules
