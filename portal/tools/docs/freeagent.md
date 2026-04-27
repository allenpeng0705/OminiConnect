# Freeagent Tools

Provider: `freeagent` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Freeagent API. They allow AI agents to manage users, contacts, invoices, bank accounts, and projects. Freeagent is a cloud accounting platform for freelancers and small businesses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Freeagent
- Token stored in Nango, accessed via `connection_ref`
- OAuth2 with Basic auth for token request
- Approval URL: https://api.freeagent.com/v2/approve_app

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `freeagent_list_users` | List users | GET | /v2/users |
| `freeagent_get_user` | Get user details | GET | /v2/me |
| `freeagent_list_contacts` | List contacts | GET | /v2/contacts |
| `freeagent_get_contact` | Get contact details | GET | /v2/contacts/{id} |
| `freeagent_list_invoices` | List invoices | GET | /v2/invoices |
| `freeagent_get_invoice` | Get invoice details | GET | /v2/invoices/{id} |
| `freeagent_list_bank_accounts` | List bank accounts | GET | /v2/bank_accounts |
| `freeagent_get_bank_account` | Get bank account details | GET | /v2/bank_accounts/{id} |
| `freeagent_list_projects` | List projects | GET | /v2/projects |
| `freeagent_get_project` | Get project details | GET | /v2/projects/{id} |

---

## Tool Details

### freeagent_list_users

**What it does**: Lists all users in the account.

**When to use**: View team members.

**Arguments**: None

**Example LLM prompt**: "List all users"

---

### freeagent_get_user

**What it does**: Gets details of the authenticated user.

**When to use**: Verify authentication, get user info.

**Arguments**: None

**Example LLM prompt**: "Get my Freeagent profile"

---

### freeagent_list_contacts

**What it does**: Lists all contacts.

**When to use**: Browse contact database.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all contacts"

---

### freeagent_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: View contact information.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact abc123"

---

### freeagent_list_invoices

**What it does**: Lists all invoices.

**When to use**: Browse invoice history.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all invoices"

---

### freeagent_get_invoice

**What it does**: Gets details of a specific invoice.

**When to use**: View invoice details.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get invoice xyz789"

---

### freeagent_list_bank_accounts

**What it does**: Lists all bank accounts.

**When to use**: View connected bank accounts.

**Arguments**: None

**Example LLM prompt**: "List all bank accounts"

---

### freeagent_get_bank_account

**What it does**: Gets details of a specific bank account.

**When to use**: View account balance, transactions.

**Arguments**:
- `id` (required): Bank account ID

**Example LLM prompt**: "Get bank account def456"

---

### freeagent_list_projects

**What it does**: Lists all projects.

**When to use**: Browse project list.

**Arguments**:
- `status` (optional): Filter by status (Active, Archived)

**Example LLM prompt**: "List all active projects"

---

### freeagent_get_project

**What it does**: Gets details of a specific project.

**When to use**: View project information.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get project ghi789"

---

## Freeagent API Notes

- **Users**: Team members with access to the account
- **Contacts**: Customers and suppliers
- **Invoices**: Sales invoices with payment tracking
- **Bank Accounts**: Connected bank accounts for reconciliation
- **Projects**: Projects for tracking time and billing
- **OAuth**: Requires app registration with Freeagent
