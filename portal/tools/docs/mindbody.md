# Mindbody Tools

Provider: `mindbody` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Mindbody API. They allow AI agents to manage locations, staff, services, classes, and clients in the wellness industry. **Requires Mindbody API key.**

## Authentication

**Nango API_KEY**:
- User provides Mindbody API key via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Header: `api-key: ${apiKey}`, `siteid: ${connectionConfig.siteId}`
- Base URL: `https://api.mindbodyonline.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mindbody_list_locations` | List locations | GET | /public/v6/site/locations |
| `mindbody_get_location` | Get location details | GET | /public/v6/site/locations/{locationId} |
| `mindbody_list_staff` | List staff members | GET | /public/v6/staff/staffmembers |
| `mindbody_get_staff` | Get staff details | GET | /public/v6/staff/staffmembers/{staffId} |
| `mindbody_list_services` | List services | GET | /public/v6/services |
| `mindbody_get_service` | Get service details | GET | /public/v6/services/{serviceId} |
| `mindbody_list_classes` | List classes | GET | /public/v6/classes |
| `mindbody_get_class` | Get class details | GET | /public/v6/classes/{classId} |
| `mindbody_list_clients` | List clients | GET | /public/v6/clients |
| `mindbody_get_client` | Get client details | GET | /public/v6/clients/{clientId} |

---

## Tool Details

### mindbody_list_locations

**What it does**: Lists all Mindbody locations.

**When to use**: Find locations, check business details.

**Arguments**: None

**Example LLM prompt**: "List all Mindbody locations"

---

### mindbody_get_location

**What it does**: Gets details of a specific Mindbody location.

**When to use**: Get location information, address, hours.

**Arguments**:
- `locationId` (required): Location ID

**Example LLM prompt**: "Get details for location 1"

---

### mindbody_list_staff

**What it does**: Lists all staff members in Mindbody.

**When to use**: Find instructors, browse staff.

**Arguments**:
- `location_id` (optional): Filter by location ID

**Example LLM prompt**: "List all staff at location 1"

---

### mindbody_get_staff

**What it does**: Gets details of a specific staff member.

**When to use**: Get staff bio, credentials.

**Arguments**:
- `staffId` (required): Staff ID

**Example LLM prompt**: "Get details for staff member 12345"

---

### mindbody_list_services

**What it does**: Lists all services (classes, appointments) in Mindbody.

**When to use**: Browse offerings, find service IDs.

**Arguments**:
- `location_id` (optional): Filter by location ID

**Example LLM prompt**: "List all services"

---

### mindbody_get_service

**What it does**: Gets details of a specific service.

**When to use**: Check service details, pricing.

**Arguments**:
- `serviceId` (required): Service ID

**Example LLM prompt**: "Get details for service 12345"

---

### mindbody_list_classes

**What it does**: Lists all classes in Mindbody.

**When to use**: Find classes, check schedules.

**Arguments**:
- `location_id` (optional): Filter by location ID
- `start_date` (optional): Start date (ISO 8601)
- `end_date` (optional): End date (ISO 8601)
- `staff_id` (optional): Filter by instructor

**Example LLM prompt**: "List all classes for next week at location 1"

---

### mindbody_get_class

**What it does**: Gets details of a specific class.

**When to use**: Check class info, availability.

**Arguments**:
- `classId` (required): Class ID

**Example LLM prompt**: "Get details for class 12345"

---

### mindbody_list_clients

**What it does**: Lists all clients in Mindbody.

**When to use**: Browse clients, find client IDs.

**Arguments**:
- `search_text` (optional): Search by name or email
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "Find client with email john@example.com"

---

### mindbody_get_client

**What it does**: Gets details of a specific client.

**When to use**: Get client info, visit history.

**Arguments**:
- `clientId` (required): Client ID

**Example LLM prompt**: "Get details for client CLIENT-12345"

---

## Mindbody Notes

- **Wellness platform**: Gyms, studios, salons
- **Locations**: Multiple business locations
- **Staff**: Instructors and service providers
- **Classes**: Scheduled group sessions
- **Clients**: Customers with visit history
