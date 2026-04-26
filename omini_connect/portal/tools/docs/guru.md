# Guru Integration

Guru is a knowledge management platform that helps teams capture, organize, and share information through collections and cards.

## Authentication

Guru uses OAuth 2.0 or API key authentication. API keys can be generated in Guru Settings > Integrations > API.

## Base URL

```
https://api.getguru.com/api/v1
```

## Rate Limits

- Default rate limit: 100 requests per minute
- Search rate limit: 30 requests per minute

## Available Tools

### Collections

| Tool | Description |
|------|-------------|
| `guru_list_collections` | Retrieve a list of all collections |
| `guru_get_collection` | Get details of a specific collection |

### Cards

| Tool | Description |
|------|-------------|
| `guru_list_cards` | Retrieve a list of all cards |
| `guru_get_card` | Get details of a specific card |
| `guru_create_card` | Create a new card in a collection |
| `guru_share_card` | Share a card with users or groups |

### Users

| Tool | Description |
|------|-------------|
| `guru_list_users` | Retrieve a list of all users |
| `guru_get_user` | Get details of a specific user |

### Search

| Tool | Description |
|------|-------------|
| `guru_search_cards` | Search for cards by keyword |
| `guru_get_suggestions` | Get suggested cards based on context |

## Card Types

| Type | Description |
|------|-------------|
| text | Text-based card with answer content |
| url | Card that links to an external URL |
| file | Card with an attached file |

## Common Use Cases

- **Knowledge Base Management**: Create and organize company knowledge
- **Team Collaboration**: Share information across teams
- **Card Creation**: Add new content to collections
- **Information Retrieval**: Search for specific answers or topics
- **Content Sharing**: Distribute cards to specific users or groups

## Notes

- Cards must belong to a collection
- Search is performed across card titles and content
- Suggestions are context-aware based on query
- Sharing requires user or group IDs
