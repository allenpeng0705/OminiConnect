# Freshdesk More Tools

Provider: `freshdesk_more` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Freshdesk REST API (v2). They allow AI agents to manage solutions, categories, agents, and teams in your Freshdesk account.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `solutions`, `agents`, `teams`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `freshdesk_list_solutions` | List all solutions/articles | GET | /solutions |
| `freshdesk_get_solution` | Get a specific solution | GET | /solutions/{id} |
| `freshdesk_create_solution` | Create a new solution | POST | /solutions |
| `freshdesk_list_categories` | List solution categories | GET | /solution/categories |
| `freshdesk_get_category` | Get a specific category | GET | /solution/categories/{id} |
| `freshdesk_list_agents` | List all agents | GET | /agents |
| `freshdesk_get_agent` | Get a specific agent | GET | /agents/{id} |
| `freshdesk_list_teams` | List all teams | GET | /teams |
| `freshdesk_get_team` | Get a specific team | GET | /teams/{id} |
| `freshdesk_get_forum_settings` | Get forum settings | GET | /forum_settings |

---

## Tool Details

### freshdesk_list_solutions

**What it does**: Lists all solutions (articles) in your Freshdesk knowledge base. Returns title, description, and category.

**When to use**: Browse your knowledge base to find relevant help articles for customers.

**Arguments**:
- `per_page` (optional): default 30
- `page` (optional): default 1

**Example LLM prompt**: "Show me all solutions in the knowledge base"

---

### freshdesk_get_solution

**What it does**: Gets full details of a specific solution article including the full content, metadata, and category.

**When to use**: Read the full content of a solution before recommending it to customers.

**Arguments**:
- `id` (required): Solution ID

**Example LLM prompt**: "Show me solution #123"

---

### freshdesk_create_solution

**What it does**: Creates a new solution article in your Freshdesk knowledge base. Include title, description, and content.

**When to use**: Add new articles to your knowledge base to help customers self-serve.

**Arguments**:
- `title` (required): Solution title
- `description` (required): Short description
- `body` (required): Solution content (HTML supported)
- `category_id` (optional): Category ID to place the solution in
- `status` (optional): `published` or `draft` — default `published`

**Example LLM prompt**: "Create a solution titled 'How to reset password' with body content explaining the steps"

---

### freshdesk_list_categories

**What it does**: Lists all categories in your Freshdesk solution portal. Categories group solutions together.

**When to use**: See available categories before creating or organizing solutions.

**Arguments**:
- `per_page` (optional): default 30
- `page` (optional): default 1

**Example LLM prompt**: "What categories do I have in the knowledge base?"

---

### freshdesk_get_category

**What it does**: Gets details of a specific category including name, description, and list of solutions.

**When to use**: Check what solutions are in a category before organizing content.

**Arguments**:
- `id` (required): Category ID

**Example LLM prompt**: "Show me category #5"

---

### freshdesk_list_agents

**What it does**: Lists all agents in your Freshdesk account with their name, email, role, and status.

**When to use**: Find available support agents or see team composition.

**Arguments**:
- `per_page` (optional): default 30
- `page` (optional): default 1

**Example LLM prompt**: "List all agents in the system"

---

### freshdesk_get_agent

**What it does**: Gets details of a specific agent including their profile, skills, availability, and current ticket assignments.

**When to use**: Check an agent's workload or competence before assigning a ticket.

**Arguments**:
- `id` (required): Agent ID

**Example LLM prompt**: "Show me agent #42"

---

### freshdesk_list_teams

**What it does**: Lists all teams in your Freshdesk account. Teams group agents together for ticket assignment.

**When to use**: See available teams before creating tickets or assigning work.

**Arguments**:
- `per_page` (optional): default 30
- `page` (optional): default 1

**Example LLM prompt**: "What teams do I have?"

---

### freshdesk_get_team

**What it does**: Gets details of a specific team including its members, name, and description.

**When to use**: Check team composition before assigning tickets.

**Arguments**:
- `id` (required): Team ID

**Example LLM prompt**: "Show me team #1 and its members"

---

### freshdesk_get_forum_settings

**What it does**: Gets the global forum settings for your Freshdesk community portal. Includes moderation settings and display preferences.

**When to use**: Understand community portal configuration before managing discussions.

**Arguments**: None

**Example LLM prompt**: "What are the current forum settings?"

---

## Freshdesk API Reference

These tools use the Freshdesk REST API v2. See official docs for full details:
- https://developers.freshdesk.com/api/
- Rate limits: Based on plan (100-1000 requests/minute)
- Pagination: Use `per_page` and `page` parameters
- All dates: ISO 8601 format
