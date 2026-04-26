# Perplexity Tools

Provider: `perplexity` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Perplexity API. They allow AI agents to manage threads, messages, user information, models, and collections. Perplexity is an AI-powered search and conversational AI platform. **Requires Perplexity API Key authentication.**

## Authentication

**Nango API Key**:
- Uses Bearer token in Authorization header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.perplexity.ai
- API key pattern: pplx-*

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `perplexity_list_threads` | List threads | GET | /threads |
| `perplexity_get_thread` | Get thread details | GET | /threads/{threadId} |
| `perplexity_create_thread` | Create a thread | POST | /threads |
| `perplexity_post_message` | Post a message | POST | /threads/{threadId}/messages |
| `perplexity_list_messages` | List messages | GET | /threads/{threadId}/messages |
| `perplexity_get_user_info` | Get user info | GET | /user |
| `perplexity_list_models` | List models | GET | /models |
| `perplexity_get_usage` | Get API usage | GET | /usage |
| `perplexity_list_collections` | List collections | GET | /collections |
| `perplexity_get_collection` | Get collection details | GET | /collections/{collectionId} |

---

## Tool Details

### perplexity_list_threads

**What it does**: Lists all conversation threads.

**When to use**: Browse conversation history.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all my threads"

---

### perplexity_get_thread

**What it does**: Gets detailed information about a specific thread.

**When to use**: Get thread details, context.

**Arguments**:
- `threadId` (required): Thread ID

**Example LLM prompt**: "Get details for thread 12345"

---

### perplexity_create_thread

**What it does**: Creates a new conversation thread.

**When to use**: Start a new conversation.

**Arguments**:
- `title` (required): Thread title

**Example LLM prompt**: "Create a new thread called 'Research'"

---

### perplexity_post_message

**What it does**: Posts a message to a thread.

**When to use**: Send a message in a conversation.

**Arguments**:
- `threadId` (required): Thread ID
- `content` (required): Message content
- `model` (optional): Model to use

**Example LLM prompt**: "Post 'What is machine learning?' to thread 12345"

---

### perplexity_list_messages

**What it does**: Lists messages in a thread.

**When to use**: View conversation history.

**Arguments**:
- `threadId` (required): Thread ID

**Example LLM prompt**: "List messages in thread 12345"

---

### perplexity_get_user_info

**What it does**: Gets user account information.

**When to use**: Get account details, subscription info.

**Arguments**: None

**Example LLM prompt**: "Get my account information"

---

### perplexity_list_models

**What it does**: Lists available Perplexity models.

**When to use**: See available AI models.

**Arguments**: None

**Example LLM prompt**: "What models are available?"

---

### perplexity_get_usage

**What it does**: Gets API usage statistics.

**When to use**: Check usage, limits.

**Arguments**: None

**Example LLM prompt**: "What's my API usage?"

---

### perplexity_list_collections

**What it does**: Lists all collections.

**When to use**: Browse knowledge collections.

**Arguments**: None

**Example LLM prompt**: "List my collections"

---

### perplexity_get_collection

**What it does**: Gets detailed information about a specific collection.

**When to use**: Get collection contents, details.

**Arguments**:
- `collectionId` (required): Collection ID

**Example LLM prompt**: "Get details for collection 67890"

---

## Perplexity API Notes

- **API Key**: Uses Bearer token (pplx-*) for authentication
- **Models**: Various models available for different use cases
- **Online Search**: Some models support real-time search
