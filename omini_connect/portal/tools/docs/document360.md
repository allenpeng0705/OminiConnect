# Document360 Tools

Provider: `document360` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Document360 API. They allow AI agents to manage knowledge base articles, categories, tags, and versions. Document360 is a documentation and knowledge management platform.

## Authentication

**Nango API_KEY**:
- User provides their Document360 API key via Nango
- Token stored in Nango, accessed via `connection_ref`
- Passed in headers as `api_token`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `document360_list_articles` | List articles | GET | /v2/articles |
| `document360_get_article` | Get article details | GET | /v2/articles/{article_id} |
| `document360_create_article` | Create an article | POST | /v2/articles |
| `document360_update_article` | Update an article | PUT | /v2/articles/{article_id} |
| `document360_list_categories` | List categories | GET | /v2/categories |
| `document360_get_category` | Get category details | GET | /v2/categories/{category_id} |
| `document360_list_tags` | List tags | GET | /v2/tags |
| `document360_get_tag` | Get tag details | GET | /v2/tags/{tag_id} |
| `document360_list_versions` | List article versions | GET | /v2/articles/{article_id}/versions |
| `document360_search_articles` | Search articles | GET | /v2/articles/search |

---

## Tool Details

### document360_list_articles

**What it does**: Lists all articles in the knowledge base.

**When to use**: Browse documentation, find articles by category, navigate knowledge base.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `category_id` (optional): Filter by category ID

**Example LLM prompt**: "List all articles in category 'Getting Started'"

---

### document360_get_article

**What it does**: Gets detailed article information.

**When to use**: Read article content, check article metadata, review documentation.

**Arguments**:
- `article_id` (required): Article ID

**Example LLM prompt**: "Get details for article a-123"

---

### document360_create_article

**What it does**: Creates a new article in the knowledge base.

**When to use**: Add new documentation, create help articles, publish knowledge base content.

**Arguments**:
- `title` (required): Article title
- `content` (optional): Article content in markdown
- `category_id` (optional): Category ID to assign

**Example LLM prompt**: "Create a new article titled 'How to Reset Password' with content in markdown"

---

### document360_update_article

**What it does**: Updates an existing article.

**When to use**: Edit documentation, update help articles, revise knowledge base content.

**Arguments**:
- `article_id` (required): Article ID
- `title` (optional): Article title
- `content` (optional): Article content in markdown

**Example LLM prompt**: "Update article a-123 with new content"

---

### document360_list_categories

**What it does**: Lists all categories in the knowledge base.

**When to use**: Browse category structure, find article categories, organize content.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all categories"

---

### document360_get_category

**What it does**: Gets detailed category information.

**When to use**: Check category details, view category articles, understand structure.

**Arguments**:
- `category_id` (required): Category ID

**Example LLM prompt**: "Get details for category c-456"

---

### document360_list_tags

**What it does**: Lists all tags used in the knowledge base.

**When to use**: Browse available tags, find related articles, organize content by tags.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all tags"

---

### document360_get_tag

**What it does**: Gets detailed tag information.

**When to use**: Check tag usage, find articles with specific tags.

**Arguments**:
- `tag_id` (required): Tag ID

**Example LLM prompt**: "Get details for tag t-789"

---

### document360_list_versions

**What it does**: Lists all versions of an article.

**When to use**: View article history, restore previous versions, track changes.

**Arguments**:
- `article_id` (required): Article ID
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "List all versions of article a-123"

---

### document360_search_articles

**What it does**: Searches articles by keyword.

**When to use**: Find specific documentation, search help articles, locate information.

**Arguments**:
- `q` (required): Search query
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "Search for articles containing 'authentication'"

---

## Document360 API Notes

- **Knowledge Base**: Documentation and help article platform
- **Articles**: Individual documentation pages with markdown content
- **Categories**: Hierarchical organization for articles
- **Tags**: Labels for cross-referencing articles
- **Versions**: Article revision history
- **Search**: Full-text search across article content
- **API Key**: Passed in headers as `api_token`
