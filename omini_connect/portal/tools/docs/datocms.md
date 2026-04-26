# DatoCMS

DatoCMS is a headless CMS that provides structured content management with powerful APIs, image transformations, and localization support.

## Provider Configuration

- **Auth Type**: API Key (Environment API token)
- **Base URL**: https://site-api.datocms.com
- **Documentation**: https://www.datocms.com/docs

## Available Tools

### Sites

| Tool | Name | Description |
|------|------|-------------|
| `datocms_list_sites` | List Sites | List all DatoCMS sites (accounts) |
| `datocms_get_site` | Get Site | Get detailed site information |

### Items

| Tool | Name | Description |
|------|------|-------------|
| `datocms_list_items` | List Items | List all content items with filtering |
| `datocms_get_item` | Get Item | Get a specific item by ID |
| `datocms_create_item` | Create Item | Create a new content item |
| `datocms_update_item` | Update Item | Update an existing item |

### Models

| Tool | Name | Description |
|------|------|-------------|
| `datocms_list_models` | List Models | List all content models |
| `datocms_get_model` | Get Model | Get a complete model schema definition |

### Assets

| Tool | Name | Description |
|------|------|-------------|
| `datocms_list_assets` | List Assets | List all media assets |
| `datocms_get_asset` | Get Asset | Get detailed asset information |

## API Endpoints

- `GET /sites` - List sites
- `GET /sites/{site_id}` - Get site
- `GET /items` - List items
- `GET /items/{item_id}` - Get item
- `POST /items` - Create item
- `PUT /items/{item_id}` - Update item
- `GET /models` - List models
- `GET /models/{model_id}` - Get model
- `GET /assets` - List assets
- `GET /assets/{asset_id}` - Get asset

## Scopes

- `sites:read` - Read access to sites
- `items:read` - Read access to items
- `items:cud` - Create, update, delete items
- `models:read` - Read access to models
- `assets:read` - Read access to assets

## Notes

- DatoCMS uses Site IDs (environment API key prefixes) for identification
- Items are typed using Content Models with typed fields
- The API supports filtering, searching, and sorting
- Assets include images with automatic transformations (resize, crop, format)