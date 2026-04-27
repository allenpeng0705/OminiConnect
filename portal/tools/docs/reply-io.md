# Reply-io Tools

Provider: `reply-io` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Reply.io is a sales engagement platform for email outreach and follow-ups. These tools allow AI agents to manage email accounts, sequences, contacts, tasks, and track outreach statistics.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Reply.io
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `email-accounts:read`, `sequences:read`, `sequences:write`, `contacts:read`, `contacts:write`, `tasks:read`, `tasks:write`, `stats:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `reply-io_list_email_accounts` | List email accounts | GET | /email-accounts |
| `reply-io_get_email_account` | Get email account details | GET | /email-accounts/{accountId} |
| `reply-io_list_sequences` | List email sequences | GET | /sequences |
| `reply-io_get_sequence` | Get sequence details | GET | /sequences/{sequenceId} |
| `reply-io_create_sequence` | Create a sequence | POST | /sequences |
| `reply-io_list_contacts` | List contacts | GET | /contacts |
| `reply-io_add_contact` | Add a contact | POST | /contacts |
| `reply-io_list_tasks` | List tasks | GET | /tasks |
| `reply-io_create_task` | Create a task | POST | /tasks |
| `reply-io_get_stats` | Get sequence stats | GET | /stats |

---

## Tool Details

### reply-io_list_email_accounts

**What it does**: Returns a list of connected email accounts.

**When to use**: Manage email accounts used for outreach.

**Arguments**:
- `limit` (optional): Number of accounts (default 50)

**Example LLM prompt**: "List all my email accounts in Reply.io"

---

### reply-io_get_email_account

**What it does**: Gets details of a specific email account.

**When to use**: Check account status and daily limits.

**Arguments**:
- `accountId` (required): The email account ID

**Example LLM prompt**: "Get details for email account ea_abc123"

---

### reply-io_list_sequences

**What it does**: Returns a list of all email sequences.

**When to use**: View outreach campaigns.

**Arguments**:
- `limit` (optional): Number of sequences (default 50)
- `status` (optional): Filter by status (active, paused, completed)

**Example LLM prompt**: "List all active sequences"

---

### reply-io_get_sequence

**What it does**: Gets details of a specific sequence.

**When to use**: Get sequence steps and settings.

**Arguments**:
- `sequenceId` (required): The sequence ID

**Example LLM prompt**: "Get details for sequence seq_xyz789"

---

### reply-io_create_sequence

**What it does**: Creates a new email sequence.

**When to use**: Set up automated outreach campaigns.

**Arguments**:
- `name` (required): Sequence name
- `subject` (optional): Email subject
- `body` (optional): Email body template

**Example LLM prompt**: "Create a sequence called 'Follow-up Series'"

---

### reply-io_list_contacts

**What it does**: Returns a list of all contacts.

**When to use**: View your outreach database.

**Arguments**:
- `limit` (optional): Number of contacts (default 50)

**Example LLM prompt**: "List all contacts"

---

### reply-io_add_contact

**What it does**: Adds a new contact to your account.

**When to use**: Import new leads.

**Arguments**:
- `email` (required): Contact email
- `firstName` (optional): First name
- `lastName` (optional): Last name
- `company` (optional): Company name

**Example LLM prompt**: "Add a contact for john@example.com"

---

### reply-io_list_tasks

**What it does**: Returns a list of all tasks.

**When to use**: View follow-up schedule.

**Arguments**:
- `limit` (optional): Number of tasks (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending tasks"

---

### reply-io_create_task

**What it does**: Creates a new task.

**When to use**: Set reminders for follow-ups.

**Arguments**:
- `title` (required): Task title
- `contactId` (optional): Related contact
- `dueDate` (optional): Due date (ISO 8601)

**Example LLM prompt**: "Create a task to follow up with john@example.com"

---

### reply-io_get_stats

**What it does**: Returns statistics for sequences.

**When to use**: Track email open rates and replies.

**Arguments**:
- `sequenceId` (optional): Filter by sequence

**Example LLM prompt**: "Get stats for sequence seq_abc123"

---

## Reply.io Notes

- Email sequences automate outreach follow-ups
- Contacts are leads and prospects in your pipeline
- Tasks track individual follow-up actions
- Stats show email performance metrics
