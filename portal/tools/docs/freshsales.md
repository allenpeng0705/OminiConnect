# Freshsales

## Overview

Freshsales is a CRM platform that provides tools for managing contacts, leads, accounts, deals, and activities. This integration allows AI agents to interact with Freshsales data through OminiConnect.

## Connection

### Authentication
- **Auth Type**: OAuth 2.0
- **Provider**: Freshsales
- **Scopes**: `crm.contacts.read`, `crm.contacts.write`, `crm.deals.read`, `crm.deals.write`, `crm.accounts.read`, `crm.accounts.write`, `crm.activities.read`

## Available Tools

### Contacts

#### list_contacts
Retrieve a paginated list of contacts from Freshsales CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `filter` | string | No | Filter type: all, walk_in, cold_call, marketing |
| `sort_by` | string | No | Field to sort by |
| `sort_order` | string | No | asc or desc |
| `per_page` | integer | No | Number of results per page (default 20) |
| `page` | integer | No | Page number for pagination |

#### get_contact
Retrieve detailed information for a specific contact by ID.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | integer | Yes | Contact ID |

#### create_contact
Create a new contact in Freshsales CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `first_name` | string | Yes | First name of the contact |
| `last_name` | string | Yes | Last name of the contact |
| `email` | string | No | Email address |
| `phone` | string | No | Phone number |
| `mobile` | string | No | Mobile number |

### Deals

#### list_deals
Retrieve a list of deals from Freshsales CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `filter` | string | No | Filter: all, open, won, lost |
| `sort_by` | string | No | Field to sort by |
| `sort_order` | string | No | asc or desc |
| `per_page` | integer | No | Number of results per page |
| `page` | integer | No | Page number |

#### get_deal
Retrieve detailed information for a specific deal by ID.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | integer | Yes | Deal ID |

#### create_deal
Create a new deal in Freshsales CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `title` | string | Yes | Deal title |
| `amount` | number | No | Deal amount |
| `pipeline` | string | No | Pipeline name |
| `stage` | string | No | Deal stage |
| `close_date` | string | No | Expected close date (YYYY-MM-DD) |

### Accounts

#### list_accounts
Retrieve a list of accounts (companies) from Freshsales CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `filter` | string | No | Filter type |
| `sort_by` | string | No | Field to sort by |
| `sort_order` | string | No | asc or desc |
| `per_page` | integer | No | Number of results per page |
| `page` | integer | No | Page number |

#### get_account
Retrieve detailed information for a specific account by ID.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | integer | Yes | Account ID |

### Activities

#### list_activities
Retrieve a list of activities (calls, meetings, notes) from Freshsales CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `filter` | string | No | Filter by type: all, call, meeting, note, email |
| `entity_type` | string | No | Related entity type (contact, deal, account) |
| `entity_id` | integer | No | Related entity ID |
| `per_page` | integer | No | Number of results per page |
| `page` | integer | No | Page number |

#### get_activity
Retrieve detailed information for a specific activity by ID.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | integer | Yes | Activity ID |

## Usage Example

```json
{
  "tool": "freshsales_list_contacts",
  "parameters": {
    "per_page": 10,
    "filter": "all"
  }
}
```

## Rate Limits

Please refer to Freshsales API documentation for current rate limits.