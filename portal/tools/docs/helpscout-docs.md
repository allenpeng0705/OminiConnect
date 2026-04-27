# Help Scout Docs Tools

Provider: `helpscout-docs` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Help Scout Docs API. They allow AI agents to manage knowledge base articles, collections, sites, and categories. Help Scout Docs is a help center and documentation platform.

## Authentication

**Nango Basic Auth**:
- User provides API key via Nango Connect
- Uses API key as username with 'X' as password
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://docsapi.helpscout.net

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `helpscout_docs_list_articles` | List articles | GET | /articles |
| `helpscout_docs_get_article` | Get article details | GET | /articles/{id} |
| `helpscout_docs_list_collections` | List collections | GET | /collections |
| `helpscout_docs_get_collection` | Get collection details | GET | /collections/{id} |
| `helpscout_docs_list_sites` | List sites | GET | /sites |
| `helpscout_docs_get_site` | Get site details | GET | /sites/{id} |
| `helpscout_docs_list_categories` | List categories | GET | /categories |
| `helpscout_docs_get_category` | Get category details | GET | /categories/{id} |
| `helpscout_docs_search_articles` | Search articles | GET | /articles/search |
| `helpscout_docs_list_users` | List users | GET | /users |

---

## Tool Details

### helpscout_docs_list_articles

**What it does**: Lists all articles in Help Scout Docs.

**When to use**: Browse knowledge base articles.

**Arguments**:
- `site_id` (optional): Filter by site ID
- `collection_id` (optional): Filter by collection ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all articles in site 1"

---

### helpscout_docs_get_article

**What it does**: Gets detailed information about a specific article.

**When to use**: View article content and metadata.

**Arguments**:
- `id` (required): Article ID

**Example LLM prompt**: "Get article with ID 123"

---

### helpscout_docs_list_collections

**What it does**: Lists all collections in Help Scout Docs.

**When to use**: Browse article collections.

**Arguments**:
- `site_id` (optional): Filter by site ID

**Example LLM prompt**: "List all collections"

---

### helpscout_docs_get_collection

**What it does**: Gets detailed information about a specific collection.

**When to use**: View collection and its articles.

**Arguments**:
- `id` (required): Collection ID

**Example LLM prompt**: "Get collection with ID 456"

---

### helpscout_docs_list_sites

**What it does**: Lists all sites in Help Scout Docs.

**When to use**: Browse help center sites.

**Arguments**: None

**Example LLM prompt**: "List all sites"

---

### helpscout_docs_get_site

**What it does**: Gets detailed information about a specific site.

**When to use**: View site settings and collections.

**Arguments**:
- `id` (required): Site ID

**Example LLM prompt**: "Get site with ID 1"

---

### helpscout_docs_list_categories

**What it does**: Lists all categories in Help Scout Docs.

**When to use**: Browse article categories.

**Arguments**:
- `collection_id` (optional): Filter by collection ID

**Example LLM prompt**: "List all categories"

---

### helpscout_docs_get_category

**What it does**: Gets detailed information about a specific category.

**When to use**: View category and its articles.

**Arguments**:
- `id` (required): Category ID

**Example LLM prompt**: "Get category with ID 789"

---

### helpscout_docs_search_articles

**What it does**: Searches for articles matching a query.

**When to use**: Find articles by keyword.

**Arguments**:
- `query` (required): Search query

**Example LLM prompt**: "Search for articles about billing"

---

### helpscout_docs_list_users

**What it does**: Lists all users in Help Scout Docs.

**When to use**: View help center authors.

**Arguments**: None

**Example LLM prompt**: "List all users"

---

## Help Scout Docs API Notes

- **API Base URL**: https://docsapi.helpscout.net
- **Auth Mode**: Basic Auth with API key as username
- **Sites**: Top-level help center sites
- **Collections**: Groups of related articles
- **Categories**: Sub-groups within collections
- **Articles**: Individual knowledge base articles
- **Users**: Article authors and editors
