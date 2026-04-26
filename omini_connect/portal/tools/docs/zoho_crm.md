# Zoho CRM

## Overview

Zoho CRM is a cloud-based CRM platform that offers sales automation, marketing, and customer service features. This tool registry provides AI agents with access to core Zoho CRM modules: Contacts, Accounts, Deals, Tasks, and Leads.

## Authentication

Zoho uses OAuth 2.0 authentication via Nango. Ensure you have connected your Zoho CRM account through the OminiConnect portal before using these tools.

## Rate Limits

- **API Calls**: 7,500 per minute (standard tier)
- **Daily Limit**: Varies by subscription
- **Concurrent Requests**: 5 per org

## Tool Categories

### Contacts

| Tool | Description |
|------|-------------|
| `zoho_crm_list_contacts` | List all contacts in Zoho CRM. Filter by email, phone, account, or tag. |
| `zoho_crm_get_contact` | Get detailed information about a specific contact including name, email, phone, account, and tags. |
| `zoho_crm_create_contact` | Create a new contact in Zoho CRM. Set name, email, phone, account association, and custom fields. |

### Accounts

| Tool | Description |
|------|-------------|
| `zoho_crm_list_accounts` | List all accounts (companies/organizations) in Zoho CRM. Filter by industry, type, or location. |
| `zoho_crm_get_account` | Get detailed information about a specific account including name, industry, website, and associated contacts. |

### Deals

| Tool | Description |
|------|-------------|
| `zoho_crm_list_deals` | List all deals in Zoho CRM. Filter by stage, amount, or closing date. |
| `zoho_crm_get_deal` | Get detailed information about a specific deal including name, amount, stage, contacts, and activities. |

### Tasks

| Tool | Description |
|------|-------------|
| `zoho_crm_list_tasks` | List all tasks in Zoho CRM. Filter by status, priority, or due date. |
| `zoho_crm_get_task` | Get detailed information about a specific task including subject, description, due date, and status. |

### Leads

| Tool | Description |
|------|-------------|
| `zoho_crm_list_leads` | List all leads in Zoho CRM. Filter by source, status, or industry. |

---

## Tool Details

### zoho_crm_list_contacts

List all contacts in Zoho CRM. Filter by email, phone, account, or tag.

**Endpoint**: `GET /crm/v2/Contacts`

**Scopes Required**: `ZohoCRM.modules.contacts.READ`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| page | integer | Page number (default 1) |
| per_page | integer | Results per page (default 200, max 200) |
| sort_by | string | Field to sort by (e.g., Email, Full_Name) |
| sort_order | string | Sort order (asc or desc) |
| phone | string | Filter by phone number |
| email | string | Filter by email address |

**Returns**: List of contact records matching filters.

---

### zoho_crm_get_contact

Get detailed information about a specific contact including name, email, phone, account, and tags.

**Endpoint**: `GET /crm/v2/Contacts/{id}`

**Scopes Required**: `ZohoCRM.modules.contacts.READ`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| id | string | Contact ID |

**Returns**: Full contact record including all standard and custom fields.

---

### zoho_crm_create_contact

Create a new contact in Zoho CRM. Set name, email, phone, account association, and custom fields.

**Endpoint**: `POST /crm/v2/Contacts`

**Scopes Required**: `ZohoCRM.modules.contacts.CREATE`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| First_Name | string | No | First name |
| Last_Name | string | No | Last name |
| Email | string | No | Email address |
| Phone | string | No | Phone number |
| Mobile | string | No | Mobile phone number |
| Account_Name | object | No | Associated account (use id or name) |
| Title | string | No | Job title |
| Department | string | No | Department |

**Returns**: Created contact record with ID.

---

### zoho_crm_list_accounts

List all accounts (companies/organizations) in Zoho CRM. Filter by industry, type, or location.

**Endpoint**: `GET /crm/v2/Accounts`

**Scopes Required**: `ZohoCRM.modules.accounts.READ`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| page | integer | Page number (default 1) |
| per_page | integer | Results per page (default 200, max 200) |
| sort_by | string | Field to sort by (e.g., Account_Name, Industry) |
| sort_order | string | Sort order (asc or desc) |
| industry | string | Filter by industry |

**Returns**: List of account records matching filters.

---

### zoho_crm_get_account

Get detailed information about a specific account including name, industry, website, and associated contacts.

**Endpoint**: `GET /crm/v2/Accounts/{id}`

**Scopes Required**: `ZohoCRM.modules.accounts.READ`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| id | string | Account ID |

**Returns**: Full account record including all standard and custom fields.

---

### zoho_crm_list_deals

List all deals in Zoho CRM. Filter by stage, amount, or closing date.

**Endpoint**: `GET /crm/v2/Deals`

**Scopes Required**: `ZohoCRM.modules.deals.READ`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| page | integer | Page number (default 1) |
| per_page | integer | Results per page (default 200, max 200) |
| sort_by | string | Field to sort by (e.g., Amount, Closing_Date) |
| sort_order | string | Sort order (asc or desc) |
| stage | string | Filter by deal stage |

**Returns**: List of deal records matching filters.

---

### zoho_crm_get_deal

Get detailed information about a specific deal including name, amount, stage, contacts, and activities.

**Endpoint**: `GET /crm/v2/Deals/{id}`

**Scopes Required**: `ZohoCRM.modules.deals.READ`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| id | string | Deal ID |

**Returns**: Full deal record including all standard and custom fields.

---

### zoho_crm_list_tasks

List all tasks in Zoho CRM. Filter by status, priority, or due date.

**Endpoint**: `GET /crm/v2/Tasks`

**Scopes Required**: `ZohoCRM.modules.tasks.READ`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| page | integer | Page number (default 1) |
| per_page | integer | Results per page (default 200, max 200) |
| status | string | Filter by status (Not Started, In Progress, Completed) |
| priority | string | Filter by priority (High, Medium, Low) |
| due_date_before | string | Due date before (YYYY-MM-DD) |
| due_date_after | string | Due date after (YYYY-MM-DD) |

**Returns**: List of task records matching filters.

---

### zoho_crm_get_task

Get detailed information about a specific task including subject, description, due date, and status.

**Endpoint**: `GET /crm/v2/Tasks/{id}`

**Scopes Required**: `ZohoCRM.modules.tasks.READ`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| id | string | Task ID |

**Returns**: Full task record including all standard and custom fields.

---

### zoho_crm_list_leads

List all leads in Zoho CRM. Filter by source, status, or industry.

**Endpoint**: `GET /crm/v2/Leads`

**Scopes Required**: `ZohoCRM.modules.leads.READ`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| page | integer | Page number (default 1) |
| per_page | integer | Results per page (default 200, max 200) |
| sort_by | string | Field to sort by (e.g., Email, First_Name) |
| sort_order | string | Sort order (asc or desc) |
| lead_source | string | Filter by lead source |
| status | string | Filter by status |

**Returns**: List of lead records matching filters.

---

## Common Use Cases

1. **Lead Management**: List and qualify leads before converting to contacts/deals
2. **Account Tracking**: Manage account information with associated contacts
3. **Deal Management**: Track deals through pipeline stages with amount and closing dates
4. **Task Management**: Create and track tasks for follow-ups and activities

## Notes

- Zoho CRM uses module-based API paths
- Record IDs are strings in Zoho CRM
- Pagination uses page/per_page rather than offset
- Module names in endpoints use singular form (Contact, not Contacts)
- Custom fields are supported and vary by org configuration