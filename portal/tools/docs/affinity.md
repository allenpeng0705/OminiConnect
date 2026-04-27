# Affinity Integration

Affinity is a relationship intelligence platform that helps teams manage relationships, organizations, and opportunities.

## Authentication

Affinity uses OAuth 2.0 for authentication. Configure the following scopes in your Affinity OAuth application:

- `persons:read` - Read access to persons
- `persons:write` - Write access to persons
- `organizations:read` - Read access to organizations
- `organizations:write` - Write access to organizations
- `opportunities:read` - Read access to opportunities
- `opportunities:write` - Write access to opportunities
- `lists:read` - Read access to lists

## Base URL

```
https://api.affinity.co/v1
```

## Rate Limits

- Standard rate limit: 1000 requests per hour
- Write operations: 100 requests per hour

## Tools

### Persons

| Tool | Description |
|------|-------------|
| `affinity_list_persons` | Retrieve a list of all persons in the relationship intelligence system |
| `affinity_get_person` | Retrieve details of a specific person by ID |
| `affinity_create_person` | Create a new person in the relationship intelligence system |

### Organizations

| Tool | Description |
|------|-------------|
| `affinity_list_organizations` | Retrieve a list of all organizations |
| `affinity_get_organization` | Retrieve details of a specific organization by ID |

### Opportunities

| Tool | Description |
|------|-------------|
| `affinity_list_opportunities` | Retrieve a list of all opportunities (deals) |
| `affinity_get_opportunity` | Retrieve details of a specific opportunity by ID |
| `affinity_create_opportunity` | Create a new opportunity (deal) in the pipeline |

### Lists

| Tool | Description |
|------|-------------|
| `affinity_list_lists` | Retrieve a list of all lists in the organization |
| `affinity_get_list` | Retrieve details of a specific list including persons and fields |

## Tool Details

### affinity_list_persons

Retrieve a list of all persons in the relationship intelligence system.

**Endpoint:** `GET /persons`

**Scopes:** `persons:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `organization_id` | string | No | Filter persons by organization ID |
| `name` | string | No | Search persons by name |

### affinity_get_person

Retrieve details of a specific person by ID.

**Endpoint:** `GET /persons/{person_id}`

**Scopes:** `persons:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `person_id` | string | Yes | The unique identifier of the person |

### affinity_create_person

Create a new person in the relationship intelligence system.

**Endpoint:** `POST /persons`

**Scopes:** `persons:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | The name of the person |
| `email` | string | Yes | The email address of the person |
| `organization_id` | string | No | The organization ID to associate with the person |

### affinity_list_organizations

Retrieve a list of all organizations.

**Endpoint:** `GET /organizations`

**Scopes:** `organizations:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | No | Search organizations by name |
| `industry` | string | No | Filter organizations by industry |

### affinity_get_organization

Retrieve details of a specific organization by ID.

**Endpoint:** `GET /organizations/{organization_id}`

**Scopes:** `organizations:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `organization_id` | string | Yes | The unique identifier of the organization |

### affinity_list_opportunities

Retrieve a list of all opportunities (deals).

**Endpoint:** `GET /opportunities`

**Scopes:** `opportunities:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | string | No | Filter opportunities by status |
| `organization_id` | string | No | Filter opportunities by organization ID |

### affinity_get_opportunity

Retrieve details of a specific opportunity by ID.

**Endpoint:** `GET /opportunities/{opportunity_id}`

**Scopes:** `opportunities:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `opportunity_id` | string | Yes | The unique identifier of the opportunity |

### affinity_create_opportunity

Create a new opportunity (deal) in the pipeline.

**Endpoint:** `POST /opportunities`

**Scopes:** `opportunities:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | The name of the opportunity |
| `value` | number | No | The monetary value of the opportunity |
| `organization_id` | string | Yes | The organization ID associated with the opportunity |
| `stage` | string | No | The pipeline stage of the opportunity |

### affinity_list_lists

Retrieve a list of all lists in the organization.

**Endpoint:** `GET /lists`

**Scopes:** `lists:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | No | Search lists by name |

### affinity_get_list

Retrieve details of a specific list including persons and fields.

**Endpoint:** `GET /lists/{list_id}`

**Scopes:** `lists:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `list_id` | string | Yes | The unique identifier of the list |