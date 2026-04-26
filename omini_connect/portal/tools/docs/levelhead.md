# Levelhead Provider Documentation

Levelhead is an analytics platform that provides campaign management, audience segmentation, reporting, and data analytics capabilities for marketing and product analytics teams.

## Overview

The Levelhead provider enables programmatic management of:
- Analytics campaigns with date-based tracking
- Audience segments for targeted analysis
- Report generation and management
- Data segments for flexible analytics queries

## Authentication

Levelhead uses API key authentication. Include your Levelhead API key in the `X-API-Key` header.

Set `LEVELHEAD_API_KEY` in your environment or pass it via the authentication header.

## Base URL

```
https://api.levelhead.io
```

## Tools

### Campaigns

#### `levelhead_list_campaigns`

List all analytics campaigns in your Levelhead account.

**Endpoint:** `GET /api/v1/campaigns`

**Scopes:** `campaigns:read`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default: 1) |
| per_page | integer | No | Number of items per page (default: 20) |
| status | string | No | Filter by status (active, paused, completed) |
| start_date | string | No | Filter by start date (ISO 8601 format) |
| end_date | string | No | Filter by end date (ISO 8601 format) |

#### `levelhead_get_campaign`

Get detailed information for a specific analytics campaign.

**Endpoint:** `GET /api/v1/campaigns/{campaign_id}`

**Scopes:** `campaigns:read`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| campaign_id | string | Yes | The unique campaign ID |

#### `levelhead_create_campaign`

Create a new analytics campaign with specified parameters.

**Endpoint:** `POST /api/v1/campaigns`

**Scopes:** `campaigns:write`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| name | string | Yes | Name for the campaign |
| description | string | No | Optional description |
| start_date | string | No | Start date (ISO 8601 format) |
| end_date | string | No | End date (ISO 8601 format) |
| status | string | No | Initial status (active, paused) |

### Audiences

#### `levelhead_list_audiences`

List all audience segments defined in your Levelhead account.

**Endpoint:** `GET /api/v1/audiences`

**Scopes:** `audiences:read`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default: 1) |
| per_page | integer | No | Number of items per page (default: 20) |
| type | string | No | Filter by audience type |

#### `levelhead_get_audience`

Get detailed information for a specific audience segment.

**Endpoint:** `GET /api/v1/audiences/{audience_id}`

**Scopes:** `audiences:read`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| audience_id | string | Yes | The unique audience ID |

#### `levelhead_create_audience`

Create a new audience segment with defined criteria.

**Endpoint:** `POST /api/v1/audiences`

**Scopes:** `audiences:write`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| name | string | Yes | Name for the audience |
| description | string | No | Optional description |
| criteria | object | Yes | Audience definition criteria |
| source | string | No | Data source for the audience |

### Reports

#### `levelhead_list_reports`

List all analytics reports generated in your Levelhead account.

**Endpoint:** `GET /api/v1/reports`

**Scopes:** `reports:read`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default: 1) |
| per_page | integer | No | Number of items per page (default: 20) |
| campaign_id | string | No | Filter by campaign ID |
| format | string | No | Filter by report format (pdf, csv, xlsx) |

#### `levelhead_get_report`

Get details and download URL for a specific analytics report.

**Endpoint:** `GET /api/v1/reports/{report_id}`

**Scopes:** `reports:read`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| report_id | string | Yes | The unique report ID |

### Segments

#### `levelhead_list_segments`

List all data segments available for analytics queries.

**Endpoint:** `GET /api/v1/segments`

**Scopes:** `segments:read`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default: 1) |
| per_page | integer | No | Number of items per page (default: 20) |
| category | string | No | Filter by segment category |

#### `levelhead_get_segment`

Get detailed information for a specific data segment.

**Endpoint:** `GET /api/v1/segments/{segment_id}`

**Scopes:** `segments:read`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| segment_id | string | Yes | The unique segment ID |

## Common Use Cases

### Campaign Analytics Workflow

1. List active campaigns: `levelhead_list_campaigns`
2. Get campaign details: `levelhead_get_campaign`
3. Create audience segments for targeting: `levelhead_create_audience`
4. Generate reports: Use reporting features
5. Analyze with segments: `levelhead_list_segments`

### Audience-Based Analysis

Define audiences based on criteria, then:
1. List audiences: `levelhead_list_audiences`
2. Get audience details: `levelhead_get_audience`
3. Create targeted campaigns based on audience

### Report Generation

1. List available reports: `levelhead_list_reports`
2. Filter by campaign or format
3. Get report details with download URL: `levelhead_get_report`

## Rate Limits

Levelhead API rate limits vary by plan:
- Free tier: 100 requests per hour
- Pro tier: 1000 requests per hour
- Enterprise: Custom limits

## Error Handling

Common error codes:
- `400` - Bad request (invalid parameters)
- `401` - Unauthorized (invalid API key)
- `403` - Forbidden (insufficient permissions)
- `404` - Not found (resource not found)
- `422` - Unprocessable entity (validation error)
- `429` - Too many requests (rate limit exceeded)
- `500` - Internal server error