# Salesforce-jwt Tools

Provider: `salesforce-jwt` | Engine: `nango` | Auth: JWT Bearer Token OAuth via Nango

## Overview

Salesforce with JWT Bearer Token authentication for secure server-to-server API access. These tools allow AI agents to manage contacts, leads, opportunities, and accounts.

## Authentication

**Nango OAuth JWT Bearer Token**:
- Uses connected app credentials (client ID, client secret, JWT key)
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `contacts:read`, `contacts:write`, `leads:read`, `leads:write`, `opportunities:read`, `accounts:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `salesforce-jwt_list_contacts` | List all contacts | GET | /v1/contacts |
| `salesforce-jwt_get_contact` | Get contact details | GET | /v1/contacts/{contactId} |
| `salesforce-jwt_create_contact` | Create a contact | POST | /v1/contacts |
| `salesforce-jwt_list_leads` | List all leads | GET | /v1/leads |
| `salesforce-jwt_get_lead` | Get lead details | GET | /v1/leads/{leadId} |
| `salesforce-jwt_create_lead` | Create a lead | POST | /v1/leads |
| `salesforce-jwt_list_opportunities` | List opportunities | GET | /v1/opportunities |
| `salesforce-jwt_get_opportunity` | Get opportunity details | GET | /v1/opportunities/{opportunityId} |
| `salesforce-jwt_list_accounts` | List all accounts | GET | /v1/accounts |
| `salesforce-jwt_get_account` | Get account details | GET | /v1/accounts/{accountId} |

---

## Tool Details

### salesforce-jwt_list_contacts

**What it does**: Returns a list of all contacts.

**When to use**: View contact database.

**Arguments**:
- `limit` (optional): Number of contacts (default 50)

**Example LLM prompt**: "List all contacts"

---

### salesforce-jwt_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: Get contact information.

**Arguments**:
- `contactId` (required): The contact ID

**Example LLM prompt**: "Get details for contact cnt_abc123"

---

### salesforce-jwt_create_contact

**What it does**: Creates a new contact.

**When to use**: Add new contacts.

**Arguments**:
- `firstName` (optional): First name
- `lastName` (required): Last name
- `email` (optional): Email address
- `accountId` (optional): Related account ID

**Example LLM prompt**: "Create a contact for John Smith"

---

### salesforce-jwt_list_leads

**What it does**: Returns a list of all leads.

**When to use**: View sales leads.

**Arguments**:
- `limit` (optional): Number of leads (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all new leads"

---

### salesforce-jwt_get_lead

**What it does**: Gets details of a specific lead.

**When to use**: Get lead information.

**Arguments**:
- `leadId` (required): The lead ID

**Example LLM prompt**: "Get details for lead ld_xyz789"

---

### salesforce-jwt_create_lead

**What it does**: Creates a new lead.

**When to use**: Add new sales prospects.

**Arguments**:
- `firstName` (optional): First name
- `lastName` (required): Last name
- `email` (optional): Email address
- `company` (required): Company name

**Example LLM prompt**: "Create a lead for Jane from Acme Corp"

---

### salesforce-jwt_list_opportunities

**What it does**: Returns a list of opportunities.

**When to use**: View sales pipeline.

**Arguments**:
- `limit` (optional): Number of opportunities (default 50)
- `stage` (optional): Filter by stage

**Example LLM prompt**: "List all opportunities in negotiation"

---

### salesforce-jwt_get_opportunity

**What it does**: Gets details of an opportunity.

**When to use**: Get deal information.

**Arguments**:
- `opportunityId` (required): The opportunity ID

**Example LLM prompt**: "Get details for opportunity opp_123"

---

### salesforce-jwt_list_accounts

**What it does**: Returns a list of all accounts.

**When to use**: View customer accounts.

**Arguments**:
- `limit` (optional): Number of accounts (default 50)

**Example LLM prompt**: "List all accounts"

---

### salesforce-jwt_get_account

**What it does**: Gets details of an account.

**When to use**: Get account information.

**Arguments**:
- `accountId` (required): The account ID

**Example LLM prompt**: "Get details for account acc_abc123"

---

## Salesforce JWT Notes

- JWT auth enables secure server-to-server integration
- Same API as standard Salesforce OAuth
- All objects: Contacts, Leads, Opportunities, Accounts
- Ideal for backend integrations and automation
