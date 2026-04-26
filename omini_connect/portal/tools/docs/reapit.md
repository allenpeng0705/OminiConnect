# Reapit Tools

Provider: `reapit` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Reapit is a property management and real estate CRM platform. These tools allow AI agents to manage properties, applicants, offers, and tasks in the property sales pipeline.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Reapit
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `properties:read`, `applicants:read`, `applicants:write`, `offers:read`, `offers:write`, `negotiations:read`, `tasks:read`, `tasks:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `reapit_list_properties` | List all properties | GET | /properties |
| `reapit_get_property` | Get property details | GET | /properties/{propertyId} |
| `reapit_list_applicants` | List all applicants | GET | /applicants |
| `reapit_get_applicant` | Get applicant details | GET | /applicants/{applicantId} |
| `reapit_create_applicant` | Create an applicant | POST | /applicants |
| `reapit_list_offers` | List all offers | GET | /offers |
| `reapit_create_offer` | Create an offer | POST | /offers |
| `reapit_list_negotiations` | List negotiations | GET | /negotiations |
| `reapit_list_tasks` | List tasks | GET | /tasks |
| `reapit_create_task` | Create a task | POST | /tasks |

---

## Tool Details

### reapit_list_properties

**What it does**: Returns a list of all properties.

**When to use**: Browse property listings, find available properties.

**Arguments**:
- `limit` (optional): Number of properties (default 50)
- `status` (optional): Filter by status (available, under-offer, sold)

**Example LLM prompt**: "List all available properties"

---

### reapit_get_property

**What it does**: Gets details of a specific property.

**When to use**: Get property information, price, and status.

**Arguments**:
- `propertyId` (required): The property ID

**Example LLM prompt**: "Get details for property prop_abc123"

---

### reapit_list_applicants

**What it does**: Returns a list of all applicants.

**When to use**: View potential buyers in your pipeline.

**Arguments**:
- `limit` (optional): Number of applicants (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active applicants"

---

### reapit_get_applicant

**What it does**: Gets details of a specific applicant.

**When to use**: Get applicant contact info and preferences.

**Arguments**:
- `applicantId` (required): The applicant ID

**Example LLM prompt**: "Get details for applicant app_xyz789"

---

### reapit_create_applicant

**What it does**: Creates a new applicant record.

**When to use**: Add new leads to your pipeline.

**Arguments**:
- `name` (required): Applicant name
- `email` (required): Applicant email
- `phone` (optional): Phone number
- `propertyInterest` (optional): Property ID they're interested in

**Example LLM prompt**: "Create an applicant for john@example.com interested in prop_123"

---

### reapit_list_offers

**What it does**: Returns a list of all offers.

**When to use**: Track offers made on properties.

**Arguments**:
- `limit` (optional): Number of offers (default 50)

**Example LLM prompt**: "List all recent offers"

---

### reapit_create_offer

**What it does**: Creates a new offer on a property.

**When to use**: Record an offer from an applicant.

**Arguments**:
- `propertyId` (required): Property ID
- `applicantId` (required): Applicant ID
- `amount` (required): Offer amount

**Example LLM prompt**: "Create an offer of $500,000 on prop_123 from app_456"

---

### reapit_list_negotiations

**What it does**: Returns a list of negotiations.

**When to use**: Track ongoing property negotiations.

**Arguments**:
- `propertyId` (optional): Filter by property

**Example LLM prompt**: "List negotiations for property prop_123"

---

### reapit_list_tasks

**What it does**: Returns a list of all tasks.

**When to use**: View pending follow-ups and to-dos.

**Arguments**:
- `limit` (optional): Number of tasks (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending tasks"

---

### reapit_create_task

**What it does**: Creates a new task.

**When to use**: Create follow-up reminders.

**Arguments**:
- `title` (required): Task title
- `description` (optional): Task description
- `dueDate` (optional): Due date (ISO 8601)

**Example LLM prompt**: "Create a task to follow up with john@example.com"

---

## Reapit Notes

- Property statuses: available, under-offer, sold
- Applicants are potential buyers in your pipeline
- Offers link applicants to properties with proposed amounts
- Tasks help track follow-ups and action items
