# Loop Returns Tools

Provider: `loop-returns` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Loop Returns API. They allow AI agents to manage product returns, policies, and customer information. **Requires Loop Returns API key.**

## Authentication

**Nango API_KEY**:
- User provides Loop Returns API key via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Header: `x-authorization: ${apiKey}`
- Base URL: `https://api.loopreturns.com/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `loop_list_returns` | List all returns | GET | /v1/returns |
| `loop_get_return` | Get return details | GET | /v1/returns/{returnId} |
| `loop_create_return` | Create a return | POST | /v1/returns |
| `loop_update_return` | Update a return | PUT | /v1/returns/{returnId} |
| `loop_list_policies` | List return policies | GET | /v1/policies |
| `loop_get_policy` | Get policy details | GET | /v1/policies/{policyId} |
| `loop_list_customers` | List customers | GET | /v1/customers |
| `loop_get_customer` | Get customer details | GET | /v1/customers/{customerId} |
| `loop_create_exchange` | Create an exchange | POST | /v1/exchanges |
| `loop_list_reasons` | List return reasons | GET | /v1/return-reasons |

---

## Tool Details

### loop_list_returns

**What it does**: Lists all returns in Loop Returns with optional filters.

**When to use**: Review returns, find returns by status or date.

**Arguments**:
- `status` (optional): Filter by status (pending, approved, rejected, received)
- `customer_id` (optional): Filter by customer ID
- `order_id` (optional): Filter by order ID
- `from_date` (optional): Filter from date (ISO 8601)
- `to_date` (optional): Filter to date (ISO 8601)
- `page` (optional): Page number (default 1)
- `limit` (optional): Results per page (default 50)

**Example LLM prompt**: "List all pending returns from the last week"

---

### loop_get_return

**What it does**: Gets detailed information about a specific return.

**When to use**: Investigate a specific return, check return status.

**Arguments**:
- `returnId` (required): Return ID

**Example LLM prompt**: "Get details for return RET-12345"

---

### loop_create_return

**What it does**: Creates a new return request.

**When to use**: Process customer return requests.

**Arguments**:
- `order_id` (required): Original order ID
- `customer_id` (required): Customer ID
- `reason` (optional): Return reason
- `items` (optional): Items to return with item_id and quantity

**Example LLM prompt**: "Create a return for order ORD-12345 from customer CUST-001"

---

### loop_update_return

**What it does**: Updates an existing return request status.

**When to use**: Approve/reject returns, add internal notes.

**Arguments**:
- `returnId` (required): Return ID
- `status` (optional): New status
- `internal_note` (optional): Internal note

**Example LLM prompt**: "Approve return RET-12345"

---

### loop_list_policies

**What it does**: Lists all return policies configured in Loop.

**When to use**: Understand return rules, check policy eligibility.

**Arguments**:
- `page` (optional): Page number (default 1)
- `limit` (optional): Results per page (default 50)

**Example LLM prompt**: "List all return policies"

---

### loop_get_policy

**What it does**: Gets details of a specific return policy.

**When to use**: Check specific policy details and rules.

**Arguments**:
- `policyId` (required): Policy ID

**Example LLM prompt**: "Get details for policy POL-30DAY"

---

### loop_list_customers

**What it does**: Lists all customers in Loop Returns.

**When to use**: Find customers, check customer return history.

**Arguments**:
- `email` (optional): Filter by email
- `page` (optional): Page number (default 1)
- `limit` (optional): Results per page (default 50)

**Example LLM prompt**: "Find customer with email john@example.com"

---

### loop_get_customer

**What it does**: Gets details of a specific customer.

**When to use**: Check customer details and return history.

**Arguments**:
- `customerId` (required): Customer ID

**Example LLM prompt**: "Get details for customer CUST-001"

---

### loop_create_exchange

**What it does**: Creates an exchange for a return.

**When to use**: Process exchange requests when customers want different items.

**Arguments**:
- `return_id` (required): Return ID
- `new_item_id` (required): New item ID to exchange for
- `quantity` (optional): Quantity (default 1)

**Example LLM prompt**: "Create an exchange for return RET-12345 with item SHIRT-BLUE-M"

---

### loop_list_reasons

**What it does**: Lists all configured return reasons.

**When to use**: Understand why items are returned, categorize returns.

**Arguments**: None

**Example LLM prompt**: "List all return reasons"

---

## Loop Returns Notes

- **Return status**: Track through lifecycle (pending -> approved -> received -> processed)
- **Exchanges**: Created against existing returns
- **Policies**: Control return window, restocking fees, eligible items
- **Rate limits**: Implement backoff for bulk operations
