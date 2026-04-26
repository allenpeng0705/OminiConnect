# SugarCRM Tools

Provider: `sugarcrm` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the SugarCRM REST API. They allow AI agents to manage accounts, contacts, leads, and opportunities. SugarCRM is an enterprise CRM platform with strong customization capabilities.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with SugarCRM
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sugarcrm_list_accounts` | List all accounts | GET | /Accounts |
| `sugarcrm_get_account` | Get account details | GET | /Accounts/{id} |
| `sugarcrm_create_account` | Create a new account | POST | /Accounts |
| `sugarcrm_list_contacts` | List all contacts | GET | /Contacts |
| `sugarcrm_get_contact` | Get contact details | GET | /Contacts/{id} |
| `sugarcrm_create_contact` | Create a new contact | POST | /Contacts |
| `sugarcrm_list_leads` | List all leads | GET | /Leads |
| `sugarcrm_get_lead` | Get lead details | GET | /Leads/{id} |
| `sugarcrm_list_opportunities` | List all opportunities | GET | /Opportunities |
| `sugarcrm_get_opportunity` | Get opportunity details | GET | /Opportunities/{id} |

---

## Tool Details

### sugarcrm_list_accounts

**What it does**: Lists all accounts/companies in SugarCRM with optional filtering.

**When to use**: Browse companies, find accounts by industry or territory.

**Arguments**:
- `filter` (optional): Filter expressions
- `fields` (optional): Fields to include (comma-separated)
- `max_results` (optional): Max results (default 20, max 1000)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all accounts in the technology industry"

---

### sugarcrm_get_account

**What it does**: Gets detailed information about a specific account including contacts, opportunities, and activities.

**When to use**: Get full account details before updating or linking records.

**Arguments**:
- `id` (required): Account ID

**Example LLM prompt**: "Get details for account 12345"

---

### sugarcrm_create_account

**What it does**: Creates a new account/company with name, industry, phone, website, and address.

**When to use**: Add a new company to the CRM.

**Arguments**:
- `name` (required): Account name
- `industry` (optional): Industry type
- `phone` (optional): Phone number
- `website` (optional): Website URL
- `billing_address_street` (optional): Billing street address
- `billing_address_city` (optional): Billing city
- `billing_address_state` (optional): Billing state
- `billing_address_postalcode` (optional): Billing postal code
- `billing_address_country` (optional): Billing country

**Example LLM prompt**: "Create a new account for Acme Corp in the technology industry"

---

### sugarcrm_list_contacts

**What it does**: Lists all contacts in SugarCRM with optional filtering by account or search term.

**When to use**: Find contacts, see all people at a company.

**Arguments**:
- `filter` (optional): Filter expressions
- `fields` (optional): Fields to include (comma-separated)
- `max_results` (optional): Max results (default 20, max 1000)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all contacts at Acme Corp"

---

### sugarcrm_get_contact

**What it does**: Gets detailed information about a specific contact including email, phone, account, and tasks.

**When to use**: Get contact details before calling or emailing.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 456"

---

### sugarcrm_create_contact

**What it does**: Creates a new contact with first name, last name, email, phone, and account association.

**When to use**: Add a new contact to the CRM.

**Arguments**:
- `first_name` (optional): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone_work` (optional): Work phone
- `phone_mobile` (optional): Mobile phone
- `account_id` (optional): Associated account ID
- `title` (optional): Job title

**Example LLM prompt**: "Create a new contact named John Smith with email john@acme.com"

---

### sugarcrm_list_leads

**What it does**: Lists all leads in SugarCRM with optional filtering by status, source, or assignment.

**When to use**: View lead pipeline, find leads needing follow-up.

**Arguments**:
- `filter` (optional): Filter expressions
- `fields` (optional): Fields to include (comma-separated)
- `max_results` (optional): Max results (default 20, max 1000)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all new leads from this week"

---

### sugarcrm_get_lead

**What it does**: Gets detailed information about a specific lead including status, source, and conversion details.

**When to use**: Get lead details before converting or assigning.

**Arguments**:
- `id` (required): Lead ID

**Example LLM prompt**: "Get details for lead 789"

---

### sugarcrm_list_opportunities

**What it does**: Lists all opportunities in SugarCRM with optional filtering by sales stage, amount, or account.

**When to use**: View pipeline, find opportunities at a certain stage.

**Arguments**:
- `filter` (optional): Filter expressions
- `fields` (optional): Fields to include (comma-separated)
- `max_results` (optional): Max results (default 20, max 1000)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all opportunities in the negotiation stage"

---

### sugarcrm_get_opportunity

**What it does**: Gets detailed information about a specific opportunity including amount, stage, account, and expected close date.

**When to use**: Get opportunity details before updating or closing.

**Arguments**:
- `id` (required): Opportunity ID

**Example LLM prompt**: "Get details for opportunity 321"

---

## SugarCRM API Notes

- **Module-based API**: SugarCRM uses a module-based REST API (Accounts, Contacts, Leads, Opportunities).
- **Lead conversion**: Leads can be converted to Contacts, Accounts, and Opportunities.
- **Relationship-based**: Contacts are linked to Accounts; Opportunities are linked to Accounts and Contacts.
- **Filtering**: SugarCRM supports filter expressions for advanced querying.
