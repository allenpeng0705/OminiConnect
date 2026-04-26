# Razorpay Tools

Provider: `razorpay` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Razorpay is a payment processing platform for Indian businesses. These tools allow AI agents to manage payments, refunds, invoices, customers, and view account balances.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Razorpay
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `payments:read`, `payments:write`, `refunds:read`, `invoices:read`, `invoices:write`, `customers:read`, `customers:write`, `balance:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `razorpay_list_payments` | List all payments | GET | /v1/payments |
| `razorpay_get_payment` | Get payment details | GET | /v1/payments/{paymentId} |
| `razorpay_create_payment_link` | Create a payment link | POST | /v1/payment_links |
| `razorpay_list_refunds` | List all refunds | GET | /v1/refunds |
| `razorpay_get_refund` | Get refund details | GET | /v1/refunds/{refundId} |
| `razorpay_list_invoices` | List invoices | GET | /v1/invoices |
| `razorpay_create_invoice` | Create an invoice | POST | /v1/invoices |
| `razorpay_list_customers` | List customers | GET | /v1/customers |
| `razorpay_create_customer` | Create a customer | POST | /v1/customers |
| `razorpay_get_balance` | Get account balance | GET | /v1/balance |

---

## Tool Details

### razorpay_list_payments

**What it does**: Returns a paginated list of all payments.

**When to use**: View payment history, find payments by date.

**Arguments**:
- `from` (optional): Start date (Unix timestamp)
- `to` (optional): End date (Unix timestamp)
- `count` (optional): Number of payments (default 100, max 100)
- `skip` (optional): Pagination offset

**Example LLM prompt**: "List all payments from this month"

---

### razorpay_get_payment

**What it does**: Gets details of a specific payment.

**When to use**: Check payment status, get payment method info.

**Arguments**:
- `paymentId` (required): The payment ID

**Example LLM prompt**: "Get details for payment pay_abc123"

---

### razorpay_create_payment_link

**What it does**: Creates a payment link to share with customers.

**When to use**: Collect payments without a website integration.

**Arguments**:
- `amount` (required): Amount in paise
- `currency` (optional): Currency code (default INR)
- `description` (optional): Description
- `customer` (optional): Customer details object

**Example LLM prompt**: "Create a payment link for 1000 rupees"

---

### razorpay_list_refunds

**What it does**: Returns a list of all refunds.

**When to use**: View refund history.

**Arguments**:
- `paymentId` (optional): Filter by payment
- `count` (optional): Number of refunds (default 100)

**Example LLM prompt**: "List all refunds from last week"

---

### razorpay_get_refund

**What it does**: Gets details of a specific refund.

**When to use**: Check refund status.

**Arguments**:
- `refundId` (required): The refund ID

**Example LLM prompt**: "Get status for refund rfd_xyz789"

---

### razorpay_list_invoices

**What it does**: Returns a list of all invoices.

**When to use**: View invoice history, track outstanding invoices.

**Arguments**:
- `customerId` (optional): Filter by customer
- `status` (optional): Filter by status (issued, paid, expired)
- `count` (optional): Number of invoices (default 100)

**Example LLM prompt**: "List all pending invoices"

---

### razorpay_create_invoice

**What it does**: Creates a new invoice with line items.

**When to use**: Send invoices to customers for payment.

**Arguments**:
- `amount` (required): Total amount in paise
- `currency` (optional): Currency code (default INR)
- `customerId` (optional): Customer ID
- `line_items` (optional): Array of line items

**Example LLM prompt**: "Create an invoice for 5000 paise"

---

### razorpay_list_customers

**What it does**: Returns a list of all customers.

**When to use**: View customer database.

**Arguments**:
- `limit` (optional): Number of customers (default 100)

**Example LLM prompt**: "List all customers"

---

### razorpay_create_customer

**What it does**: Creates a new customer.

**When to use**: Add new customers to your Razorpay account.

**Arguments**:
- `name` (required): Customer name
- `email` (optional): Customer email
- `phone` (optional): Customer phone

**Example LLM prompt**: "Create a customer named 'John Doe' with email john@example.com"

---

### razorpay_get_balance

**What it does**: Gets your Razorpay account balance.

**When to use**: Check available balance for payouts.

**Arguments**: None

**Example LLM prompt**: "What's my Razorpay balance?"

---

## Razorpay API Notes

- All amounts are in paise (smallest currency unit) - e.g., 1000 paise = 10 INR
- Default currency is INR (Indian Rupees)
- Payment links are shareable URLs for collecting payments
- Customer IDs link payments and invoices to specific customers
