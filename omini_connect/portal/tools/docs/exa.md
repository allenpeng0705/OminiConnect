# Exa Tools

Provider: `exa` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Exa API. They allow AI agents to search the web, find similar content, and manage document collections. Exa is a web search engine optimized for AI applications.

## Authentication

**Nango API_KEY**:
- User provides their Exa API key via Nango Connect
- Key is passed in the x-api-key header
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `exa_search` | Search the web | POST | /search |
| `exa_search_contents` | Search and get contents | POST | /search |
| `exa_find_similar` | Find similar documents | POST | /find-similar |
| `exa_get_document` | Get document by ID | GET | /documents/{id} |
| `exa_list_collections` | List collections | GET | /collections |
| `exa_create_collection` | Create a collection | POST | /collections |
| `exa_delete_document` | Delete a document | DELETE | /documents/{id} |
| `exa_get_stats` | Get usage statistics | GET | /stats |
| `exa_list_webpages` | List discovered webpages | GET | /webpages |
| `exa_highlight` | Get highlights from search results | POST | /highlight |

---

## Tool Details

### exa_search

**What it does**: Searches the web using natural language queries.

**When to use**: Find current information, research topics, get web results.

**Arguments**:
- `query` (required): Natural language search query
- `num_results` (optional): Number of results (default 10)
- `start_date` (optional): Start date filter (YYYY-MM-DD)
- `end_date` (optional): End date filter (YYYY-MM-DD)

**Example LLM prompt**: "Search for recent news about AI"

---

### exa_search_contents

**What it does**: Searches and retrieves the full contents of matching pages.

**When to use**: Get detailed content from search results, deep research.

**Arguments**:
- `query` (required): Natural language search query
- `num_results` (optional): Number of results (default 10)
- `contents` (optional): Include page contents (default true)
- `start_date` (optional): Start date filter
- `end_date` (optional): End date filter

**Example LLM prompt**: "Search for the latest AI research papers with full content"

---

### exa_find_similar

**What it does**: Finds documents similar to a given URL or text.

**When to use**: Discover related content, expand research.

**Arguments**:
- `query` (required): URL or text to find similar content
- `num_results` (optional): Number of results (default 10)

**Example LLM prompt**: "Find articles similar to https://example.com/article"

---

### exa_get_document

**What it does**: Gets a specific document by its ID.

**When to use**: Retrieve saved document content.

**Arguments**:
- `id` (required): Document ID

**Example LLM prompt**: "Get document abc123"

---

### exa_list_collections

**What it does**: Lists all document collections.

**When to use**: Browse your document libraries.

**Arguments**: None

**Example LLM prompt**: "List my collections"

---

### exa_create_collection

**What it does**: Creates a new collection for storing documents.

**When to use**: Organize documents by topic or project.

**Arguments**:
- `name` (required): Collection name
- `description` (optional): Collection description

**Example LLM prompt**: "Create a collection called 'AI Research'"

---

### exa_delete_document

**What it does**: Deletes a document from a collection.

**When to use**: Remove unwanted or outdated documents.

**Arguments**:
- `id` (required): Document ID

**Example LLM prompt**: "Delete document abc123"

---

### exa_get_stats

**What it does**: Gets API usage statistics.

**When to use**: Monitor usage, check limits.

**Arguments**: None

**Example LLM prompt**: "Get my API usage stats"

---

### exa_list_webpages

**What it does**: Lists discovered webpages from previous searches.

**When to use**: Access cached search results.

**Arguments**:
- `num_results` (optional): Number of results (default 10)

**Example LLM prompt**: "List my recent webpages"

---

### exa_highlight

**What it does**: Gets highlighted text passages from search results.

**When to use**: Extract relevant snippets from results.

**Arguments**:
- `query` (required): Query to highlight
- `ids` (required): Array of result IDs

**Example LLM prompt**: "Get highlights for results abc123 and def456"

---

## Exa API Notes

- **Natural language search**: Query using plain English, not keywords
- **Date filters**: Filter results by time period
- **Collections**: Organize and store documents for later access
- **Similarity search**: Find related content by URL or text
- **Highlights**: Extract relevant text passages from results
