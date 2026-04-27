# Prismic

Prismic is a headless CMS that provides a customizable content management system with slicing for building dynamic page layouts.

## Provider Configuration

- **Auth Type**: OAuth 2.0 / API Key
- **Base URL**: https://prismic.io
- **Documentation**: https://prismic.io/docs

## Available Tools

### Repositories

| Tool | Name | Description |
|------|------|-------------|
| `prismic_list_repositories` | List Repositories | List all Prismic repositories accessible to the authenticated user |
| `prismic_get_repository` | Get Repository | Get details of a specific Prismic repository |

### Documents

| Tool | Name | Description |
|------|------|-------------|
| `prismic_list_documents` | List Documents | List all documents in a repository with filtering options |
| `prismic_get_document` | Get Document | Get a specific document by its unique ID |
| `prismic_create_document` | Create Document | Create a new document in a repository |
| `prismic_update_document` | Update Document | Update an existing document |

### Types

| Tool | Name | Description |
|------|------|-------------|
| `prismic_list_types` | List Types | List all custom types defined in a repository |
| `prismic_get_type` | Get Type | Get the complete schema definition of a custom type |

### Tags

| Tool | Name | Description |
|------|------|-------------|
| `prismic_list_tags` | List Tags | List all tags defined in a repository |
| `prismic_get_tag` | Get Tag | Get details of a specific tag and its usage count |

## API Endpoints

- `GET /v2/repositories` - List repositories
- `GET /v2/repositories/{repository}` - Get repository
- `GET /v2/documents/search` - Search documents
- `GET /v2/documents/{document_id}` - Get document
- `POST /v2/documents` - Create document
- `PUT /v2/documents/{document_id}` - Update document
- `GET /v2/types` - List custom types
- `GET /v2/types/{type_id}` - Get custom type
- `GET /v2/tags` - List tags
- `GET /v2/tags/{tag}` - Get tag

## Scopes

- `masters` - Full access to repository content

## Notes

- Prismic uses a "repository" concept that maps to a project/workspace
- Documents are typed using Custom Types with structured fields
- Tags provide flexible categorization beyond type hierarchy
- The API supports filtering by type, tags, and full-text search