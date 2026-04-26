# Akamai Provider Documentation

Akamai is a global CDN and cloud security platform providing content delivery, web security, video delivery, and performance optimization services across its intelligent edge network.

## Overview

The Akamai provider enables programmatic management of Akamai services, including:
- Property configuration and version management
- Activation management for staging and production
- Content Provider (CP) code management
- Edge hostname configuration

## Authentication

Akamai uses Akamai Open API authentication. You can use:
- `AKAMAI_ACCESS_TOKEN` + `AKAMAI_CLIENT_SECRET` + `AKAMAI_CLIENT_TOKEN` + `AKAMAI_HOST`
- Or configure via `.edgerc` file with `default` section

## Base URL

```
https://akumbai-prod-v1.api.edgekey.net
```

## Tools

### Properties

#### `akamai_list_properties`

List all Akamai property configurations in your account.

**Endpoint:** `GET /papi/v1/properties`

**Scopes:** `Property Provisioning Manager`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| contract_id | string | No | Filter by contract ID |
| group_id | string | No | Filter by group ID |
| page | integer | No | Page number for pagination (default: 0) |
| page_size | integer | No | Number of items per page (default: 100) |

#### `akamai_get_property`

Get details for a specific Akamai property by ID.

**Endpoint:** `GET /papi/v1/properties/{property_id}`

**Scopes:** `Property Provisioning Manager`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| property_id | string | Yes | The unique property ID |
| property_version | integer | No | Specific version to retrieve (latest if not specified) |

#### `akamai_create_property`

Create a new Akamai property configuration with a product and contract.

**Endpoint:** `POST /papi/v1/properties`

**Scopes:** `Property Provisioning Manager`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| product_id | string | Yes | The product ID to use |
| contract_id | string | Yes | The contract ID |
| group_id | string | Yes | The group ID |
| property_name | string | Yes | Name for the new property |

### Activations

#### `akamai_list_activations`

List all activation history for a specific Akamai property.

**Endpoint:** `GET /papi/v1/properties/{property_id}/activations`

**Scopes:** `Property Provisioning Manager`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| property_id | string | Yes | The unique property ID |
| page | integer | No | Page number for pagination (default: 0) |
| page_size | integer | No | Number of items per page (default: 100) |

#### `akamai_get_activation`

Get details for a specific activation by ID.

**Endpoint:** `GET /papi/v1/properties/{property_id}/activations/{activation_id}`

**Scopes:** `Property Provisioning Manager`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| property_id | string | Yes | The unique property ID |
| activation_id | string | Yes | The activation ID |

#### `akamai_activate_property`

Activate a specific version of an Akamai property to staging or production network.

**Endpoint:** `POST /papi/v1/properties/{property_id}/activations`

**Scopes:** `Property Provisioning Manager`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| property_id | string | Yes | The unique property ID |
| property_version | integer | Yes | The version number to activate |
| network | string | Yes | Network to activate on (STAGING or PRODUCTION) |
| notes | string | No | Optional notes for the activation |

### CP Codes

#### `akamai_list_cp_codes`

List all Content Provider (CP) codes in your account.

**Endpoint:** `GET /papi/v1/cp-codes`

**Scopes:** `Property Provisioning Manager`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| contract_id | string | No | Filter by contract ID |
| group_id | string | No | Filter by group ID |
| page | integer | No | Page number for pagination (default: 0) |
| page_size | integer | No | Number of items per page (default: 100) |

#### `akamai_get_cp_code`

Get details for a specific CP code by ID.

**Endpoint:** `GET /papi/v1/cp-codes/{cp_code_id}`

**Scopes:** `Property Provisioning Manager`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| cp_code_id | string | Yes | The CP code ID |

### Edge Hostnames

#### `akamai_list_edge_hostnames`

List all edge hostnames in your account.

**Endpoint:** `GET /papi/v1/edge-hostnames`

**Scopes:** `Property Provisioning Manager`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| contract_id | string | No | Filter by contract ID |
| group_id | string | No | Filter by group ID |
| page | integer | No | Page number for pagination (default: 0) |
| page_size | integer | No | Number of items per page (default: 100) |

#### `akamai_get_edge_hostname`

Get details for a specific edge hostname by ID.

**Endpoint:** `GET /papi/v1/edge-hostnames/{edge_hostname_id}`

**Scopes:** `Property Provisioning Manager`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| edge_hostname_id | string | Yes | The edge hostname ID |

## Common Use Cases

### Creating and Activating a Property

1. Create property: `akamai_create_property`
2. Modify property configuration
3. Activate on staging: `akamai_activate_property` with network: STAGING
4. Test and verify
5. Activate on production: `akamai_activate_property` with network: PRODUCTION

### Managing CP Codes

CP codes track usage for billing and reporting:
1. List existing CP codes: `akamai_list_cp_codes`
2. Get details: `akamai_get_cp_code`
3. Use CP code ID when configuring property behaviors

## Rate Limits

Akamai API rate limits are determined by your contract and plan. Contact your account team for specific limits.

## Error Handling

Common error codes:
- `400` - Bad request (invalid parameters)
- `401` - Unauthorized (authentication failed)
- `403` - Forbidden (insufficient permissions)
- `404` - Not found (property/resource not found)
- `422` - Unprocessable entity (validation error)
- `429` - Too many requests (rate limit exceeded)
- `500` - Internal server error