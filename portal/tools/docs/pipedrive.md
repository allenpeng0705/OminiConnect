# Pipedrive CRM

## Overview

Pipedrive is a sales-focused CRM platform designed for pipeline management and deal tracking. This tool registry provides AI agents with access to core Pipedrive objects: Persons (contacts), Deals, Activities, and Pipelines.

## Authentication

Pipedrive uses API token authentication via Nango. Ensure you have connected your Pipedrive account through the OminiConnect portal before using these tools.

## Rate Limits

- **API Calls**: 100 per 30 seconds (standard tier)
- **Daily Limit**: Varies by plan (1,000 - 100,000+ calls/day)
- **Batch Operations**: Supported for bulk updates

## Tool Categories

### Persons (Contacts)

| Tool | Description |
|------|-------------|
| `pipedrive_list_persons` | Retrieve a list of persons (contacts) from Pipedrive CRM. Filter by name, email, organization, or custom fields. |
| `pipedrive_get_person` | Retrieve detailed information for a specific person by ID including name, email, phone, and organization. |
| `pipedrive_create_person` | Create a new person (contact) in Pipedrive CRM. Set name, email, phone, and organization. |

### Deals

| Tool | Description |
|------|-------------|
| `pipedrive_list_deals` | Retrieve a list of deals from Pipedrive CRM. Filter by stage, status, pipeline, or organization. |
| `pipedrive_get_deal` | Retrieve detailed information for a specific deal by ID including title, value, stage, and activities. |
| `pipedrive_create_deal` | Create a new deal in Pipedrive CRM. Set title, value, currency, stage, and associations. |

### Activities

| Tool | Description |
|------|-------------|
| `pipedrive_list_activities` | Retrieve a list of activities (calls, meetings, tasks, notes) from Pipedrive. Filter by type, assignee, or date range. |
| `pipedrive_get_activity` | Retrieve detailed information for a specific activity by ID including type, subject, and participants. |

### Pipelines

| Tool | Description |
|------|-------------|
| `pipedrive_list_pipelines` | Retrieve all sales pipelines and their stages from Pipedrive. Use to see available pipelines and stage definitions. |
| `pipedrive_get_pipeline` | Retrieve detailed information for a specific pipeline by ID including stages, deal counts, and totals. |

---

## Tool Details

### pipedrive_list_persons

Retrieve a list of persons (contacts) from Pipedrive CRM. Filter by name, email, organization, or custom fields.

**Endpoint**: `GET /v1/persons`

**Scopes Required**: `persons:read`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| start | integer | Pagination start value (default 0) |
| limit | integer | Number of results per page (max 100, default 50) |
| filter_id | integer | Filter ID to apply |
| sort_by | string | Field to sort by (e.g., name, create_time) |
| sort_order | string | asc or desc |
| search | string | Search term for name/email |

**Returns**: List of person records matching filters.

---

### pipedrive_get_person

Retrieve detailed information for a specific person by ID including name, email, phone, and organization.

**Endpoint**: `GET /v1/persons/{id}`

**Scopes Required**: `persons:read`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| id | integer | Person ID |

**Returns**: Full person record with all fields and organization association.

---

### pipedrive_create_person

Create a new person (contact) in Pipedrive CRM. Set name, email, phone, and organization.

**Endpoint**: `POST /v1/persons`

**Scopes Required**: `persons:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| name | string | Yes | Person name |
| email | array | No | Email addresses array with label and value |
| phone | array | No | Phone numbers array with label and value |
| org_id | integer | No | Organization ID to associate |
| owner_id | integer | No | Owner user ID |

**Returns**: Created person record with ID.

---

### pipedrive_list_deals

Retrieve a list of deals from Pipedrive CRM. Filter by stage, status, pipeline, or organization.

**Endpoint**: `GET /v1/deals`

**Scopes Required**: `deals:read`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| start | integer | Pagination start value (default 0) |
| limit | integer | Number of results per page (max 100, default 50) |
| filter_id | integer | Filter ID to apply |
| sort_by | string | Field to sort by (e.g., title, value, status) |
| sort_order | string | asc or desc |
| status | string | open, won, lost, or all |
| pipeline_id | integer | Filter by pipeline ID |

**Returns**: List of deal records matching filters.

---

### pipedrive_get_deal

Retrieve detailed information for a specific deal by ID including title, value, stage, and activities.

**Endpoint**: `GET /v1/deals/{id}`

**Scopes Required**: `deals:read`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| id | integer | Deal ID |

**Returns**: Full deal record with all fields and activity history.

---

### pipedrive_create_deal

Create a new deal in Pipedrive CRM. Set title, value, currency, stage, and associations.

**Endpoint**: `POST /v1/deals`

**Scopes Required**: `deals:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| title | string | Yes | Deal title |
| value | number | No | Deal value/amount |
| currency | string | No | Currency code (e.g., USD, EUR, default USD) |
| person_id | integer | No | Associated person ID |
| org_id | integer | No | Organization ID |
| stage_id | integer | No | Pipeline stage ID |
| pipeline_id | integer | No | Pipeline ID |
| close_date | string | No | Expected close date (YYYY-MM-DD) |
| visible_to | string | No | Visibility: assigned_owner, entire_company |

**Returns**: Created deal record with ID.

---

### pipedrive_list_activities

Retrieve a list of activities (calls, meetings, tasks, notes) from Pipedrive. Filter by type, assignee, or date range.

**Endpoint**: `GET /v1/activities`

**Scopes Required**: `activities:read`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| start | integer | Pagination start value (default 0) |
| limit | integer | Number of results per page (max 100, default 50) |
| filter_id | integer | Filter ID to apply |
| type | string | Activity type: call, meeting, task, note, deadline, lunch |
| user_id | integer | Filter by assignee user ID |
| sort_by | string | Field to sort by |
| sort_order | string | asc or desc |
| done | integer | 0 for undone, 1 for done, -1 for all |

**Returns**: List of activity records matching filters.

---

### pipedrive_get_activity

Retrieve detailed information for a specific activity by ID including type, subject, and participants.

**Endpoint**: `GET /v1/activities/{id}`

**Scopes Required**: `activities:read`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| id | integer | Activity ID |

**Returns**: Full activity record with all fields and participants.

---

### pipedrive_list_pipelines

Retrieve all sales pipelines and their stages from Pipedrive. Use to see available pipelines and stage definitions.

**Endpoint**: `GET /v1/pipelines`

**Scopes Required**: `pipelines:read`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| start | integer | Pagination start value (default 0) |
| limit | integer | Number of results per page (default 50) |
| include_inactive | string | Include inactive pipelines (yes/no, default no) |

**Returns**: List of pipeline records with stages.

---

### pipedrive_get_pipeline

Retrieve detailed information for a specific pipeline by ID including stages, deal counts, and totals.

**Endpoint**: `GET /v1/pipelines/{id}`

**Scopes Required**: `pipelines:read`

**Parameters**:
| Name | Type | Description |
|------|------|-------------|
| id | integer | Pipeline ID |

**Returns**: Full pipeline record with stages, deal counts, and values.

---

## Common Use Cases

1. **Pipeline Management**: View pipelines to understand deal stages before interacting with deals
2. **Deal Tracking**: Create and track deals through the sales process
3. **Activity Logging**: Record calls, meetings, and tasks associated with deals
4. **Contact Management**: Maintain person records with organization associations

## Notes

- Pipedrive uses numeric IDs for all records
- Activity types include: call, meeting, task, note, deadline, lunch
- Deals must be associated with a pipeline and stage
- Persons can be linked to organizations for better CRM hygiene
- Visibility settings control who can see the record