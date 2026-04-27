# Domo

Business intelligence platform for datasets, dataflows, cards, users, and projects.

## Authentication

Domo uses OAuth 2.0 authentication. The Nango integration handles token refresh automatically.

## API Endpoints

Domo API base URL: `https://api.domo.com`

## Tools

### Datasets

#### `domo_list_datasets`
List all datasets in the Domo instance.

**Endpoint:** `GET /v1/datasets`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| limit | integer | Maximum datasets to return (default: 50) |
| offset | integer | Offset for pagination |
| sort | string | Sort field |
| ascending | boolean | Sort in ascending order |

#### `domo_get_dataset`
Get metadata for a specific dataset.

**Endpoint:** `GET /v1/datasets/{id}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | string | The dataset ID |

#### `domo_create_dataset`
Create a new dataset with specified schema.

**Endpoint:** `POST /v1/datasets`

**Scopes:** `write`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| name | string | Dataset name |
| description | string | Dataset description |
| schema | object | Column definitions |
| rows | integer | Initial number of rows |

### Dataflows

#### `domo_list_dataflows`
List all dataflows in the Domo instance.

**Endpoint:** `GET /v1/dataflows`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| limit | integer | Maximum dataflows to return (default: 50) |
| offset | integer | Offset for pagination |

#### `domo_get_dataflow`
Get details of a specific dataflow.

**Endpoint:** `GET /v1/dataflows/{id}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | string | The dataflow ID |

### Cards

#### `domo_list_cards`
List all cards (visualizations/dashboards) in Domo.

**Endpoint:** `GET /v1/cards`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| limit | integer | Maximum cards to return (default: 50) |
| offset | integer | Offset for pagination |
| project_id | string | Filter by project ID |

#### `domo_get_card`
Get details of a specific card.

**Endpoint:** `GET /v1/cards/{id}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | string | The card ID |

### Users

#### `domo_list_users`
List all users in the Domo instance.

**Endpoint:** `GET /v1/users`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| limit | integer | Maximum users to return (default: 50) |
| offset | integer | Offset for pagination |
| email | string | Filter by email address |

#### `domo_get_user`
Get details of a specific user.

**Endpoint:** `GET /v1/users/{id}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | string | The user ID |

### Projects

#### `domo_list_projects`
List all projects in the Domo instance.

**Endpoint:** `GET /v1/projects`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| limit | integer | Maximum projects to return (default: 50) |
| offset | integer | Offset for pagination |

## Rate Limits

Domo API rate limits depend on your plan. Standard limits are around 1000 requests per hour for read operations and 100 requests per hour for write operations.

## Common Use Cases

1. **Data Pipeline Management**: Monitor and manage ETL dataflows
2. **Business Reporting**: Access cards and datasets for external dashboards
3. **User Management**: Manage Domo users and permissions programmatically
