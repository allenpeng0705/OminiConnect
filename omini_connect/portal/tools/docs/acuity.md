# Acuity Scheduling Tools

Provider: `acuity` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Acuity Scheduling API. They allow AI agents to manage appointments, clients, services, and staff. Acuity is a popular online appointment scheduling platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Acuity
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `acuity_list_appointments` | List appointments | GET | /appointments |
| `acuity_get_appointment` | Get appointment details | GET | /appointments/{appointment_id} |
| `acuity_create_appointment` | Create an appointment | POST | /appointments |
| `acuity_cancel_appointment` | Cancel an appointment | DELETE | /appointments/{appointment_id} |
| `acuity_list_clients` | List clients | GET | /clients |
| `acuity_get_client` | Get client details | GET | /clients/{client_id} |
| `acuity_list_services` | List services | GET | /services |
| `acuity_get_service` | Get service details | GET | /services/{service_id} |
| `acuity_list_staff` | List staff members | GET | /staff |
| `acuity_get_staff` | Get staff details | GET | /staff/{staff_id} |

---

## Tool Details

### acuity_list_appointments

**What it does**: Lists all appointments with optional filters.

**When to use**: Find appointments by date, client, or service.

**Arguments**:
- `min_date` (optional): Filter from date (YYYY-MM-DD)
- `max_date` (optional): Filter until date (YYYY-MM-DD)
- `client_id` (optional): Filter by client ID
- `service_id` (optional): Filter by service ID
- `count` (optional): Max appointments to return

**Example LLM prompt**: "List all appointments for next week"

---

### acuity_get_appointment

**What it does**: Gets detailed information about a specific appointment.

**When to use**: Get appointment details, notes, or status.

**Arguments**:
- `appointment_id` (required): Appointment ID

**Example LLM prompt**: "Get details for appointment 12345"

---

### acuity_create_appointment

**What it does**: Creates a new appointment for a client.

**When to use**: Book a new appointment, schedule a session.

**Arguments**:
- `client_id` (required): Client ID
- `service_id` (required): Service ID
- `datetime` (required): Appointment date and time (YYYY-MM-DDTHH:MM:SS)
- `duration` (optional): Duration in minutes
- `notes` (optional): Appointment notes

**Example LLM prompt**: "Create an appointment for client 123 with service 456 tomorrow at 2pm"

---

### acuity_cancel_appointment

**What it does**: Cancels an existing appointment.

**When to use**: Cancel a booking, free up the time slot.

**Arguments**:
- `appointment_id` (required): Appointment ID
- `reason` (optional): Cancellation reason

**Example LLM prompt**: "Cancel appointment 12345"

---

### acuity_list_clients

**What it does**: Lists all clients in the account.

**When to use**: Find clients, search by name or email.

**Arguments**:
- `search` (optional): Search by name or email
- `count` (optional): Max clients to return

**Example LLM prompt**: "List all clients with email containing @example.com"

---

### acuity_get_client

**What it does**: Gets detailed information about a specific client.

**When to use**: Get client profile, contact info, appointment history.

**Arguments**:
- `client_id` (required): Client ID

**Example LLM prompt**: "Get details for client 123"

---

### acuity_list_services

**What it does**: Lists all services available for booking.

**When to use**: Find available services, check pricing.

**Arguments**:
- `category` (optional): Filter by category name
- `count` (optional): Max services to return

**Example LLM prompt**: "What services are available in the wellness category?"

---

### acuity_get_service

**What it does**: Gets detailed information about a specific service.

**When to use**: Get service details, duration, pricing.

**Arguments**:
- `service_id` (required): Service ID

**Example LLM prompt**: "Get details for service 456"

---

### acuity_list_staff

**What it does**: Lists all staff members who provide services.

**When to use**: Find staff members, check availability.

**Arguments**:
- `active` (optional): Filter by active status (default true)
- `count` (optional): Max staff to return

**Example LLM prompt**: "List all active staff members"

---

### acuity_get_staff

**What it does**: Gets detailed information about a specific staff member.

**When to use**: Get staff profile, services they provide.

**Arguments**:
- `staff_id` (required): Staff ID

**Example LLM prompt**: "Get details for staff member 789"

---

## Acuity API Notes

- **Pagination**: Use `count` parameter to limit results
- **IDs**: Numeric identifiers for appointments, clients, services, and staff
- **Date format**: ISO 8601 format for datetime values
- **Cancellation**: Canceled appointments are removed from active lists