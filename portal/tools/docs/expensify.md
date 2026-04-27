# Expensify

Expense management platform for reports, expenses, policies, and employees.

## Authentication

Expensify uses OAuth 2.0 authentication. The Nango integration handles token refresh automatically.

## API Endpoints

Expensify API base URL: `https://api.expensify.com`

## Tools

### Reports

#### `expensify_list_reports`
List all expense reports with optional filtering.

**Endpoint:** `GET /v1/reports`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| status | string | Filter by status: open, submitted, approved, rejected, paid |
| from_date | string | Filter from date (YYYY-MM-DD) |
| to_date | string | Filter to date (YYYY-MM-DD) |
| limit | integer | Maximum reports to return (default: 100) |

#### `expensify_get_report`
Get details of a specific expense report.

**Endpoint:** `GET /v1/reports/{reportID}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| reportID | string | The report ID |

#### `expensify_create_report`
Create a new expense report.

**Endpoint:** `POST /v1/reports`

**Scopes:** `write`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| title | string | Report title |
| policyID | string | The policy ID to use |
| fields | object | Additional report fields |

#### `expensify_submit_report`
Submit a report for approval.

**Endpoint:** `POST /v1/reports/{reportID}/submit`

**Scopes:** `write`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| reportID | string | The report ID to submit |

### Expenses

#### `expensify_list_expenses`
List all expenses with optional filtering.

**Endpoint:** `GET /v1/expenses`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| report_id | string | Filter by report ID |
| status | string | Filter by status: pending, approved, rejected |
| from_date | string | Filter from date (YYYY-MM-DD) |
| to_date | string | Filter to date (YYYY-MM-DD) |
| limit | integer | Maximum expenses to return (default: 100) |

#### `expensify_get_expense`
Get details of a specific expense.

**Endpoint:** `GET /v1/expenses/{expenseID}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| expenseID | string | The expense ID |

#### `expensify_create_expense`
Create a new expense.

**Endpoint:** `POST /v1/expenses`

**Scopes:** `write`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| amount | number | Expense amount |
| currency | string | Currency code (e.g., USD) |
| merchant | string | Merchant name |
| date | string | Expense date (YYYY-MM-DD) |
| report_id | string | Report ID to add expense to |
| category | string | Expense category |

### Policies

#### `expensify_list_policies`
List all expense policies in the organization.

**Endpoint:** `GET /v1/policies`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| status | string | Filter by status: enabled, disabled |
| limit | integer | Maximum policies to return (default: 100) |

#### `expensify_get_policy`
Get details of a specific expense policy.

**Endpoint:** `GET /v1/policies/{policyID}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| policyID | string | The policy ID |

### Employees

#### `expensify_list_employees`
List all employees in the organization.

**Endpoint:** `GET /v1/employees`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| status | string | Filter by employment status: active, inactive |
| policy_id | string | Filter by policy ID |
| limit | integer | Maximum employees to return (default: 100) |

## Rate Limits

Expensify API rate limits depend on your plan. Standard limits are around 100 requests per minute for most operations.

## Common Use Cases

1. **Expense Automation**: Create and submit reports programmatically
2. **Policy Enforcement**: Check expenses against company policies
3. **Financial Reporting**: Extract expense data for accounting and auditing
4. **Employee Management**: Sync employee information with external HR systems
