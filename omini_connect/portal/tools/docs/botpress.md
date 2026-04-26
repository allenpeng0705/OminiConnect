# Botpress Tools

Provider: `botpress` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

Botpress is a conversational AI platform that enables businesses to build and deploy AI-powered chatbots. These tools allow AI agents to manage bots, flows, intents, and entities.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `bots`, `flows`, `intents`, `entities`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `botpress_list_bots` | List all bots | GET | /bots |
| `botpress_get_bot` | Get a specific bot | GET | /bots/{id} |
| `botpress_create_bot` | Create a new bot | POST | /bots |
| `botpress_list_flows` | List all flows | GET | /bots/{bot_id}/flows |
| `botpress_get_flow` | Get a specific flow | GET | /bots/{bot_id}/flows/{flow_id} |
| `botpress_create_flow` | Create a new flow | POST | /bots/{bot_id}/flows |
| `botpress_list_intents` | List all intents | GET | /bots/{bot_id}/intents |
| `botpress_get_intent` | Get a specific intent | GET | /bots/{bot_id}/intents/{intent_id} |
| `botpress_list_entities` | List all entities | GET | /bots/{bot_id}/entities |
| `botpress_get_entity` | Get a specific entity | GET | /bots/{bot_id}/entities/{entity_id} |

---

## Tool Details

### botpress_list_bots

**What it does**: Retrieve all bots in your Botpress workspace. Bots are conversational AI applications.

**When to use**: Browse available bots before creating flows, intents, or entities.

**Arguments**:
- `status` (optional): Filter by bot status (active, archived)
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page

**Example LLM prompt**: "List all bots in my Botpress workspace"

---

### botpress_get_bot

**What it does**: Get details of a specific bot. Returns configuration, status, and metadata.

**When to use**: Review bot configuration and status before making changes.

**Arguments**:
- `id` (required): Unique identifier of the bot

**Example LLM prompt**: "Show me details for bot bot_123"

---

### botpress_create_bot

**What it does**: Create a new bot in your Botpress workspace. Bots are conversational AI applications that can be configured with flows, intents, and entities.

**When to use**: Set up a new conversational AI application.

**Arguments**:
- `name` (required): Name of the bot
- `description` (optional): Description of the bot
- `category` (optional): Category for the bot
- `language` (optional): Primary language code

**Example LLM prompt**: "Create a new bot called 'Customer Support Bot' in English"

---

### botpress_list_flows

**What it does**: Retrieve all flows in a Botpress bot. Flows define the conversation paths and logic.

**When to use**: Browse available flows to understand conversation structure.

**Arguments**:
- `bot_id` (required): ID of the bot
- `status` (optional): Filter by flow status

**Example LLM prompt**: "List all flows in bot bot_123"

---

### botpress_get_flow

**What it does**: Get details of a specific flow including nodes and transitions. Use this to understand the conversation logic.

**When to use**: Review flow structure before modifying or triggering.

**Arguments**:
- `bot_id` (required): ID of the bot
- `flow_id` (required): ID of the flow

**Example LLM prompt**: "Show me the flow flow_456 in bot bot_123"

---

### botpress_create_flow

**What it does**: Create a new flow in a Botpress bot. Flows automate conversations based on user inputs and logic.

**When to use**: Set up new conversation paths for a bot.

**Arguments**:
- `bot_id` (required): ID of the bot
- `name` (required): Name of the flow
- `nodes` (optional): Array of flow nodes
- `transitions` (optional): Array of transitions between nodes

**Example LLM prompt**: "Create a flow called 'Welcome Flow' in bot bot_123"

---

### botpress_list_intents

**What it does**: Retrieve all intents in a Botpress bot. Intents represent user intentions or goals.

**When to use**: Browse available intents for NLU configuration.

**Arguments**:
- `bot_id` (required): ID of the bot
- `page` (optional): Page number for pagination

**Example LLM prompt**: "List all intents in bot bot_123"

---

### botpress_get_intent

**What it does**: Get details of a specific intent including utterances and slots. Intents are trained with example phrases to recognize user goals.

**When to use**: Review intent training data before modifying.

**Arguments**:
- `bot_id` (required): ID of the bot
- `intent_id` (required): ID of the intent

**Example LLM prompt**: "Show me the intent intent_789 in bot bot_123"

---

### botpress_list_entities

**What it does**: Retrieve all entities in a Botpress bot. Entities extract specific data types from user input.

**When to use**: Browse available entities for NLU configuration.

**Arguments**:
- `bot_id` (required): ID of the bot
- `type` (optional): Filter by entity type (system, custom)

**Example LLM prompt**: "List all custom entities in bot bot_123"

---

### botpress_get_entity

**What it does**: Get details of a specific entity including synonyms and patterns. Entities extract structured information like dates, names, or locations.

**When to use**: Review entity configuration for NLU training.

**Arguments**:
- `bot_id` (required): ID of the bot
- `entity_id` (required): ID of the entity

**Example LLM prompt**: "Show me the entity entity_456 in bot bot_123"

---

## Botpress API Reference

These tools use the Botpress API. See official docs for full details:
- https://botpress.com/docs
- Rate limits: Check your plan limits
- Pagination: Use `page` and `limit` parameters
- All dates: ISO 8601 format
