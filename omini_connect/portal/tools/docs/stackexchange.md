# Stack Exchange Tools

Provider: `stackexchange` | Engine: `nango` | Auth: OAUTH2

## Overview

These tools wrap the Stack Exchange API. They allow AI agents to interact with Stack Exchange functionality. **Requires OAUTH2 authentication.**

## Authentication

**OAuth2 Authentication**:
- User authenticates via OAuth2 authorization code flow
- Nango manages the OAuth handshake and token refresh
- Default scopes depend on the provider configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_questions` | List questions | GET | /questions |
| `get_question` | Get question details | GET | /questions/{id} |
| `list_answers` | List answers for question | GET | /questions/{id}/answers |
| `get_answer` | Get answer details | GET | /answers/{id} |
| `list_users` | List users | GET | /users |
| `get_user` | Get user details | GET | /users/{id} |
| `search_questions` | Search questions | GET | /search/advanced |
| `list_tags` | List all tags | GET | /tags |
| `get_site_info` | Get site information | GET | /info |
| `list_comments` | List comments | GET | /comments |

---

## Tool Details

### list_questions

**What it does**: List questions

**When to use**: Use this tool when you need to list questions.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list questions to..."

---

### get_question

**What it does**: Get question details

**When to use**: Use this tool when you need to get question details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get question to..."

---

### list_answers

**What it does**: List answers for question

**When to use**: Use this tool when you need to list answers for question.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list answers to..."

---

### get_answer

**What it does**: Get answer details

**When to use**: Use this tool when you need to get answer details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get answer to..."

---

### list_users

**What it does**: List users

**When to use**: Use this tool when you need to list users.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list users to..."

---

### get_user

**What it does**: Get user details

**When to use**: Use this tool when you need to get user details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get user to..."

---

### search_questions

**What it does**: Search questions

**When to use**: Use this tool when you need to search questions.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use search questions to..."

---

### list_tags

**What it does**: List all tags

**When to use**: Use this tool when you need to list all tags.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list tags to..."

---

### get_site_info

**What it does**: Get site information

**When to use**: Use this tool when you need to get site information.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get site info to..."

---

### list_comments

**What it does**: List comments

**When to use**: Use this tool when you need to list comments.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list comments to..."

---

## Stack Exchange API Notes

- **Auth mode**: OAUTH2
- **Base URL**: https://api.stackexchange.com
- **API prefix**: /2.3
- **Rate limits**: Check provider documentation for specific limits
