# Medium Tools

Provider: `medium` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Medium API. They allow AI agents to manage posts, publications, user profiles, and content discovery. Medium is a publishing platform for long-form content, known for thoughtful articles on technology, business, culture, and more.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Medium
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `basic`, `publish`, `delete`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `medium_list_posts` | List user's posts | GET | /users/{user_id}/posts |
| `medium_get_post` | Get post details | GET | /posts/{post_id} |
| `medium_create_post` | Create a new post | POST | /users/{user_id}/posts |
| `medium_update_post` | Update an existing post | PUT | /posts/{post_id} |
| `medium_delete_post` | Delete a post | DELETE | /posts/{post_id} |
| `medium_list_publication_posts` | List publication posts | GET | /publications/{publication_id}/posts |
| `medium_get_user_info` | Get user profile | GET | /users/{user_id} |
| `medium_list_followers` | List user's followers | GET | /users/{user_id}/followers |
| `medium_get_follower` | Get follower details | GET | /users/{user_id}/followers/{follower_id} |
| `medium_search_posts` | Search posts | GET | /search/posts |

---

## Tool Details

### medium_list_posts

**What it does**: Retrieves a list of posts for the authenticated user.

**When to use**: Browse your existing content, find posts to update, or check publishing history.

**Arguments**:
- `user_id` (required): The user ID
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "List all my Medium posts"

---

### medium_get_post

**What it does**: Gets details about a specific post by its ID.

**When to use**: Read full post details before editing or sharing.

**Arguments**:
- `post_id` (required): The post ID

**Example LLM prompt**: "Get the details of post abc123"

---

### medium_create_post

**What it does**: Creates a new post on Medium.

**When to use**: Publish articles, blog posts, or long-form content to your Medium profile or publications.

**Arguments**:
- `user_id` (required): The user ID
- `title` (required): The post title
- `content_format` (required): Format of the content (html or markdown)
- `content` (required): The post content
- `tags` (optional): Tags for the post
- `publish_status` (optional): Publication status (public, draft, unlisted)

**Example LLM prompt**: "Create a new Medium post titled 'The Future of AI' in markdown format"

---

### medium_update_post

**What it does**: Updates an existing post on Medium.

**When to use**: Fix errors, add updates, change tags, or modify publish status.

**Arguments**:
- `post_id` (required): The post ID
- `title` (optional): The updated post title
- `content` (optional): The updated post content
- `content_format` (optional): Format of the content (html or markdown)
- `tags` (optional): Updated tags for the post
- `publish_status` (optional): Updated publication status

**Example LLM prompt**: "Update post abc123 to add the tag 'machine-learning'"

---

### medium_delete_post

**What it does**: Deletes a post from Medium.

**When to use**: Remove outdated or unwanted content.

**Arguments**:
- `post_id` (required): The post ID to delete

**Example LLM prompt**: "Delete post abc123"

---

### medium_list_publication_posts

**What it does**: Retrieves posts from a specific publication.

**When to use**: Browse posts in a publication you're following or contributing to.

**Arguments**:
- `publication_id` (required): The publication ID
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "List posts in the towards-data-science publication"

---

### medium_get_user_info

**What it does**: Gets profile information for the authenticated user.

**When to use**: Look up user profiles, verify identity, or find publication associations.

**Arguments**:
- `user_id` (required): The user ID

**Example LLM prompt**: "Get my Medium profile"

---

### medium_list_followers

**What it does**: Retrieves the list of followers for a user.

**When to use**: See your audience, find connections, or understand your reach.

**Arguments**:
- `user_id` (required): The user ID
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "List my Medium followers"

---

### medium_get_follower

**What it does**: Gets details about a specific follower.

**When to use**: Look up a specific follower's profile.

**Arguments**:
- `user_id` (required): The user ID
- `follower_id` (required): The follower ID

**Example LLM prompt**: "Get details about follower xyz789"

---

### medium_search_posts

**What it does**: Searches for posts matching a query.

**When to use**: Find articles on a specific topic or discover new content.

**Arguments**:
- `q` (required): Search query string
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "Search for posts about artificial intelligence"

---

## Medium API Notes

- **Content Format**: Medium accepts both HTML and Markdown for post content
- **Tags**: Use up to 5 tags per post for optimal discoverability
- **Publications**: Submit posts to publications for broader reach
- **Distribution**: Posts can be public (visible to all), unlisted (link only), or draft
- **Followers**: Track your audience growth through follower metrics
