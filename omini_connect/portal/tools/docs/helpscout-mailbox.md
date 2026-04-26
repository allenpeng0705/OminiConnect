# Help Scout Mailbox Tools

Provider: `helpscout-mailbox` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Help Scout Mailbox API. They allow AI agents to manage conversations, customers, mailboxes, and reports. Help Scout Mailbox is a shared inbox platform for customer support.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Help Scout
- Token stored in Nango, accessed via `connection_ref`
- Authorization URL: https://secure.helpscout.net/authentication/authorizeClientApplication
- Token URL: https://api.helpscout.net/v2/oauth2/token

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `helpscout_list_conversations` | List conversations | GET | /conversations |
| `helpscout_get_conversation` | Get conversation details | GET | /conversations/{id} |
| `helpscout_create_conversation` | Create a conversation | POST | /conversations |
| `helpscout_list_mailboxes` | List mailboxes | GET | /mailboxes |
| `helpscout_get_mailbox` | Get mailbox details | GET | /mailboxes/{id} |
| `helpscout_list_customers` | List customers | GET | /customers |
| `helpscout_get_customer` | Get customer details | GET | /customers/{id} |
| `helpscout_list_tags` | List tags | GET | /tags |
| `helpscout_list_workflows` | List workflows | GET | /workflows |
| `helpscout_list_reports` | List reports | GET | /reports |

---

## Tool Details

### helpscout_list_conversations

**What it does**: Lists all conversations in Help Scout Mailbox.

**When to use**: Browse support conversations.

**Arguments**:
- `mailbox_id` (optional): Filter by mailbox ID
- `status` (optional): Filter by status (`active`, `all`, `archived`, `spam`, `trash`)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all active conversations"

---

### helpscout_get_conversation

**What it does**: Gets detailed information about a specific conversation.

**When to use**: View conversation details and messages.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "Get conversation with ID 123"

---

### helpscout_create_conversation

**What it does**: Creates a new conversation in Help Scout Mailbox.

**When to use**: Create a new support conversation.

**Arguments**:
- `subject` (required): Conversation subject
- `customer_id` (required): Customer ID
- `mailbox_id` (required): Mailbox ID
- `status` (optional): Status (`active`, `pending`, `snoozed`)

**Example LLM prompt**: "Create a conversation for customer 456 in mailbox 1"

---

### helpscout_list_mailboxes

**What it does**: Lists all mailboxes in Help Scout Mailbox.

**When to use**: Browse available mailboxes.

**Arguments**: None

**Example LLM prompt**: "List all mailboxes"

---

### helpscout_get_mailbox

**What it does**: Gets detailed information about a specific mailbox.

**When to use**: View mailbox settings and folders.

**Arguments**:
- `id` (required): Mailbox ID

**Example LLM prompt**: "Get mailbox with ID 1"

---

### helpscout_list_customers

**What it does**: Lists all customers in Help Scout Mailbox.

**When to use**: Browse customer list.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all customers"

---

### helpscout_get_customer

**What it does**: Gets detailed information about a specific customer.

**When to use**: View customer profile and history.

**Arguments**:
- `id` (required): Customer ID

**Example LLM prompt**: "Get customer with ID 789"

---

### helpscout_list_tags

**What it does**: Lists all tags in Help Scout Mailbox.

**When to use**: View available tags for organizing.

**Arguments**: None

**Example LLM prompt**: "List all tags"

---

### helpscout_list_workflows

**What it does**: Lists all workflows in Help Scout Mailbox.

**When to use**: View automation workflows.

**Arguments**: None

**Example LLM prompt**: "List all workflows"

---

### helpscout_list_reports

**What it does**: Lists all reports in Help Scout Mailbox.

**When to use**: View analytics and metrics.

**Arguments**: None

**Example LLM prompt**: "List all reports"

---

## Help Scout Mailbox API Notes

- **API Base URL**: https://api.helpscout.net
- **Auth Mode**: OAuth2
- **Conversations**: Customer support threads
- **Mailboxes**: Shared inboxes for teams
- **Customers**: Contact records with history
- **Tags**: Labels for organizing conversations
- **Workflows**: Automation rules for processing
- **Reports**: Analytics and performance metrics
