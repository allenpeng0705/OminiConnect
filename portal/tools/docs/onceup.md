# OnceUp Tools

Provider: `onceup` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the OnceUp API. They allow AI agents to manage appointments, clients, services, and reminders. OnceUp is an appointment scheduling and reminder management platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with OnceUp
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `onceup_list_appointments` | List appointments | GET | /appointments |
| `onceup_get_appointment` | Get appointment details | GET | /appointments/{appointment_id} |
| `onceup_create_appointment` | Create an appointment | POST | /appointments |
| `onceup_cancel_appointment` | Cancel an appointment | DELETE | /appointments/{appointment_id} |
| `onceup_list_clients` | List clients | GET | /clients |
| `onceup_get_client` | Get client details | GET | /clients/{client_id} |
| `onceup_list_services` | List services | GET | /services |
| `onceup_get_service` | Get service details | GET | /services/{service_id} |
| `onceup_list_reminders` | List reminders | GET | /reminders |
| `onceup_send_reminder` | Send a reminder | POST | /reminders/send |

---

## Tool Details

### onceup_list_appointments

**What it does**: Lists all appointments with optional filters.

**When to use**: Find appointments by date, client, or status.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)
- `client_id` (optional): Filter by client ID
- `status` (optional): Filter by status (scheduled, completed, cancelled)
- `count` (optional): Max appointments to return

**Example LLM prompt**: "List all scheduled appointments for next week"

---

### onceup_get_appointment

**What it does**: Gets detailed information about a specific appointment.

**When to use**: Get appointment details, notes, or status.

**Arguments**:
- `appointment_id` (required): Appointment ID

**Example LLM prompt**: "Get details for appointment abc123"

---

### onceup_create_appointment

**What it does**: Creates a new appointment for a client with a service.

**When to use**: Book a new appointment, schedule a session.

**Arguments**:
- `client_id` (required): Client ID
- `service_id` (required): Service ID
- `start_time` (required): Start time (ISO 8601)
- `end_time` (optional): End time (ISO 8601)
- `notes` (optional): Appointment notes

**Example LLM prompt**: "Create an appointment for client 123 with service 456 tomorrow at 2pm"

---

### onceup_cancel_appointment

**What it does**: Cancels an existing appointment.

**When to use**: Cancel a booking, free up the time slot.

**Arguments**:
- `appointment_id` (required): Appointment ID
- `reason` (optional): Cancellation reason

**Example LLM prompt**: "Cancel appointment abc123"

---

### onceup_list_clients

**What it does**: Lists all clients in the account.

**When to use**: Find clients, search by name or email.

**Arguments**:
- `search` (optional): Search by name or email
- `count` (optional): Max clients to return

**Example LLM prompt**: "List all clients with email containing @example.com"

---

### onceup_get_client

**What it does**: Gets detailed information about a specific client.

**When to use**: Get client profile, contact info, appointment history.

**Arguments**:
- `client_id` (required): Client ID

**Example LLM prompt**: "Get details for client 123"

---

### onceup_list_services

**What it does**: Lists all services available for booking.

**When to use**: Find available services, check pricing.

**Arguments**:
- `category` (optional): Filter by category
- `count` (optional): Max services to return

**Example LLM prompt**: "What services are available?"

---

### onceup_get_service

**What it does**: Gets detailed information about a specific service.

**When to use**: Get service details, duration, pricing.

**Arguments**:
- `service_id` (required): Service ID

**Example LLM prompt**: "Get details for service 456"

---

### onceup_list_reminders

**What it does**: Lists all configured reminders for appointments.

**When to use**: Find reminders by appointment or status.

**Arguments**:
- `appointment_id` (optional): Filter by appointment ID
- `sent` (optional): Filter by sent status
- `count` (optional): Max reminders to return

**Example LLM prompt**: "List all pending reminders for today"

---

### onceup_send_reminder

**What it does**: Sends a reminder notification for an appointment.

**When to use**: Send appointment reminders via email, SMS, or push.

**Arguments**:
- `appointment_id` (required): Appointment ID
- `channel` (required): Notification channel (email, sms, push)

**Example LLM prompt**: "Send an email reminder for appointment abc123"

---

## OnceUp API Notes

- **Pagination**: Use `count` parameter to limit results
- **IDs**: String identifiers for appointments, clients, and services
- **Time format**: ISO 8601 format for datetime values
- **Reminders**: Send notifications via email, SMS, or push notifications