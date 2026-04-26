# Researchdesk Tools

Provider: `researchdesk` | Engine: `nango` | Auth: OAuth via Nango

## Overview

ResearchDesk is a market research and article management platform. These tools allow AI agents to manage research articles, organize them into collections, search through saved content, and discover trending topics.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with ResearchDesk
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `articles:read`, `collections:read`, `collections:write`, `tags:read`, `tags:write`, `trending:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `researchdesk_list_articles` | List research articles | GET | /articles |
| `researchdesk_get_article` | Get article details | GET | /articles/{articleId} |
| `researchdesk_search_articles` | Search articles | GET | /articles/search |
| `researchdesk_list_collections` | List collections | GET | /collections |
| `researchdesk_create_collection` | Create a collection | POST | /collections |
| `researchdesk_add_to_collection` | Add article to collection | POST | /collections/{collectionId}/articles |
| `researchdesk_list_tags` | List all tags | GET | /tags |
| `researchdesk_create_tag` | Create a tag | POST | /tags |
| `researchdesk_get_trending` | Get trending topics | GET | /trending |
| `researchdesk_export_research` | Export research data | GET | /export |

---

## Tool Details

### researchdesk_list_articles

**What it does**: Returns a list of all saved articles.

**When to use**: Browse your research library.

**Arguments**:
- `limit` (optional): Number of articles (default 50)
- `status` (optional): Filter by status (unread, read, archived)

**Example LLM prompt**: "List all unread articles"

---

### researchdesk_get_article

**What it does**: Gets details of a specific article.

**When to use**: Read article summary and metadata.

**Arguments**:
- `articleId` (required): The article ID

**Example LLM prompt**: "Get details for article art_abc123"

---

### researchdesk_search_articles

**What it does**: Searches through your saved articles.

**When to use**: Find specific research content.

**Arguments**:
- `query` (required): Search query
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "Search for articles about AI trends"

---

### researchdesk_list_collections

**What it does**: Returns a list of all collections.

**When to use**: View your research organization.

**Arguments**:
- `limit` (optional): Number of collections (default 50)

**Example LLM prompt**: "List all my collections"

---

### researchdesk_create_collection

**What it does**: Creates a new collection.

**When to use**: Organize research by topic or project.

**Arguments**:
- `name` (required): Collection name
- `description` (optional): Description

**Example LLM prompt**: "Create a collection called 'AI Research 2024'"

---

### researchdesk_add_to_collection

**What it does**: Adds an article to a collection.

**When to use**: Organize articles into topics.

**Arguments**:
- `collectionId` (required): Collection ID
- `articleId` (required): Article ID

**Example LLM prompt**: "Add article art_123 to collection col_456"

---

### researchdesk_list_tags

**What it does**: Returns a list of all tags.

**When to use**: View available tags for categorization.

**Arguments**:
- `limit` (optional): Number of tags (default 50)

**Example LLM prompt**: "List all tags"

---

### researchdesk_create_tag

**What it does**: Creates a new tag.

**When to use**: Create tags for article categorization.

**Arguments**:
- `name` (required): Tag name
- `color` (optional): Tag color

**Example LLM prompt**: "Create a tag called 'Important'"

---

### researchdesk_get_trending

**What it does**: Returns trending topics and articles.

**When to use**: Discover popular research topics.

**Arguments**:
- `limit` (optional): Number of topics (default 10)

**Example LLM prompt**: "What are the trending topics?"

---

### researchdesk_export_research

**What it does**: Exports your research data.

**When to use**: Backup or share your research.

**Arguments**:
- `format` (optional): Export format (json, csv, markdown)
- `collectionId` (optional): Export specific collection

**Example LLM prompt**: "Export all research as JSON"

---

## ResearchDesk Notes

- Collections organize research by topic or project
- Tags provide additional categorization
- Search finds relevant articles across your library
- Trending shows popular topics in your field
