# Elevio Tools

Provider: `elevio` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Elevio API. They allow AI agents to interact with articles, categories, users, and collections in the Elevio knowledge base platform.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `articles:read`, `articles:write`, `categories:read`, `users:read`, `collections:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `elevio_list_articles` | List all articles in the knowledge base | GET | /articles |
| `elevio_get_article` | Get details of a specific article | GET | /articles/{id} |
| `elevio_create_article` | Create a new article | POST | /articles |
| `elevio_list_categories` | List all categories | GET | /categories |
| `elevio_get_category` | Get details of a specific category | GET | /categories/{id} |
| `elevio_list_users` | List all users | GET | /users |
| `elevio_get_user` | Get details of a specific user | GET | /users/{id} |
| `elevio_list_collections` | List all collections | GET | /collections |
| `elevio_get_collection` | Get details of a specific collection | GET | /collections/{id} |
| `elevio_search_articles` | Search articles by keyword | GET | /articles/search |

---

## Tool Details

### elevio_list_articles

**What it does**: Returns a paginated list of articles with their titles, slugs, and publication status.

**When to use**: Browse available knowledge base content, or filter articles by category or publication status.

**Arguments**:
- `published` (optional): Filter by publication status (boolean)
- `category_id` (optional): Filter by category ID
- `per_page` (optional): Results per page (default 20)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "Show me all published articles in the help center"

---

### elevio_get_article

**What it does**: Get details of a specific article including title, body content, author, and metadata.

**When to use**: Read full article content, check authorship, or retrieve article details before updating.

**Arguments**:
- `id` (required): Article ID

**Example LLM prompt**: "Get the article with ID abc123 about getting started"

---

### elevio_create_article

**What it does**: Creates a new article in the knowledge base as a draft by default.

**When to use**: Add new content to the knowledge base programmatically.

**Arguments**:
- `title` (required): Article title
- `body` (optional): Article body content (HTML or Markdown)
- `category_id` (optional): Category ID to assign the article to
- `published` (optional): Whether to publish immediately (default false)

**Example LLM prompt**: "Create a new article titled 'FAQ' with body content 'This is the FAQ page'"

---

### elevio_list_categories

**What it does**: Returns a paginated list of categories that organize articles.

**When to use**: Browse available categories before creating or filtering articles.

**Arguments**:
- `per_page` (optional): Results per page (default 20)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all categories in the knowledge base"

---

### elevio_get_category

**What it does**: Get details of a specific category including name, description, and parent category.

**When to use**: Understand category structure before organizing articles.

**Arguments**:
- `id` (required): Category ID

**Example LLM prompt**: "Get details for the category with ID cat456"

---

### elevio_list_users

**What it does**: Returns a paginated list of users in the Elevio workspace with their roles.

**When to use**: List workspace members, filter by role, or find specific users.

**Arguments**:
- `role` (optional): Filter by user role
- `per_page` (optional): Results per page (default 20)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all admins in the workspace"

---

### elevio_get_user

**What it does**: Get details of a specific user including email, name, role, and activity status.

**When to use**: Get user profile information or verify user details.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get the profile of user user789"

---

### elevio_list_collections

**What it does**: Returns a paginated list of collections that group related articles for curated learning paths.

**When to use**: Browse learning paths or find grouped article sets.

**Arguments**:
- `per_page` (optional): Results per page (default 20)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all collections available in the knowledge base"

---

### elevio_get_collection

**What it does**: Get details of a specific collection including name, description, and article list.

**When to use**: View the contents of a learning path or collection.

**Arguments**:
- `id` (required): Collection ID

**Example LLM prompt**: "Get the articles in collection coll123"

---

### elevio_search_articles

**What it does**: Search articles by keyword or phrase with highlighted search terms.

**When to use**: Find relevant articles when users ask questions about specific topics.

**Arguments**:
- `q` (required): Search query string
- `category_id` (optional): Limit search to a specific category
- `per_page` (optional): Results per page (default 20)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "Search for articles about billing and subscriptions"
