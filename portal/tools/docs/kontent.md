# Kontent.ai

Kontent.ai is a headless CMS that provides structured content management with strong workflow support, localization, and a GraphQL API.

## Provider Configuration

- **Auth Type**: API Key (Management API key)
- **Base URL**: https://manage.kontent.ai
- **Documentation**: https://kontent.ai/learn/docs

## Available Tools

### Projects

| Tool | Name | Description |
|------|------|-------------|
| `kontent_list_projects` | List Projects | List all Kontent.ai projects |
| `kontent_get_project` | Get Project | Get detailed project information |

### Items

| Tool | Name | Description |
|------|------|-------------|
| `kontent_list_items` | List Items | List all content items with filtering |
| `kontent_get_item` | Get Item | Get a specific item by ID |
| `kontent_create_item` | Create Item | Create a new content item |
| `kontent_update_item` | Update Item | Update an existing item |

### Types

| Tool | Name | Description |
|------|------|-------------|
| `kontent_list_types` | List Types | List all content types |
| `kontent_get_type` | Get Type | Get a complete type schema definition |

### Assets

| Tool | Name | Description |
|------|------|-------------|
| `kontent_list_assets` | List Assets | List all digital assets |
| `kontent_get_asset` | Get Asset | Get detailed asset information |

## API Endpoints

- `GET /v2/projects` - List projects
- `GET /v2/projects/{project_id}` - Get project
- `GET /v2/items` - List items
- `GET /v2/items/{item_id}` - Get item
- `POST /v2/items` - Create item
- `PUT /v2/items/{item_id}` - Update item
- `GET /v2/types` - List types
- `GET /v2/types/{type_id}` - Get type
- `GET /v2/assets` - List assets
- `GET /v2/assets/{asset_id}` - Get asset

## Scopes

- `projects:read` - Read access to projects
- `items:read` - Read access to items
- `items:cud` - Create, update, delete items
- `types:read` - Read access to types
- `assets:read` - Read access to assets

## Notes

- Kontent.ai organizes content into Projects (each with its own environment)
- Items have system metadata (ID, name, type) and elements (typed content fields)
- Content types define the structure of items with specific element types
- Workflows support publishing/unpublishing with status tracking
- Assets support multiple renditions for responsive delivery