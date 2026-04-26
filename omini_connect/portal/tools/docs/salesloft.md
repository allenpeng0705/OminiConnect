# Salesloft

Provider: `salesloft` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

Salesloft is a sales engagement platform for managing people, cadences, calls, and emails. This integration allows AI agents to interact with Salesloft data through OminiConnect for automated sales outreach and follow-ups.

## Authentication

**Nango (OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `people.read`, `people.write`, `cadences.read`, `cadences.write`, `calls.read`, `emails.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `salesloft_list_people` | List people | GET | /api/v2/people |
| `salesloft_get_person` | Get person details | GET | /api/v2/people/{personId} |
| `salesloft_create_person` | Create a person | POST | /api/v2/people |
| `salesloft_list_cadences` | List cadences | GET | /api/v2/cadences |
| `salesloft_get_cadence` | Get cadence details | GET | /api/v2/cadences/{cadenceId} |
| `salesloft_add_person_to_cadence` | Add person to cadence | POST | /api/v2/cadence_enrollments |
| `salesloft_list_calls` | List calls | GET | /api/v2/calls |
| `salesloft_get_call` | Get call details | GET | /api/v2/calls/{callId} |
| `salesloft_list_emails` | List emails | GET | /api/v2/emails |
| `salesloft_get_email` | Get email details | GET | /api/v2/emails/{emailId} |

## Tool Details

### People

#### salesloft_list_people

List people in Salesloft. Filter by name, email, cadence, or stage.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression (e.g., cadence_id='123')

#### salesloft_get_person

Get detailed information about a Salesloft person.

**Input Parameters:**
- `person_id` (string, required): The person ID

#### salesloft_create_person

Create a new person in Salesloft.

**Input Parameters:**
- `first_name` (string, optional): First name
- `last_name` (string, optional): Last name
- `email_address` (string, required): Email address
- `phone_number` (string, optional): Phone number
- `company_name` (string, optional): Company name
- `title` (string, optional): Job title

### Cadences

#### salesloft_list_cadences

List cadences in Salesloft.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression (e.g., status='active')

#### salesloft_get_cadence

Get detailed information about a cadence.

**Input Parameters:**
- `cadence_id` (string, required): The cadence ID

#### salesloft_add_person_to_cadence

Add a person to a Salesloft cadence.

**Input Parameters:**
- `person_id` (string, required): The person ID
- `cadence_id` (string, required): The cadence ID
- `scheduled_for` (string, optional): Schedule date (ISO 8601 format)

### Calls

#### salesloft_list_calls

List calls in Salesloft.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression (e.g., outcome='connected')

#### salesloft_get_call

Get detailed information about a call.

**Input Parameters:**
- `call_id` (string, required): The call ID

### Emails

#### salesloft_list_emails

List emails in Salesloft.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression (e.g., status='sent')

#### salesloft_get_email

Get detailed information about an email.

**Input Parameters:**
- `email_id` (string, required): The email ID
