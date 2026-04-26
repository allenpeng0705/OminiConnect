# Salesforce-sandbox Tools

Provider: `salesforce-sandbox` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Salesforce Sandbox is a testing environment for Salesforce CRM. These tools allow AI agents to test contact, lead, opportunity, and account management without affecting production data.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Salesforce Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `contacts:read`, `contacts:write`, `leads:read`, `leads:write`, `opportunities:read`, `accounts:read`

**Important**: This is the sandbox environment. All operations are test operations and do not affect production data.

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `salesforce-sandbox_list_contacts` | List all contacts | GET | /v1/contacts |
| `salesforce-sandbox_get_contact` | Get contact details | GET | /v1/contacts/{contactId} |
| `salesforce-sandbox_create_contact` | Create a contact | POST | /v1/contacts |
| `salesforce-sandbox_list_leads` | List all leads | GET | /v1/leads |
| `salesforce-sandbox_get_lead` | Get lead details | GET | /v1/leads/{leadId} |
| `salesforce-sandbox_create_lead` | Create a lead | POST | /v1/leads |
| `salesforce-sandbox_list_opportunities` | List opportunities | GET | /v1/opportunities |
| `salesforce-sandbox_get_opportunity` | Get opportunity details | GET | /v1/opportunities/{opportunityId} |
| `salesforce-sandbox_list_accounts` | List all accounts | GET | /v1/accounts |
| `salesforce-sandbox_get_account` | Get account details | GET | /v1/accounts/{accountId} |

---

## Tool Details

### salesforce-sandbox_list_contacts

**What it does**: Returns a list of all contacts.

**When to use**: Test contact listing.

**Arguments**:
- `limit` (optional): Number of contacts (default 50)

**Example LLM prompt**: "List all test contacts"

---

### salesforce-sandbox_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: Test contact retrieval.

**Arguments**:
- `contactId` (required): The contact ID

**Example LLM prompt**: "Get details for contact cnt_test123"

---

### salesforce-sandbox_create_contact

**What it does**: Creates a new contact in sandbox.

**When to use**: Test contact creation.

**Arguments**:
- `firstName` (optional): First name
- `lastName` (required): Last name
- `email` (optional): Email address

**Example LLM prompt**: "Create a test contact for John Smith"

---

### salesforce-sandbox_list_leads

**What it does**: Returns a list of all leads.

**When to use**: Test lead listing.

**Arguments**:
- `limit` (optional): Number of leads (default 50)

**Example LLM prompt**: "List all test leads"

---

### salesforce-sandbox_get_lead

**What it does**: Gets details of a specific lead.

**When to use**: Test lead retrieval.

**Arguments**:
- `leadId` (required): The lead ID

**Example LLM prompt**: "Get details for lead ld_test789"

---

### salesforce-sandbox_create_lead

**What it does**: Creates a new lead in sandbox.

**When to use**: Test lead creation.

**Arguments**:
- `firstName` (optional): First name
- `lastName` (required): Last name
- `email` (optional): Email address
- `company` (required): Company name

**Example LLM prompt**: "Create a test lead for Test Company"

---

### salesforce-sandbox_list_opportunities

**What it does**: Returns a list of all opportunities.

**When to use**: Test opportunity listing.

**Arguments**:
- `limit` (optional): Number of opportunities (default 50)

**Example LLM prompt**: "List all test opportunities"

---

### salesforce-sandbox_get_opportunity

**What it does**: Gets details of an opportunity.

**When to use**: Test opportunity retrieval.

**Arguments**:
- `opportunityId` (required): The opportunity ID

**Example LLM prompt**: "Get details for opportunity opp_test123"

---

### salesforce-sandbox_list_accounts

**What it does**: Returns a list of all accounts.

**When to use**: Test account listing.

**Arguments**:
- `limit` (optional): Number of accounts (default 50)

**Example LLM prompt**: "List all test accounts"

---

### salesforce-sandbox_get_account

**What it does**: Gets details of an account.

**When to use**: Test account retrieval.

**Arguments**:
- `accountId` (required): The account ID

**Example LLM prompt**: "Get details for account acc_test123"

---

## Salesforce Sandbox Notes

- This is a TEST environment - data does not affect production
- Use for testing integrations and workflows
- IDs are sandbox-specific and not valid in production
- May have limited functionality compared to production
