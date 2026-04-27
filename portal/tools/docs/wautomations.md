# Wautomations Tools

Provider: `wautomations` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Wautomations API. They allow AI agents to manage marketing campaigns, contacts, workflows, and email templates. Wautomations is a marketing automation platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Wautomations
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `campaigns:read`, `campaigns:write`, `contacts:read`, `contacts:write`, `workflows:read`, `workflows:write`, `emails:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wautomations_list_campaigns` | List marketing campaigns | GET | /api/campaigns |
| `wautomations_get_campaign` | Get campaign details | GET | /api/campaigns/{campaign_id} |
| `wautomations_create_campaign` | Create a new campaign | POST | /api/campaigns |
| `wautomations_list_contacts` | List contacts | GET | /api/contacts |
| `wautomations_get_contact` | Get contact details | GET | /api/contacts/{contact_id} |
| `wautomations_add_contact` | Add a new contact | POST | /api/contacts |
| `wautomations_list_workflows` | List automation workflows | GET | /api/workflows |
| `wautomations_get_workflow` | Get workflow details | GET | /api/workflows/{workflow_id} |
| `wautomations_trigger_workflow` | Trigger a workflow for a contact | POST | /api/workflows/{workflow_id}/trigger |
| `wautomations_list_emails` | List email templates | GET | /api/emails |

---

## Tool Details

### wautomations_list_campaigns

**What it does**: Lists all marketing campaigns with status, stats, and targeting info.

**When to use**: Browse campaigns, check status, review campaign performance.

**Arguments**:
- `status` (optional): Filter by status: draft, scheduled, active, paused, completed
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all active marketing campaigns"

---

### wautomations_get_campaign

**What it does**: Gets detailed information about a campaign including stats and settings.

**When to use**: Check campaign details, view recipient count, review email settings.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign abc123"

---

### wautomations_create_campaign

**What it does**: Creates a new marketing campaign with targeting and content settings.

**When to use**: Set up new campaigns, configure email content, select audience.

**Arguments**:
- `name` (required): Campaign name
- `subject` (required): Email subject line
- `list_id` (optional): Target contact list ID
- `from_name` (optional): Sender name
- `from_email` (optional): Sender email

**Example LLM prompt**: "Create a campaign called 'Spring Sale' with subject 'Limited Time Offer'"

---

### wautomations_list_contacts

**What it does**: Lists all contacts with filtering and pagination support.

**When to use**: Browse contacts, find by list or tag, see contact overview.

**Arguments**:
- `list_id` (optional): Filter by contact list ID
- `tag` (optional): Filter by tag
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all contacts with the VIP tag"

---

### wautomations_get_contact

**What it does**: Gets detailed information about a contact including tags and activity.

**When to use**: Look up individual contact, check engagement history.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get contact details for abc123"

---

### wautomations_add_contact

**What it does**: Adds a new contact to the system with optional list assignment and tags.

**When to use**: Add new subscribers, import contacts, capture leads.

**Arguments**:
- `email` (required): Contact email
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `list_id` (optional): List ID to add contact to
- `tags` (optional): Tags to apply

**Example LLM prompt**: "Add a new contact with email john@example.com to the Marketing list with tag new-subscriber"

---

### wautomations_list_workflows

**What it does**: Lists all automation workflows with status and trigger info.

**When to use**: Browse workflows, check active automations, see workflow catalog.

**Arguments**:
- `status` (optional): Filter by status: inactive, active
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all active workflows"

---

### wautomations_get_workflow

**What it does**: Gets detailed workflow information including steps and trigger conditions.

**When to use**: View workflow configuration, understand automation logic.

**Arguments**:
- `workflow_id` (required): Workflow ID

**Example LLM prompt**: "Get details for workflow abc123"

---

### wautomations_trigger_workflow

**What it does**: Triggers a workflow manually for a specific contact.

**When to use**: Run automation on demand, test workflows, apply to specific contact.

**Arguments**:
- `workflow_id` (required): Workflow ID
- `contact_id` (required): Contact ID to trigger workflow for

**Example LLM prompt**: "Trigger the welcome workflow for contact abc123"

---

### wautomations_list_emails

**What it does**: Lists all email templates with subject and status info.

**When to use**: Browse templates, select email content for campaigns.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all available email templates"

---

## Wautomations API Notes

- **Campaigns**: Marketing emails with subject, content, and audience targeting
- **Contacts**: Individual recipients with email, name, tags, and list membership
- **Workflows**: Automated sequences triggered by events or schedules
- **Emails**: Reusable email templates for campaigns
- **Lists**: Groups of contacts for targeting campaigns
- **Tags**: Labels for segmenting contacts (e.g., VIP, new-subscriber)
- **Status**: Campaigns can be draft, scheduled, active, paused, or completed