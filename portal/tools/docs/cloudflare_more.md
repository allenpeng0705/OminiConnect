# Cloudflare More Provider Documentation

Cloudflare More provides additional Cloudflare services beyond core CDN, including advanced log analytics, video streaming, image management, and serverless Workers computing.

## Overview

The Cloudflare More provider enables programmatic management of:
- Logpush configurations for advanced analytics
- Cloudflare Stream video platform
- Cloudflare Images storage and delivery
- Cloudflare Workers serverless scripts

## Authentication

Cloudflare uses API token authentication. Create an API token with the appropriate zone and account permissions.

Set `CLOUDFLARE_API_TOKEN` in your environment or include `Authorization: Bearer <token>` in requests.

## Base URL

```
https://api.cloudflare.com/client/v4
```

## Tools

### Logs

#### `cloudflare_more_list_logpull_configs`

List all logpush configurations for a Cloudflare zone.

**Endpoint:** `GET /zones/{zone_id}/logspush/ownership`

**Scopes:** `Zone Settings`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| zone_id | string | Yes | The zone ID |

#### `cloudflare_more_get_logpull_config`

Get details for a specific logpush configuration.

**Endpoint:** `GET /zones/{zone_id}/logspush/ownership/{job_id}`

**Scopes:** `Zone Settings`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| zone_id | string | Yes | The zone ID |
| job_id | integer | Yes | The logpush job ID |

#### `cloudflare_more_update_logpull_config`

Update a logpush configuration to change destination or filters.

**Endpoint:** `PUT /zones/{zone_id}/logspush/ownership/{job_id}`

**Scopes:** `Zone Settings`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| zone_id | string | Yes | The zone ID |
| job_id | integer | Yes | The logpush job ID |
| enabled | boolean | No | Enable or disable the logpush job |
| destination_conf | string | No | Destination configuration (splunk, s3, etc.) |
| filter | string | No | Log filter expression |

### Videos (Cloudflare Stream)

#### `cloudflare_more_list_stream_videos`

List all videos in your Cloudflare Stream library.

**Endpoint:** `GET /accounts/{account_id}/stream`

**Scopes:** `Cloudflare Stream`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| account_id | string | Yes | The account ID |
| cursor | string | No | Pagination cursor for next page |
| per_page | integer | No | Number of items per page (default: 25) |

#### `cloudflare_more_get_stream_video`

Get details and metadata for a specific Cloudflare Stream video.

**Endpoint:** `GET /accounts/{account_id}/stream/{video_id}`

**Scopes:** `Cloudflare Stream`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| account_id | string | Yes | The account ID |
| video_id | string | Yes | The video ID |

#### `cloudflare_more_upload_stream_video`

Upload a new video to Cloudflare Stream.

**Endpoint:** `POST /accounts/{account_id}/stream`

**Scopes:** `Cloudflare Stream`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| account_id | string | Yes | The account ID |
| video_url | string | No | URL of the video to upload |
| caption_url | string | No | URL of VTT caption file |
| meta | object | No | Metadata key-value pairs |
| require_signed_urls | boolean | No | Require signed URLs for playback |

### Images (Cloudflare Images)

#### `cloudflare_more_list_images`

List all images in your Cloudflare Images storage.

**Endpoint:** `GET /accounts/{account_id}/images/v1`

**Scopes:** `Cloudflare Images`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| account_id | string | Yes | The account ID |
| page | integer | No | Page number (default: 1) |
| per_page | integer | No | Items per page (default: 25, max: 100) |

#### `cloudflare_more_get_image`

Get metadata for a specific Cloudflare Images image.

**Endpoint:** `GET /accounts/{account_id}/images/v1/{image_id}`

**Scopes:** `Cloudflare Images`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| account_id | string | Yes | The account ID |
| image_id | string | Yes | The image ID |

#### `cloudflare_more_upload_image`

Upload a new image to Cloudflare Images.

**Endpoint:** `POST /accounts/{account_id}/images/v1`

**Scopes:** `Cloudflare Images`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| account_id | string | Yes | The account ID |
| file | string | No | Base64 encoded image data or URL to image file |
| name | string | No | Image identifier name |
| require_signed_urls | boolean | No | Require signed URLs for access |
| metadata | object | No | Custom metadata key-value pairs |

### Workers

#### `cloudflare_more_list_workers`

List all Workers scripts deployed in your account.

**Endpoint:** `GET /accounts/{account_id}/workers/scripts`

**Scopes:** `Cloudflare Workers`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| account_id | string | Yes | The account ID |
| page | integer | No | Page number (default: 1) |
| per_page | integer | No | Items per page (default: 25) |

## Common Use Cases

### Setting Up Logpush

Send Cloudflare logs to your SIEM or log aggregator:

1. List existing configs: `cloudflare_more_list_logpull_configs`
2. Create new logpush destination (Splunk, S3, etc.)
3. Update config as needed: `cloudflare_more_update_logpull_config`

### Managing Stream Videos

1. List videos: `cloudflare_more_list_stream_videos`
2. Get video details: `cloudflare_more_get_stream_video`
3. Upload new video: `cloudflare_more_upload_stream_video`

### Image Management

1. List images: `cloudflare_more_list_images`
2. Get image metadata: `cloudflare_more_get_image`
3. Upload images: `cloudflare_more_upload_image`

## Rate Limits

Cloudflare API rate limits vary by endpoint and plan:
- General API: 1200 requests per minute
- Stream: Specific limits based on plan
- Workers: 1000 requests per second per script

## Error Handling

Common error codes:
- `400` - Bad request (invalid parameters)
- `401` - Unauthorized (invalid API token)
- `403` - Forbidden (insufficient permissions)
- `404` - Not found (resource not found)
- `429` - Too many requests (rate limit exceeded)
- `10000` - Zone not found