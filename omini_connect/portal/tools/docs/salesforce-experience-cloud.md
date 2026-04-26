# Salesforce-experience-cloud Tools

Provider: `salesforce-experience-cloud` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Salesforce Experience Cloud provides community and portal features. These tools allow AI agents to manage communities, Chatter groups, files, knowledge articles, and user profiles.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Salesforce Experience Cloud
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `communities:read`, `chatter:read`, `chatter:write`, `files:read`, `files:write`, `knowledge:read`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `salesforce-experience-cloud_list_communities` | List communities | GET | /v1/communities |
| `salesforce-experience-cloud_get_community` | Get community details | GET | /v1/communities/{communityId} |
| `salesforce-experience-cloud_list_chatters` | List Chatter groups | GET | /v1/chatter/groups |
| `salesforce-experience-cloud_create_chatter_post` | Create Chatter post | POST | /v1/chatter/posts |
| `salesforce-experience-cloud_list_files` | List files | GET | /v1/files |
| `salesforce-experience-cloud_upload_file` | Upload a file | POST | /v1/files |
| `salesforce-experience-cloud_list_articles` | List knowledge articles | GET | /v1/knowledge/articles |
| `salesforce-experience-cloud_get_article` | Get article details | GET | /v1/knowledge/articles/{articleId} |
| `salesforce-experience-cloud_list_users` | List community users | GET | /v1/users |
| `salesforce-experience-cloud_get_user_profile` | Get user profile | GET | /v1/users/{userId} |

---

## Tool Details

### salesforce-experience-cloud_list_communities

**What it does**: Returns a list of all communities.

**When to use**: View available communities.

**Arguments**:
- `limit` (optional): Number of communities (default 50)

**Example LLM prompt**: "List all communities"

---

### salesforce-experience-cloud_get_community

**What it does**: Gets details of a specific community.

**When to use**: Get community settings and status.

**Arguments**:
- `communityId` (required): The community ID

**Example LLM prompt**: "Get details for community com_abc123"

---

### salesforce-experience-cloud_list_chatters

**What it does**: Returns Chatter groups.

**When to use**: Find collaboration groups.

**Arguments**:
- `limit` (optional): Number of groups (default 50)

**Example LLM prompt**: "List all Chatter groups"

---

### salesforce-experience-cloud_create_chatter_post

**What it does**: Creates a post in a Chatter group.

**When to use**: Share updates with community.

**Arguments**:
- `groupId` (required): Chatter group ID
- `message` (required): Post message

**Example LLM prompt**: "Post 'Welcome everyone!' to group grp_123"

---

### salesforce-experience-cloud_list_files

**What it does**: Returns files in the community.

**When to use**: Browse shared documents.

**Arguments**:
- `limit` (optional): Number of files (default 50)
- `folderId` (optional): Filter by folder

**Example LLM prompt**: "List all files in the community"

---

### salesforce-experience-cloud_upload_file

**What it does**: Uploads a file to the community.

**When to use**: Share documents.

**Arguments**:
- `name` (required): File name
- `content` (required): File content (base64)
- `folderId` (optional): Folder ID

**Example LLM prompt**: "Upload a file called 'report.pdf'"

---

### salesforce-experience-cloud_list_articles

**What it does**: Returns knowledge articles.

**When to use**: Browse help content.

**Arguments**:
- `limit` (optional): Number of articles (default 50)
- `category` (optional): Filter by category

**Example LLM prompt**: "List all knowledge articles"

---

### salesforce-experience-cloud_get_article

**What it does**: Gets details of a knowledge article.

**When to use**: Read article content.

**Arguments**:
- `articleId` (required): The article ID

**Example LLM prompt**: "Get article art_xyz789"

---

### salesforce-experience-cloud_list_users

**What it does**: Returns community users.

**When to use**: View community members.

**Arguments**:
- `limit` (optional): Number of users (default 50)

**Example LLM prompt**: "List all community users"

---

### salesforce-experience-cloud_get_user_profile

**What it does**: Gets user profile information.

**When to use**: Get user details.

**Arguments**:
- `userId` (required): The user ID

**Example LLM prompt**: "Get profile for user usr_123"

---

## Experience Cloud Notes

- Communities provide customer/partner portals
- Chatter enables collaboration
- Files share documents
- Knowledge articles provide self-service help
- Users are community members
