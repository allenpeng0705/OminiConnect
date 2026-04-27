# Fluidtopics Tools

Provider: `fluidtopics` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Fluidtopics is a knowledge management platform that organizes content into topics, categories, and tags. It provides intelligent search and content delivery. These tools wrap the Fluidtopics API, enabling AI agents to access and manage knowledge base content for research, support, and information retrieval.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Fluidtopics
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write` (based on permission level)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fluidtopics_list_topics` | List topics in knowledge base | GET | /api/topics |
| `fluidtopics_get_topic` | Get topic details | GET | /api/topics/{id} |
| `fluidtopics_create_topic` | Create a new topic | POST | /api/topics |
| `fluidtopics_list_categories` | List categories | GET | /api/categories |
| `fluidtopics_get_category` | Get category details | GET | /api/categories/{id} |
| `fluidtopics_list_tags` | List tags | GET | /api/tags |
| `fluidtopics_get_tag` | Get tag details | GET | /api/tags/{id} |
| `fluidtopics_search_topics` | Search topics | POST | /api/searches |
| `fluidtopics_get_topic_content` | Get topic content | GET | /api/topics/{id}/content |
| `fluidtopics_list_feeds` | List feeds | GET | /api/feeds |

---

## Tool Details

### fluidtopics_list_topics

**What it does**: Lists all topics in the knowledge base with optional category and tag filtering.

**When to use**: Browse available knowledge base articles, find topics in a specific category.

**Arguments**:
- `category_id` (optional): Filter by category ID
- `tag_id` (optional): Filter by tag ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all topics in the knowledge base"

---

### fluidtopics_get_topic

**What it does**: Gets metadata information about a specific topic (not the full content).

**When to use**: Get topic summary, see associated categories and tags.

**Arguments**:
- `id` (required): Topic ID

**Example LLM prompt**: "Get details for topic abc123"

---

### fluidtopics_create_topic

**What it does**: Creates a new topic in the knowledge base.

**When to use**: Add new articles, documentation, or knowledge base entries.

**Arguments**:
- `title` (required): Topic title
- `content` (optional): Topic content (HTML or Markdown)
- `category_id` (optional): Category ID to place topic in
- `tag_ids` (optional): Tag IDs to associate

**Example LLM prompt**: "Create a new topic titled 'How to Reset Password' with content explaining the steps"

---

### fluidtopics_list_categories

**What it does**: Lists all categories in the knowledge base.

**When to use**: Browse topic organization, find categories for content grouping.

**Arguments**:
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all categories"

---

### fluidtopics_get_category

**What it does**: Gets detailed information about a specific category.

**When to use**: Understand category structure, see what topics are included.

**Arguments**:
- `id` (required): Category ID

**Example LLM prompt**: "Get details for category xyz789"

---

### fluidtopics_list_tags

**What it does**: Lists all tags used in the knowledge base.

**When to use**: Browse available tags, find topics by tag.

**Arguments**:
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all tags"

---

### fluidtopics_get_tag

**What it does**: Gets detailed information about a specific tag.

**When to use**: Understand tag usage, see topics associated with a tag.

**Arguments**:
- `id` (required): Tag ID

**Example LLM prompt**: "Get details for tag tag456"

---

### fluidtopics_search_topics

**What it does**: Searches topics in the knowledge base using a query string.

**When to use**: Find relevant articles, search for specific information.

**Arguments**:
- `query` (required): Search query string
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "Search for topics about password reset"

---

### fluidtopics_get_topic_content

**What it does**: Gets the full content of a specific topic.

**When to use**: Read full article content, retrieve detailed documentation.

**Arguments**:
- `id` (required): Topic ID

**Example LLM prompt**: "Get the full content of topic abc123"

---

### fluidtopics_list_feeds

**What it does**: Lists all feeds in the Fluidtopics knowledge base.

**When to use**: Browse available feeds, understand content delivery channels.

**Arguments**:
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all available feeds"

---

## Fluidtopics API Notes

- **IDs**: Fluidtopics uses string IDs (often alphanumeric) for all objects
- **Topics**: Individual knowledge base articles or documents
- **Categories**: Hierarchical organization structure for topics
- **Tags**: Metadata labels for cross-referencing topics
- **Searches**: Full-text search endpoint for finding relevant topics
- **Feeds**: Content delivery channels or topic collections
- **Content**: Topics can contain HTML or Markdown formatted content
