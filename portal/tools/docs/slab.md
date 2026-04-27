# Slab Tools

Provider: `slab` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Slab API. They allow AI agents to interact with Slab functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_posts` | List all posts | GET | /posts |
| `get_post` | Get post details | GET | /posts/{id} |
| `create_post` | Create a new post | POST | /posts |
| `update_post` | Update a post | PUT | /posts/{id} |
| `list_topics` | List all topics | GET | /topics |
| `get_topic` | Get topic details | GET | /topics/{id} |
| `list_members` | List workspace members | GET | /members |
| `list_invitations` | List pending invitations | GET | /invitations |
| `create_invitation` | Invite a member | POST | /invitations |
| `get_workspace` | Get workspace settings | GET | /workspace |

---

## Tool Details

### list_posts

**What it does**: List all posts

**When to use**: Use this tool when you need to list all posts.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list posts to..."

---

### get_post

**What it does**: Get post details

**When to use**: Use this tool when you need to get post details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get post to..."

---

### create_post

**What it does**: Create a new post

**When to use**: Use this tool when you need to create a new post.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create post to..."

---

### update_post

**What it does**: Update a post

**When to use**: Use this tool when you need to update a post.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use update post to..."

---

### list_topics

**What it does**: List all topics

**When to use**: Use this tool when you need to list all topics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list topics to..."

---

### get_topic

**What it does**: Get topic details

**When to use**: Use this tool when you need to get topic details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get topic to..."

---

### list_members

**What it does**: List workspace members

**When to use**: Use this tool when you need to list workspace members.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list members to..."

---

### list_invitations

**What it does**: List pending invitations

**When to use**: Use this tool when you need to list pending invitations.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list invitations to..."

---

### create_invitation

**What it does**: Invite a member

**When to use**: Use this tool when you need to invite a member.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create invitation to..."

---

### get_workspace

**What it does**: Get workspace settings

**When to use**: Use this tool when you need to get workspace settings.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get workspace to..."

---

## Slab API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://api.slab.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
