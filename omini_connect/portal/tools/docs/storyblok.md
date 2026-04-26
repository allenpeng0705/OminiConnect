# Storyblok

Storyblok is a headless CMS with a visual editor and component-based content structure. It supports stories (content entries), assets, and collaborative workflows.

## Provider Configuration

- **Auth Type**: OAuth 2.0
- **Base URL**: https://api.storyblok.com
- **Documentation**: https://storyblok.com/docs

## Available Tools

### Stories

| Tool | Name | Description |
|------|------|-------------|
| `storyblok_list_stories` | List Stories | List all stories in a space with filtering |
| `storyblok_get_story` | Get Story | Get a specific story by ID or slug |
| `storyblok_create_story` | Create Story | Create a new story |
| `storyblok_update_story` | Update Story | Update an existing story |
| `storyblok_delete_story` | Delete Story | Delete a story permanently |

### Spaces

| Tool | Name | Description |
|------|------|-------------|
| `storyblok_list_spaces` | List Spaces | List all Storyblok spaces |
| `storyblok_get_space` | Get Space | Get details of a specific space |

### Assets

| Tool | Name | Description |
|------|------|-------------|
| `storyblok_list_assets` | List Assets | List all assets in a space |
| `storyblok_get_asset` | Get Asset | Get details of a specific asset |

### Users

| Tool | Name | Description |
|------|------|-------------|
| `storyblok_list_users` | List Users | List all users in a space |

## API Endpoints

- `GET /v2/stories` - List stories
- `GET /v2/stories/{story_id}` - Get story
- `POST /v2/stories` - Create story
- `PUT /v2/stories/{story_id}` - Update story
- `DELETE /v2/stories/{story_id}` - Delete story
- `GET /v2/spaces` - List spaces
- `GET /v2/spaces/{space_id}` - Get space
- `GET /v2/assets` - List assets
- `GET /v2/assets/{asset_id}` - Get asset
- `GET /v2/spaces/{space_id}/users` - List users

## Scopes

- `stories:read` - Read access to stories
- `stories:write` - Write access to stories
- `spaces:read` - Read access to spaces
- `assets:read` - Read access to assets

## Notes

- Storyblok organizes content into Spaces (multi-tenant)
- Stories can be nested in folders for organization
- Each story has a slug and multiple versions (draft/published)
- Assets are media files (images, videos, documents) stored in a CDN