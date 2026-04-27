# Aircall Tools

Provider: `aircall` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Aircall API. They allow AI agents to manage users, calls, contacts, and teams. Aircall is a cloud-based phone system designed for sales and support teams.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Aircall
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `aircall_list_users` | List users | GET | /users |
| `aircall_get_user` | Get user details | GET | /users/{user_id} |
| `aircall_list_calls` | List calls | GET | /calls |
| `aircall_get_call` | Get call details | GET | /calls/{call_id} |
| `aircall_list_contacts` | List contacts | GET | /contacts |
| `aircall_get_contact` | Get contact details | GET | /contacts/{contact_id} |
| `aircall_create_contact` | Create a contact | POST | /contacts |
| `aircall_list_teams` | List teams | GET | /teams |
| `aircall_get_team` | Get team details | GET | /teams/{team_id} |
| `aircall_create_call` | Create a call | POST | /calls |

---

## Tool Details

### aircall_list_users

**What it does**: Lists all users in Aircall with pagination.

**When to use**: Browse team members, find agents, see availability status.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Users per page (default 25)

**Example LLM prompt**: "List all users in Aircall"

---

### aircall_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user profile, check availability, see direct numbers.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user abc123"

---

### aircall_list_calls

**What it does**: Lists all calls in Aircall with optional filtering.

**When to use**: See call history, analyze call patterns, track metrics.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Calls per page (default 25)
- `direction` (optional): Filter by direction (inbound, outbound)
- `status` (optional): Filter by status (queued, ringing, answered, on_hold, completed, voicemail, no_answer)

**Example LLM prompt**: "List all inbound calls from today"

---

### aircall_get_call

**What it does**: Gets detailed information about a specific call.

**When to use**: Get call details, access recordings and transcriptions.

**Arguments**:
- `call_id` (required): Call ID

**Example LLM prompt**: "Get details for call abc123"

---

### aircall_list_contacts

**What it does**: Lists all contacts in Aircall with pagination.

**When to use**: Browse contacts, find customers, see contact list.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Contacts per page (default 25)

**Example LLM prompt**: "List all contacts in Aircall"

---

### aircall_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Get contact profile, see company and call history.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact abc123"

---

### aircall_create_contact

**What it does**: Creates a new contact in Aircall.

**When to use**: Add new customers, store lead information.

**Arguments**:
- `first_name` (required): Contact first name
- `last_name` (required): Contact last name
- `phone_number` (optional): Phone number
- `email` (optional): Email address
- `company_id` (optional): Company ID

**Example LLM prompt**: "Create a new contact with first name 'John', last name 'Smith', and email john@example.com"

---

### aircall_list_teams

**What it does**: Lists all teams in Aircall with pagination.

**When to use**: Browse teams, understand call routing structure.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Teams per page (default 25)

**Example LLM prompt**: "List all teams in Aircall"

---

### aircall_get_team

**What it does**: Gets detailed information about a specific team.

**When to use**: Get team details, see members and routing rules.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "Get details for team abc123"

---

### aircall_create_call

**What it does**: Initiates a new outbound call.

**When to use**: Make outbound calls, connect agents to customers.

**Arguments**:
- `from` (required): Source (user ID or phone number)
- `to` (required): Destination phone number
- `user_id` (optional): User ID to use for the call

**Example LLM prompt**: "Make a call from user abc123 to phone number 555-1234"

---

## Aircall API Notes

- **Users**: Agents and admins with availability status and direct numbers
- **Calls**: Can be inbound, outbound, with various statuses (queued, ringing, answered, etc.)
- **Contacts**: Customer records with company association and call history
- **Teams**: Groups of agents for call routing and queue management
- **Call Recording**: Calls can be recorded with optional transcription
- **Voicemail**: Calls that go to voicemail generate a voicemail object
