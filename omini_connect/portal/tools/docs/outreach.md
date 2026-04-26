# Outreach

Provider: `outreach` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

Outreach is a sales engagement platform for managing prospects, sequences, templates, and calls. This integration allows AI agents to interact with Outreach data through OminiConnect for automated sales outreach and follow-ups.

## Authentication

**Nango (OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `prospects.read`, `prospects.write`, `sequences.read`, `sequences.write`, `templates.read`, `calls.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `outreach_list_prospects` | List prospects | GET | /api/v2/prospects |
| `outreach_get_prospect` | Get prospect details | GET | /api/v2/prospects/{prospectId} |
| `outreach_create_prospect` | Create a prospect | POST | /api/v2/prospects |
| `outreach_list_sequences` | List email sequences | GET | /api/v2/sequences |
| `outreach_get_sequence` | Get sequence details | GET | /api/v2/sequences/{sequenceId} |
| `outreach_add_prospect_to_sequence` | Add prospect to sequence | POST | /api/v2/sequenceenrollments |
| `outreach_list_templates` | List email templates | GET | /api/v2/templates |
| `outreach_get_template` | Get template details | GET | /api/v2/templates/{templateId} |
| `outreach_list_calls` | List calls | GET | /api/v2/calls |
| `outreach_get_call` | Get call details | GET | /api/v2/calls/{callId} |

## Tool Details

### Prospects

#### outreach_list_prospects

List prospects in Outreach. Filter by name, email, stage, or assigned user.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression (e.g., stage='Qualified')

#### outreach_get_prospect

Get detailed information about an Outreach prospect.

**Input Parameters:**
- `prospect_id` (string, required): The prospect ID

#### outreach_create_prospect

Create a new prospect in Outreach.

**Input Parameters:**
- `first_name` (string, optional): First name
- `last_name` (string, optional): Last name
- `email` (string, required): Email address
- `phone` (string, optional): Phone number
- `company` (string, optional): Company name
- `title` (string, optional): Job title

### Sequences

#### outreach_list_sequences

List email sequences in Outreach.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression (e.g., status='active')

#### outreach_get_sequence

Get detailed information about a sequence.

**Input Parameters:**
- `sequence_id` (string, required): The sequence ID

#### outreach_add_prospect_to_sequence

Add a prospect to an Outreach sequence.

**Input Parameters:**
- `prospect_id` (string, required): The prospect ID
- `sequence_id` (string, required): The sequence ID
- `scheduled_at` (string, optional): Schedule date (ISO 8601 format)

### Templates

#### outreach_list_templates

List email templates in Outreach.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression

#### outreach_get_template

Get detailed information about a template.

**Input Parameters:**
- `template_id` (string, required): The template ID

### Calls

#### outreach_list_calls

List calls in Outreach.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression (e.g., outcome='connected')

#### outreach_get_call

Get detailed information about a call.

**Input Parameters:**
- `call_id` (string, required): The call ID
