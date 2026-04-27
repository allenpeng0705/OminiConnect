# Firstbase Tools

Provider: `firstbase` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Firstbase API. They allow AI agents to manage employees, orders, documents, and offers. Firstbase is an HR platform for managing employee onboarding and documents.

## Authentication

**Nango API_KEY**:
- User provides their Firstbase API key via Nango Connect
- Key is passed in the Authorization header as `ApiKey {apiKey}`
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `firstbase_list_employees` | List employees | GET | /api/employees |
| `firstbase_get_employee` | Get employee details | GET | /api/employees/{id} |
| `firstbase_create_employee` | Create an employee | POST | /api/employees |
| `firstbase_update_employee` | Update employee details | PUT | /api/employees/{id} |
| `firstbase_list_orders` | List orders | GET | /api/orders |
| `firstbase_get_order` | Get order details | GET | /api/orders/{id} |
| `firstbase_list_documents` | List documents | GET | /api/documents |
| `firstbase_get_document` | Get document details | GET | /api/documents/{id} |
| `firstbase_list_offers` | List offers | GET | /api/offers |
| `firstbase_get_offer` | Get offer details | GET | /api/offers/{id} |

---

## Tool Details

### firstbase_list_employees

**What it does**: Lists all employees.

**When to use**: Browse employee directory.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all employees"

---

### firstbase_get_employee

**What it does**: Gets details of a specific employee.

**When to use**: View employee information.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get details for employee abc123"

---

### firstbase_create_employee

**What it does**: Creates a new employee.

**When to use**: Add new employees to the system.

**Arguments**:
- `email` (required): Email address
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Create an employee for john@company.com"

---

### firstbase_update_employee

**What it does**: Updates employee details.

**When to use**: Modify employee information.

**Arguments**:
- `id` (required): Employee ID
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Update employee abc123"

---

### firstbase_list_orders

**What it does**: Lists all orders.

**When to use**: Browse order history.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all orders"

---

### firstbase_get_order

**What it does**: Gets details of a specific order.

**When to use**: View order information.

**Arguments**:
- `id` (required): Order ID

**Example LLM prompt**: "Get order xyz789"

---

### firstbase_list_documents

**What it does**: Lists all documents.

**When to use**: Browse document repository.

**Arguments**:
- `employee_id` (optional): Filter by employee

**Example LLM prompt**: "List documents for employee abc123"

---

### firstbase_get_document

**What it does**: Gets details of a specific document.

**When to use**: View document information.

**Arguments**:
- `id` (required): Document ID

**Example LLM prompt**: "Get document def456"

---

### firstbase_list_offers

**What it does**: Lists all offers.

**When to use**: Browse job offers.

**Arguments**: None

**Example LLM prompt**: "List all offers"

---

### firstbase_get_offer

**What it does**: Gets details of a specific offer.

**When to use**: View offer information.

**Arguments**:
- `id` (required): Offer ID

**Example LLM prompt**: "Get offer ghi789"

---

## Firstbase API Notes

- **Employees**: HR records for team members
- **Orders**: Equipment or service orders
- **Documents**: Employee onboarding documents
- **Offers**: Job offer letters and details
- **HR Workflow**: Employee lifecycle management
