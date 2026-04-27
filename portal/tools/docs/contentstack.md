# Contentstack Integration

## Overview

Contentstack is a headless CMS (Content Management System) that provides APIs for managing content, assets, content types, and locales. This integration enables AI agents to interact with the Contentstack Content Management API.

## Authentication

Contentstack uses Management API tokens for authentication. Configure the following credentials:

- **API Key**: Your Contentstack stack API key
- **Management Token**: A dedicated management token for API access
- **Environment**: The environment name (e.g., `production`, `staging`)

## Available Tools

### Assets

| Tool | Description |
|------|-------------|
| `contentstack_list_assets` | List all assets in the stack with optional filtering |
| `contentstack_get_asset` | Get details of a specific asset by UID |
| `contentstack_upload_asset` | Upload a new asset to the stack |

### Entries

| Tool | Description |
|------|-------------|
| `contentstack_list_entries` | List entries for a specific content type |
| `contentstack_get_entry` | Get details of a specific entry by UID |
| `contentstack_create_entry` | Create a new entry in a content type |

### Content Types

| Tool | Description |
|------|-------------|
| `contentstack_list_content_types` | List all content types in the stack |
| `contentstack_get_content_type` | Get details of a specific content type schema |

### Locales

| Tool | Description |
|------|-------------|
| `contentstack_list_locales` | List all locales configured in the stack |
| `contentstack_get_locale` | Get details of a specific locale |

## API Base URL

```
https://api.contentstack.io/v3
```

## Common Use Cases

### Listing Assets

```json
{
  "limit": 20,
  "skip": 0,
  "include_dimensions": true,
  "include_fallback": false
}
```

### Listing Entries

```json
{
  "content_type_uid": "blog_post",
  "limit": 20,
  "skip": 0,
  "locale": "en-us",
  "query": {
    "author": "john-doe"
  }
}
```

### Creating an Entry

```json
{
  "content_type_uid": "blog_post",
  "entry": {
    "title": "My New Blog Post",
    "url": "/my-new-blog-post",
    "body": "This is the content of my new blog post."
  },
  "locale": "en-us"
}
```

## Scopes

The following OAuth scopes are used by these tools:

- `assets:read` - Read asset data
- `assets:create` - Upload and create assets
- `content_type.read` - Read content type schemas
- `entry:read` - Read entry content
- `entry:create` - Create new entries
- `settings:read` - Read locale settings

## Rate Limits

Contentstack implements rate limiting on API requests. The integration handles pagination automatically to stay within rate limits.

## Error Handling

Common error responses:

- `400 Bad Request` - Invalid request parameters or malformed JSON
- `401 Unauthorized` - Invalid or missing API key / management token
- `403 Forbidden` - Insufficient permissions for the requested operation
- `404 Not Found` - The requested resource does not exist
- `429 Too Many Requests` - Rate limit exceeded

## Resources

- [Contentstack API Documentation](https://www.contentstack.com/docs/developers/)
- [Contentstack Management API](https://www.contentstack.com/docs/developers/apis/content-management-api/)
- [Contentstack Developer Documentation](https://www.contentstack.com/docs/)
