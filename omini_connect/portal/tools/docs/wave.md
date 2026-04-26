# Wave Tools

Provider: `wave` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Wave API. They allow AI agents to manage invoices, customers, receipts, and employees. Wave is a free accounting and invoicing platform popular with small businesses and freelancers.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Wave
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `BUSINESS_INVOICE_READ`, `BUSINESS_INVOICE_CREATE`, `BUSINESS_INVOICE_SEND`, `BUSINESS_READ`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wave_list_invoices` | List all invoices | GET | /v1/businesses/{business_id}/invoices |
| `wave_get_invoice` | Get a specific invoice | GET | /v1/businesses/{business_id}/invoices/{invoice_id} |
| `wave_create_invoice` | Create a new invoice | POST | /v1/businesses/{business_id}/invoices |
| `wave_send_invoice` | Send an invoice to customer | POST | /v1/businesses/{business_id}/invoices/{invoice_id}/send |
| `wave_list_customers` | List all customers | GET | /v1/businesses/{business_id}/customers |
| `wave_get_customer` | Get a specific customer | GET | /v1/businesses/{business_id}/customers/{customer_id} |
| `wave_list_receipts` | List all receipts | GET | /v1/businesses/{business_id}/receipts |
| `wave_get_receipt` | Get a specific receipt | GET | /v1/businesses/{business_id}/receipts/{receipt_id} |
| `wave_list_employees` | List all employees | GET | /v1/businesses/{business_id}/employees |
| `wave_get_employee` | Get a specific employee | GET | /v1/businesses/{business_id}/employees/{employee_id} |

---

## Tool Details

### wave_list_invoices

**What it does**: Lists all invoices in the Wave business with pagination.

**When to use**: Find unpaid invoices, see all invoices for a customer, track overdue payments.

**Arguments**:
- `business_id` (required): Wave Business ID
- `page` (optional): Page number for pagination (default: 1)
- `page_size` (optional): Results per page, max 100 (default: 20)

**Example LLM prompt**: "List all overdue invoices"

---

### wave_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items, totals, status, and customer.

**When to use**: Check invoice details, verify line items, see payment history.

**Arguments**:
- `business_id` (required): Wave Business ID
- `invoice_id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice xyz-456"

---

### wave_create_invoice

**What it does**: Creates a new invoice in Wave with customer and line items.

**When to use**: Bill customers for products or services, create project invoices.

**Arguments**:
- `business_id` (required): Wave Business ID
- `customer_id` (required): Customer ID
- `items` (required): Array of line item objects
- `invoice_number` (optional): Custom invoice number
- `due_date` (optional): Due date (YYYY-MM-DD)
- `notes` (optional): Invoice notes
- `currency` (optional): Currency code (default: USD)

**Example LLM prompt**: "Create an invoice for customer abc-123 for $500"

---

### wave_send_invoice

**What it does**: Sends an existing invoice to the customer via email.

**When to use**: Deliver an invoice to the customer for payment, send a reminder.

**Arguments**:
- `business_id` (required): Wave Business ID
- `invoice_id` (required): Invoice ID
- `recipient_email` (optional): Override recipient email
- `subject` (optional): Email subject
- `message` (optional): Custom message to include in email

**Example LLM prompt**: "Send invoice xyz-456 to the customer"

---

### wave_list_customers

**What it does**: Lists all customers in the Wave business with pagination.

**When to use**: Find customers, view all customers, search by name.

**Arguments**:
- `business_id` (required): Wave Business ID
- `page` (optional): Page number for pagination (default: 1)
- `page_size` (optional): Results per page, max 100 (default: 20)

**Example LLM prompt**: "List all customers named Acme"

---

### wave_get_customer

**What it does**: Gets detailed information about a specific customer including contact details and metadata.

**When to use**: View customer profile before creating an invoice, check customer details.

**Arguments**:
- `business_id` (required): Wave Business ID
- `customer_id` (required): Customer ID

**Example LLM prompt**: "Get details for customer abc-123"

---

### wave_list_receipts

**What it does**: Lists all receipts in the Wave business with pagination.

**When to use**: Track business expenses, find receipts by date, review spending.

**Arguments**:
- `business_id` (required): Wave Business ID
- `page` (optional): Page number for pagination (default: 1)
- `page_size` (optional): Results per page, max 100 (default: 20)

**Example LLM prompt**: "List all receipts from this month"

---

### wave_get_receipt

**What it does**: Gets detailed information about a specific receipt including vendor, amount, and attachments.

**When to use**: Review receipt details, check expense amount, verify vendor information.

**Arguments**:
- `business_id` (required): Wave Business ID
- `receipt_id` (required): Receipt ID

**Example LLM prompt**: "Get details for receipt rcp-789"

---

### wave_list_employees

**What it does**: Lists all employees in the Wave business with pagination.

**When to use**: View employee directory, find employee contact information.

**Arguments**:
- `business_id` (required): Wave Business ID
- `page` (optional): Page number for pagination (default: 1)
- `page_size` (optional): Results per page, max 100 (default: 20)

**Example LLM prompt**: "List all employees"

---

### wave_get_employee

**What it does**: Gets detailed information about a specific employee including contact details and role.

**When to use**: Get employee details, verify role or contact information.

**Arguments**:
- `business_id` (required): Wave Business ID
- `employee_id` (required): Employee ID

**Example LLM prompt**: "Get details for employee emp-101"

---

## Wave API Notes

- **Business ID**: Wave requires a Business ID for all API calls. Found in your Wave account settings.
- **Invoice status flow**: Invoices progress from `DRAFT` -> `SENT` -> `VIEWED` -> `PAID`. Overdue invoices are marked `OVERDUE`.
- **Line items**: Invoices require line items with product_id, description, quantity, and unit_price.
- **Pagination**: Default page size is 20, maximum is 100 per page.
- **Date format**: Use YYYY-MM-DD format for all date parameters.
- **Currency**: Default currency is USD. Other supported currencies include CAD, GBP, etc.
- **Email sending**: The send_invoice tool delivers invoices directly to customers via email.
