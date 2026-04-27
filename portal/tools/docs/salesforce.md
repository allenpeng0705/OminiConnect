# Salesforce CRM

## Overview

Salesforce is a cloud-based customer relationship management (CRM) platform that helps businesses manage sales, customer service, marketing, and more. This tool registry provides AI agents with access to core Salesforce objects: Accounts, Contacts, Leads, and Opportunities.

## Authentication

Salesforce uses OAuth 2.0 authentication via Nango. Ensure you have connected your Salesforce account through the OminiConnect portal before using these tools.

## Rate Limits

- **API Calls**: Varies by Salesforce edition (typically 100k+ per day)
- **Concurrent Requests**: 5 per org (for standard REST API)
- **Bulk API**: Separate limits apply

## Tool Categories

### Accounts

| Tool | Description |
|------|-------------|
| `salesforce_list_accounts` | List Salesforce accounts (companies/organizations). Filter by industry, type, or location. |
| `salesforce_get_account` | Get detailed information about a Salesforce account including name, industry, revenue, and contacts. |
| `salesforce_create_account` | Create a new account in Salesforce. Set company name, industry, type, and contact information. |

### Contacts

| Tool | Description |
|------|-------------|
| `salesforce_list_contacts` | List Salesforce contacts. Filter by account, title, department, or state. |
| `salesforce_get_contact` | Get detailed information about a Salesforce contact including name, email, phone, and account. |
| `salesforce_create_contact` | Create a new contact in Salesforce. Set name, email, phone, and account association. |

### Leads

| Tool | Description |
|------|-------------|
| `salesforce_list_leads` | List Salesforce leads with optional filters. Filter by status, source, industry, rating, or created date. |
| `salesforce_get_lead` | Get detailed information about a Salesforce lead including name, company, status, and source. |

### Opportunities

| Tool | Description |
|------|-------------|
| `salesforce_list_opportunities` | List Salesforce opportunities. Filter by stage, type, close date, or account. |
| `salesforce_get_opportunity` | Get detailed information about an opportunity including name, amount, stage, close date, and account. |

---

## Tool Details

### salesforce_list_accounts

List Salesforce accounts (companies/organizations). Filter by industry, type, or location.

**Endpoint**: `GET /services/data/v59.0/query`

**Scopes Required**: `api`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| industry | string | Filter by industry |
| type | string | Filter by account type |
| limit | integer | Max results (default 20, max 200) |

**Returns**: List of account records matching filters.

---

### salesforce_get_account

Get detailed information about a Salesforce account including name, industry, revenue, and contacts.

**Endpoint**: `GET /services/data/v59.0/sobjects/Account/{accountId}`

**Scopes Required**: `api`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| accountId | string | Account ID |

**Returns**: Full account record including all standard and custom fields.

---

### salesforce_create_account

Create a new account in Salesforce. Set company name, industry, type, and contact information.

**Endpoint**: `POST /services/data/v59.0/sobjects/Account`

**Scopes Required**: `api`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| Name | string | Yes | Account/Company name |
| Industry | string | No | Industry (e.g., Technology, Healthcare, Finance) |
| Type | string | No | Account type (Prospect, Customer, Competitor) |
| Website | string | No | Website URL |
| Phone | string | No | Phone number |

**Returns**: Created account record with ID.

---

### salesforce_list_contacts

List Salesforce contacts. Filter by account, title, department, or state.

**Endpoint**: `GET /services/data/v59.0/query`

**Scopes Required**: `api`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| accountId | string | Filter by account ID |
| title | string | Filter by job title |
| department | string | Filter by department |
| limit | integer | Max results (default 20, max 200) |

**Returns**: List of contact records matching filters.

---

### salesforce_get_contact

Get detailed information about a Salesforce contact including name, email, phone, and account.

**Endpoint**: `GET /services/data/v59.0/sobjects/Contact/{contactId}`

**Scopes Required**: `api`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| contactId | string | Contact ID |

**Returns**: Full contact record including all standard and custom fields.

---

### salesforce_create_contact

Create a new contact in Salesforce. Set name, email, phone, and account association.

**Endpoint**: `POST /services/data/v59.0/sobjects/Contact`

**Scopes Required**: `api`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| FirstName | string | No | First name |
| LastName | string | Yes | Last name |
| Email | string | No | Email address |
| Phone | string | No | Phone number |
| MobilePhone | string | No | Mobile phone number |
| Title | string | No | Job title |
| Department | string | No | Department |
| AccountId | string | No | Associated account ID |

**Returns**: Created contact record with ID.

---

### salesforce_list_leads

List Salesforce leads with optional filters. Filter by status, source, industry, rating, or created date.

**Endpoint**: `GET /services/data/v59.0/query`

**Scopes Required**: `api`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| status | string | Lead status (Open, Working, Closed - Converted, Closed - Not Converted) |
| leadSource | string | Lead source (Web, Referral, Partner, Trade Show, etc.) |
| industry | string | Filter by industry |
| rating | string | Rating (Hot, Warm, Cold) |
| limit | integer | Max results (default 20, max 200) |

**Returns**: List of lead records matching filters.

---

### salesforce_get_lead

Get detailed information about a Salesforce lead including name, company, status, and source.

**Endpoint**: `GET /services/data/v59.0/sobjects/Lead/{leadId}`

**Scopes Required**: `api`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| leadId | string | Lead ID |

**Returns**: Full lead record including all standard and custom fields.

---

### salesforce_list_opportunities

List Salesforce opportunities. Filter by stage, type, close date, or account.

**Endpoint**: `GET /services/data/v59.0/query`

**Scopes Required**: `api`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| stage | string | Opportunity stage (Prospecting, Qualification, etc.) |
| type | string | Opportunity type (New Business, Existing Business) |
| closeDate | string | Filter by close date (This Month, Next Month, This Quarter) |
| accountId | string | Filter by account ID |
| limit | integer | Max results (default 20, max 200) |

**Returns**: List of opportunity records matching filters.

---

### salesforce_get_opportunity

Get detailed information about an opportunity including name, amount, stage, close date, and account.

**Endpoint**: `GET /services/data/v59.0/sobjects/Opportunity/{opportunityId}`

**Scopes Required**: `api`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| opportunityId | string | Opportunity ID |

**Returns**: Full opportunity record including all standard and custom fields.

---

## Common Use Cases

1. **Lead Management**: List and qualify leads before converting to contacts/opportunities
2. **Account Review**: Get account details to understand customer context before calls
3. **Opportunity Tracking**: Monitor deal progress through pipeline stages
4. **Contact Enrichment**: Create/update contacts with information gathered during conversations

## Notes

- All list endpoints use Salesforce SOQL queries for flexible filtering
- IDs are 18-character Salesforce record IDs (case-insensitive)
- Timestamps are returned in ISO 8601 format
- Custom fields are supported but may vary by org configuration