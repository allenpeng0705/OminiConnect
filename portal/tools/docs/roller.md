# Roller Tools

Provider: `roller` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Roller is an appointment scheduling and booking management platform. These tools allow AI agents to manage bookings, customers, services, staff, and availability.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Roller
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `bookings:read`, `bookings:write`, `customers:read`, `customers:write`, `services:read`, `staff:read`, `availability:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `roller_list_bookings` | List all bookings | GET | /v1/bookings |
| `roller_get_booking` | Get booking details | GET | /v1/bookings/{bookingId} |
| `roller_create_booking` | Create a booking | POST | /v1/bookings |
| `roller_list_customers` | List all customers | GET | /v1/customers |
| `roller_get_customer` | Get customer details | GET | /v1/customers/{customerId} |
| `roller_create_customer` | Create a customer | POST | /v1/customers |
| `roller_list_services` | List services offered | GET | /v1/services |
| `roller_list_staff` | List staff members | GET | /v1/staff |
| `roller_get_availability` | Get availability slots | GET | /v1/availability |
| `roller_cancel_booking` | Cancel a booking | POST | /v1/bookings/{bookingId}/cancel |

---

## Tool Details

### roller_list_bookings

**What it does**: Returns a list of all bookings.

**When to use**: View appointments, track schedule.

**Arguments**:
- `limit` (optional): Number of bookings (default 50)
- `status` (optional): Filter by status (confirmed, cancelled, completed)
- `date` (optional): Filter by date

**Example LLM prompt**: "List all bookings for today"

---

### roller_get_booking

**What it does**: Gets details of a specific booking.

**When to use**: Get booking information, customer, service.

**Arguments**:
- `bookingId` (required): The booking ID

**Example LLM prompt**: "Get details for booking bk_abc123"

---

### roller_create_booking

**What it does**: Creates a new booking.

**When to use**: Schedule an appointment.

**Arguments**:
- `customerId` (required): Customer ID
- `serviceId` (required): Service ID
- `staffId` (optional): Staff ID
- `startTime` (required): Booking start time (ISO 8601)

**Example LLM prompt**: "Book customer cst_123 for service svc_456 with staff stf_789 tomorrow at 2pm"

---

### roller_list_customers

**What it does**: Returns a list of all customers.

**When to use**: View customer database.

**Arguments**:
- `limit` (optional): Number of customers (default 50)

**Example LLM prompt**: "List all customers"

---

### roller_get_customer

**What it does**: Gets details of a specific customer.

**When to use**: Get customer info and booking history.

**Arguments**:
- `customerId` (required): The customer ID

**Example LLM prompt**: "Get details for customer cst_xyz789"

---

### roller_create_customer

**What it does**: Creates a new customer.

**When to use**: Add new customers.

**Arguments**:
- `firstName` (required): First name
- `lastName` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number

**Example LLM prompt**: "Create a customer named 'Jane Doe' with email jane@example.com"

---

### roller_list_services

**What it does**: Returns a list of all services.

**When to use**: View available appointments.

**Arguments**:
- `limit` (optional): Number of services (default 50)

**Example LLM prompt**: "List all services"

---

### roller_list_staff

**What it does**: Returns a list of all staff members.

**When to use**: View team members.

**Arguments**:
- `limit` (optional): Number of staff (default 50)

**Example LLM prompt**: "List all staff"

---

### roller_get_availability

**What it does**: Returns available time slots.

**When to use**: Find open appointment times.

**Arguments**:
- `serviceId` (required): Service ID
- `staffId` (optional): Staff ID
- `date` (required): Date to check

**Example LLM prompt**: "Show availability for service svc_123 on 2024-01-15"

---

### roller_cancel_booking

**What it does**: Cancels an existing booking.

**When to use**: Cancel an appointment.

**Arguments**:
- `bookingId` (required): The booking ID
- `reason` (optional): Cancellation reason

**Example LLM prompt**: "Cancel booking bk_abc123"

---

## Roller Notes

- Bookings link customers to services with staff
- Availability shows open time slots
- Customers have booking history
- Services define what can be booked
- Staff provide the services
