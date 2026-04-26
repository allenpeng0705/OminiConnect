# Freeagent Sandbox Tools

Provider: `freeagent-sandbox` | Engine: `nango` | Auth: OAuth via Nango (Sandbox)

## Overview

These tools wrap the Freeagent Sandbox API. They allow AI agents to test accounting workflows in a sandbox environment. Freeagent Sandbox is a testing environment for the Freeagent platform.

## Authentication

**Nango OAuth (Sandbox)**:
- User authenticates via Nango Connect with Freeagent Sandbox
- Token stored in Nango, accessed via `connection_ref`
- OAuth2 with Basic auth for token request
- Sandbox URL: https://api.sandbox.freeagent.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `freeagent_sandbox_list_users` | List users | GET | /v2/users |
| `freeagent_sandbox_get_user` | Get user details | GET | /v2/me |
| `freeagent_sandbox_list_contacts` | List contacts | GET | /v2/contacts |
| `freeagent_sandbox_get_contact` | Get contact details | GET | /v2/contacts/{id} |
| `freeagent_sandbox_list_invoices` | List invoices | GET | /v2/invoices |
| `freeagent_sandbox_get_invoice` | Get invoice details | GET | /v2/invoices/{id} |
| `freeagent_sandbox_list_bank_accounts` | List bank accounts | GET | /v2/bank_accounts |
| `freeagent_sandbox_get_bank_account` | Get bank account details | GET | /v2/bank_accounts/{id} |
| `freeagent_sandbox_list_projects` | List projects | GET | /v2/projects |
| `freeagent_sandbox_get_project` | Get project details | GET | /v2/projects/{id} |

---

## Tool Details

### freeagent_sandbox_list_users

**What it does**: Lists all users in the sandbox account.

**When to use**: View team members in sandbox.

**Arguments**: None

**Example LLM prompt**: "List all users in sandbox"

---

### freeagent_sandbox_get_user

**What it does**: Gets details of the authenticated user.

**When to use**: Verify authentication in sandbox.

**Arguments**: None

**Example LLM prompt**: "Get my sandbox profile"

---

### freeagent_sandbox_list_contacts

**What it does**: Lists all contacts.

**When to use**: Browse contact database in sandbox.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all contacts"

---

### freeagent_sandbox_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: View contact information.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact abc123"

---

### freeagent_sandbox_list_invoices

**What it does**: Lists all invoices.

**When to use**: Browse invoice history in sandbox.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all invoices"

---

### freeagent_sandbox_get_invoice

**What it does**: Gets details of a specific invoice.

**When to use**: View invoice details.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get invoice xyz789"

---

### freeagent_sandbox_list_bank_accounts

**What it does**: Lists all bank accounts.

**When to use**: View connected bank accounts.

**Arguments**: None

**Example LLM prompt**: "List all bank accounts"

---

### freeagent_sandbox_get_bank_account

**What it does**: Gets details of a specific bank account.

**When to use**: View account balance.

**Arguments**:
- `id` (required): Bank account ID

**Example LLM prompt**: "Get bank account def456"

---

### freeagent_sandbox_list_projects

**What it does**: Lists all projects.

**When to use**: Browse project list.

**Arguments**:
- `status` (optional): Filter by status (Active, Archived)

**Example LLM prompt**: "List all active projects"

---

### freeagent_sandbox_get_project

**What it does**: Gets details of a specific project.

**When to use**: View project information.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get project ghi789"

---

## Freeagent Sandbox Notes

- **Sandbox**: Testing environment separate from production
- **Same API**: Identical API structure as production Freeagent
- **Safe Testing**: Test invoices, contacts, and workflows safely
- **OAuth**: Uses sandbox authentication URLs
- **Users**: Team members for testing
- **Invoices/Bank Accounts/Projects**: Same as production API
