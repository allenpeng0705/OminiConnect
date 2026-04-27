# Canny Tools

Provider: `canny` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Canny API for feature tracking and user feedback management. They allow AI agents to interact with boards, posts, comments, and votes.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `boards:read`, `posts:read`, `posts:create`, `posts:vote`, `comments:read`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `canny_list_boards` | List all boards | GET | /boards |
| `canny_get_board` | Get details of a specific board | GET | /boards |
| `canny_list_posts` | List posts in a board | GET | /posts |
| `canny_get_post` | Get details of a specific post | GET | /posts |
| `canny_create_post` | Create a new post | POST | /posts |
| `canny_list_comments` | List comments on a post | GET | /comments |
| `canny_get_comment` | Get details of a specific comment | GET | /comments |
| `canny_list_users` | List users | GET | /users |
| `canny_get_user` | Get details of a specific user | GET | /users |
| `canny_vote_on_post` | Vote on a post | POST | /posts/vote |

---

## Tool Details

### canny_list_boards

**What it does**: Returns a list of all boards in your Canny workspace. Boards contain grouped feature requests.

**When to use**: Discover available boards before listing or creating posts.

**Arguments**:
- `limit` (optional): Maximum number of boards (default 50)
- `skip` (optional): Number to skip for pagination

**Example LLM prompt**: "List all boards in our Canny workspace"

---

### canny_get_board

**What it does**: Get details of a specific board including name, description, post counts, and category.

**When to use**: Understand a board's focus before browsing or posting feature requests.

**Arguments**:
- `id` (required): Board ID

**Example LLM prompt**: "Get details for board abc-123"

---

### canny_list_posts

**What it does**: Returns a paginated list of posts in a board. Filter by status, author, or visibility.

**When to use**: Browse feature requests, find related posts, or understand popular requests.

**Arguments**:
- `boardID` (required): Board ID to filter posts by
- `status` (optional): Filter by post status
- `authorID` (optional): Filter by author user ID
- `limit` (optional): Maximum number of posts (default 50)
- `skip` (optional): Number to skip

**Example LLM prompt**: "Show me all open posts in the product board"

---

### canny_get_post

**What it does**: Get full details of a specific post including title, body, author, status, and vote count.

**When to use**: Read a feature request before responding or voting on it.

**Arguments**:
- `id` (required): Post ID

**Example LLM prompt**: "Get details for post xyz-456"

---

### canny_create_post

**What it does**: Create a new feature request post in a specified board.

**When to use**: File a new feature request, bug report, or improvement idea from conversation.

**Arguments**:
- `boardID` (required): Board ID to create post in
- `title` (required): Post title
- `details` (optional): Detailed description
- `authorID` (optional): User ID of the post author
- `categoryID` (optional): Category ID for the post

**Example LLM prompt**: "Create a post in the product board titled 'Add dark mode' with details about the feature request"

---

### canny_list_comments

**What it does**: Returns a paginated list of comments on a specific post.

**When to use**: Read discussion around a feature request before contributing.

**Arguments**:
- `postID` (required): Post ID to get comments for
- `limit` (optional): Maximum number of comments (default 50)
- `skip` (optional): Number to skip

**Example LLM prompt**: "Show me all comments on post xyz-456"

---

### canny_get_comment

**What it does**: Get details of a specific comment including text, author, and creation date.

**When to use**: Read a specific comment in full.

**Arguments**:
- `id` (required): Comment ID

**Example LLM prompt**: "Get details for comment 789"

---

### canny_list_users

**What it does**: Returns a paginated list of users in your Canny workspace.

**When to use**: Find users to attribute posts or comments to.

**Arguments**:
- `limit` (optional): Maximum number of users (default 50)
- `skip` (optional): Number to skip

**Example LLM prompt**: "List all users in Canny"

---

### canny_get_user

**What it does**: Get details of a specific user including name, email, and avatar.

**When to use**: Look up a specific user's profile.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user 123"

---

### canny_vote_on_post

**What it does**: Vote on a feature request post. Supports both upvoting and removing a vote.

**When to use**: Support or remove support for a feature request.

**Arguments**:
- `postID` (required): Post ID to vote on
- `voterID` (required): User ID of the voter
- `type` (optional): Vote type - `upvote` or `removeVote` (default: upvote)

**Example LLM prompt**: "Upvote post xyz-456 for voter 123"

---

## Canny API Reference

These tools use the Canny API. See official docs for full details:
- https://developers.canny.io
- Rate limits: Follow your Canny plan limits
- Pagination: Use `limit` and `skip` parameters
- All IDs are strings (Canny uses string identifiers)
