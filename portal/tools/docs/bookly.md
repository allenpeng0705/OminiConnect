# Bookly Tools

Provider: `bookly` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Bookly API. They allow AI agents to manage appointments, customers, services, and staff. Bookly is a popular appointment booking plugin for WordPress.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Bookly
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bookly_list_appointments` | List appointments | GET | /appointments |
| `bookly_get_appointment` | Get appointment details | GET | /appointments/{appointment_id} |
| `bookly_create_appointment` | Create an appointment | POST | /appointments |
| `bookly_cancel_appointment` | Cancel an appointment | DELETE | /appointments/{appointment_id} |
| `bookly_list_customers` | List customers | GET | /customers |
| `bookly_get_customer` | Get customer details | GET | /customers/{customer_id} |
| `bookly_list_services` | List services | GET | /services |
| `bookly_get_service` | Get service details | GET | /services/{service_id} |
| `bookly_list_staff` | List staff members | GET | /staff |
| `bookly_get_staff` | Get staff details | GET | /staff/{staff_id} |

---

## Tool Details

### bookly_list_appointments

**What it does**: Lists all appointments with optional filters.

**When to use**: Find appointments by date, customer, staff, or status.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)
- `customer_id` (optional): Filter by customer ID
- `staff_id` (optional): Filter by staff ID
- `status` (optional): Filter by status (approved, pending, cancelled)
- `count` (optional): Max appointments to return

**Example LLM prompt**: "List all approved appointments for next week"

---

### bookly_get_appointment

**What it does**: Gets detailed information about a specific appointment.

**When to use**: Get appointment details, notes, or status.

**Arguments**:
- `appointment_id` (required): Appointment ID

**Example LLM prompt**: "Get details for appointment 12345"

---

### bookly_create_appointment

**What it does**: Creates a new appointment for a customer with a staff member.

**When to use**: Book a new appointment, schedule a session.

**Arguments**:
- `customer_id` (required): Customer ID
- `service_id` (required): Service ID
- `staff_id` (required): Staff ID
- `start_date` (required): Appointment date and time (YYYY-MM-DD HH:MM:SS)
- `end_date` (optional): End date and time (YYYY-MM-DD HH:MM:SS)
- `notes` (optional): Appointment notes

**Example LLM prompt**: "Create an appointment for customer 123 with staff 456 for service 789 tomorrow at 2pm"

---

### bookly_cancel_appointment

**What it does**: Cancels an existing appointment.

**When to use**: Cancel a booking, free up the time slot.

**Arguments**:
- `appointment_id` (required): Appointment ID
- `reason` (optional): Cancellation reason

**Example LLM prompt**: "Cancel appointment 12345"

---

### bookly_list_customers

**What it does**: Lists all customers in the system.

**When to use**: Find customers, search by name or email.

**Arguments**:
- `search` (optional): Search by name, email, or phone
- `count` (optional): Max customers to return

**Example LLM prompt**: "List all customers with phone number containing 555"

---

### bookly_get_customer

**What it does**: Gets detailed information about a specific customer.

**When to use**: Get customer profile, contact info, history.

**Arguments**:
- `customer_id` (required): Customer ID

**Example LLM prompt**: "Get details for customer 123"

---

### bookly_list_services

**What it does**: Lists all services available for booking.

**When to use**: Find available services, check categories.

**Arguments**:
- `category_id` (optional): Filter by category ID
- `count` (optional): Max services to return

**Example LLM prompt**: "What massage services are available?"

---

### bookly_get_service

**What it does**: Gets detailed information about a specific service.

**When to use**: Get service details, duration, pricing.

**Arguments**:
- `service_id` (required): Service ID

**Example LLM prompt**: "Get details for service 789"

---

### bookly_list_staff

**What it does**: Lists all staff members who provide services.

**When to use**: Find available staff, check services per staff.

**Arguments**:
- `service_id` (optional): Filter by service ID
- `count` (optional): Max staff to return

**Example LLM prompt**: "Which staff members provide massage services?"

---

### bookly_get_staff

**What it does**: Gets detailed information about a specific staff member.

**When to use**: Get staff profile, services they provide, schedule.

**Arguments**:
- `staff_id` (required): Staff ID

**Example LLM prompt**: "Get details for staff member 456"

---

## Bookly API Notes

- **Pagination**: Use `count` parameter to limit results
- **IDs**: Numeric identifiers for appointments, customers, services, and staff
- **Date format**: YYYY-MM-DD HH:MM:SS for datetime values
- **Staff**: Staff members are associated with specific services
- **Cancellation**: Canceled appointments are removed from active lists