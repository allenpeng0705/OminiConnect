# Help Scout Tools

Provider: `helpscout` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Help Scout API. They allow AI agents to manage conversations, mailboxes, customers, reports, and workflows. Help Scout is a help desk platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Help Scout
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `helpscout_list_conversations` | List conversations | GET | /conversations |
| `helpscout_get_conversation` | Get conversation details | GET | /conversations/{id} |
| `helpscout_create_conversation` | Create a new conversation | POST | /conversations |
| `helpscout_list_mailboxes` | List mailboxes | GET | /mailboxes |
| `helpscout_get_mailbox` | Get mailbox details | GET | /mailboxes/{id} |
| `helpscout_list_customers` | List customers | GET | /customers |
| `helpscout_get_customer` | Get customer details | GET | /customers/{id} |
| `helpscout_list_reports` | List reports | GET | /reports |
| `helpscout_get_report` | Get report details | GET | /reports/{id} |
| `helpscout_list_workflows` | List workflows | GET | /workflows |

---

## Tool Details

### helpscout_list_conversations

**What it does**: Retrieves a list of conversations from Help Scout.

**When to use**: Browse open conversations, filter by mailbox or status.

**Arguments**:
- `page` (optional): Page number for pagination
- `per_page` (optional): Number of conversations per page (default 25, max 100)
- `status` (optional): Filter by status: `active`, `archived`, `draft`, `snoozed`
- `mailbox_id` (optional): Filter by mailbox ID

**Example LLM prompt**: "List all active conversations in Help Scout"

---

### helpscout_get_conversation

**What it does**: Retrieves a single conversation by its ID.

**When to use**: Get full conversation details including messages and customer info.

**Arguments**:
- `id` (required): The conversation ID

**Example LLM prompt**: "Get conversation 12345 details"

---

### helpscout_create_conversation

**What it does**: Creates a new conversation in Help Scout.

**When to use**: Log a new customer issue or create a conversation.

**Arguments**:
- `mailbox_id` (required): The mailbox ID
- `subject` (required): The conversation subject
- `customer` (required): Customer information (id or email)
- `message` (required): Initial message object with `body` and `type` properties

**Example LLM prompt**: "Create a conversation in mailbox 1 with subject 'Help needed' for customer john@example.com"

---

### helpscout_list_mailboxes

**What it does**: Retrieves a list of all mailboxes in Help Scout.

**When to use**: See available mailboxes before creating or filtering conversations.

**Arguments**:
- `page` (optional): Page number for pagination
- `per_page` (optional): Number of mailboxes per page

**Example LLM prompt**: "List all mailboxes in Help Scout"

---

### helpscout_get_mailbox

**What it does**: Retrieves a single mailbox by its ID.

**When to use**: Get mailbox details including email address and settings.

**Arguments**:
- `id` (required): The mailbox ID

**Example LLM prompt**: "Get mailbox 1 details"

---

### helpscout_list_customers

**What it does**: Retrieves a list of customers from Help Scout.

**When to use**: Search for customers, browse customer base.

**Arguments**:
- `page` (optional): Page number for pagination
- `per_page` (optional): Number of customers per page
- `email` (optional): Search by email address

**Example LLM prompt**: "Find customer john@example.com"

---

### helpscout_get_customer

**What it does**: Retrieves a single customer by their ID.

**When to use**: Get customer details including contact info and conversation history.

**Arguments**:
- `id` (required): The customer ID

**Example LLM prompt**: "Get customer 67890 details"

---

### helpscout_list_reports

**What it does**: Retrieves a list of available reports in Help Scout.

**When to use**: See available report types for analytics.

**Arguments**:
- `page` (optional): Page number for pagination
- `per_page` (optional): Number of reports per page

**Example LLM prompt**: "List all available reports"

---

### helpscout_get_report

**What it does**: Retrieves a specific report with date range data.

**When to use**: Generate analytics on conversations, team performance.

**Arguments**:
- `id` (required): The report ID (e.g., `conversations`, `happiness`, `team`)
- `start_date` (required): Report start date (YYYY-MM-DD)
- `end_date` (required): Report end date (YYYY-MM-DD)

**Example LLM prompt**: "Get the conversations report for the last 30 days"

---

### helpscout_list_workflows

**What it does**: Retrieves a list of all workflows in Help Scout.

**When to use**: See automated workflows for conversation handling.

**Arguments**:
- `page` (optional): Page number for pagination
- `per_page` (optional): Number of workflows per page
- `status` (optional): Filter by status: `active`, `inactive`

**Example LLM prompt**: "List all active workflows"

---

## Help Scout API Notes

- **Conversation Statuses**: `active`, `archived`, `draft`, `snoozed`
- **Mailboxes**: Organize conversations by team or topic
- **Customers**: Can be searched by email address
- **Reports**: Require date range parameters for data retrieval
- **Workflows**: Automate actions based on conversation events
- **Pagination**: Default per_page is 25, maximum is 100
- **Date Format**: ISO 8601 format (YYYY-MM-DD) for report date ranges
