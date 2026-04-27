# Envoy Tools

Provider: `envoy` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Envoy API. They allow AI agents to interact with visitors, employees, devices, places, and events in the Envoy workplace platform.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `visitors:read`, `visitors:write`, `employees:read`, `devices:read`, `places:read`, `events:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `envoy_list_visitors` | List all visitors | GET | /visitors |
| `envoy_get_visitor` | Get details of a specific visitor | GET | /visitors/{id} |
| `envoy_create_visitor` | Create a new visitor | POST | /visitors |
| `envoy_list_employees` | List all employees | GET | /employees |
| `envoy_get_employee` | Get details of a specific employee | GET | /employees/{id} |
| `envoy_list_devices` | List all devices | GET | /devices |
| `envoy_get_device` | Get details of a specific device | GET | /devices/{id} |
| `envoy_list_places` | List all places | GET | /places |
| `envoy_get_place` | Get details of a specific place | GET | /places/{id} |
| `envoy_list_events` | List all events | GET | /events |

---

## Tool Details

### envoy_list_visitors

**What it does**: Returns a paginated list of visitors with their check-in status, host, and visit details.

**When to use**: Review visitor records, check who's expected, or monitor workplace access.

**Arguments**:
- `date_from` (optional): Filter by start date (YYYY-MM-DD)
- `date_to` (optional): Filter by end date (YYYY-MM-DD)
- `status` (optional): Filter by status: expected, checked_in, checked_out
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "Show me all visitors expected this week"

---

### envoy_get_visitor

**What it does**: Get details of a specific visitor including contact info, host, and visit history.

**When to use**: Look up a specific visitor's details or verify their check-in status.

**Arguments**:
- `id` (required): Visitor ID

**Example LLM prompt**: "Get details for visitor with ID vis123"

---

### envoy_create_visitor

**What it does**: Creates a new visitor record and optionally sends an invitation email to the visitor.

**When to use**: Pre-register visitors before their arrival or automate visitor check-in.

**Arguments**:
- `first_name` (required): Visitor first name
- `last_name` (required): Visitor last name
- `email` (required): Visitor email address
- `host_id` (required): Employee ID of the host
- `scheduled_at` (optional): Expected arrival time (ISO 8601)
- `send_email` (optional): Send invitation email (default true)

**Example LLM prompt**: "Register a visitor named John Doe arriving tomorrow at 2pm hosted by employee emp456"

---

### envoy_list_employees

**What it does**: Returns a paginated list of employees with their departments and roles.

**When to use**: Browse the workplace directory or find specific employees.

**Arguments**:
- `department` (optional): Filter by department
- `role` (optional): Filter by role
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all employees in the engineering department"

---

### envoy_get_employee

**What it does**: Get details of a specific employee including contact info, department, and role.

**When to use**: Look up employee details or verify organizational information.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get details for employee with ID emp789"

---

### envoy_list_devices

**What it does**: Returns a paginated list of devices including badges, tablets, and kiosks with their status.

**When to use**: Manage workplace devices, check device status, or monitor badge inventory.

**Arguments**:
- `type` (optional): Filter by device type: badge, tablet, kiosk
- `status` (optional): Filter by status: online, offline
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all tablets currently online"

---

### envoy_get_device

**What it does**: Get details of a specific device including type, status, location, and configuration.

**When to use**: Check device details or troubleshoot device issues.

**Arguments**:
- `id` (required): Device ID

**Example LLM prompt**: "Get details for device dev123"

---

### envoy_list_places

**What it does**: Returns a paginated list of places including offices, floors, desks, and rooms.

**When to use**: Browse workplace locations or find available spaces.

**Arguments**:
- `type` (optional): Filter by place type: office, floor, desk, room
- `building` (optional): Filter by building name
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all meeting rooms in the main building"

---

### envoy_get_place

**What it does**: Get details of a specific place including address, capacity, amenities, and availability.

**When to use**: Get place information before booking or space planning.

**Arguments**:
- `id` (required): Place ID

**Example LLM prompt**: "Get details for place with ID plc456"

---

### envoy_list_events

**What it does**: Returns a paginated list of events from the workplace calendar with attendee information.

**When to use**: View upcoming events, check room bookings, or monitor calendar activity.

**Arguments**:
- `date_from` (optional): Filter by start date (YYYY-MM-DD)
- `date_to` (optional): Filter by end date (YYYY-MM-DD)
- `place_id` (optional): Filter by place ID
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all events scheduled for next week"
