# Ghost Tools

Provider: `ghost` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Ghost CMS API. They allow AI agents to manage posts, pages, members, tags, and content settings. Ghost is a professional publishing platform focused on independent journalism, blogs, and newsletters with built-in membership and subscription features.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Ghost
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`, `delete`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ghost_list_posts` | List posts | GET | /posts |
| `ghost_get_post` | Get post details | GET | /posts/{post_id} |
| `ghost_create_post` | Create a new post | POST | /posts |
| `ghost_update_post` | Update an existing post | PUT | /posts/{post_id} |
| `ghost_delete_post` | Delete a post | DELETE | /posts/{post_id} |
| `ghost_list_pages` | List pages | GET | /pages |
| `ghost_get_page` | Get page details | GET | /pages/{page_id} |
| `ghost_list_members` | List members | GET | /members |
| `ghost_get_member` | Get member details | GET | /members/{member_id} |
| `ghost_list_tags` | List tags | GET | /tags |

---

## Tool Details

### ghost_list_posts

**What it does**: Retrieves a list of posts from the Ghost CMS.

**When to use**: Browse your content archive, find posts to update, or check publishing history.

**Arguments**:
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 25)
- `filter` (optional): Filter expression (e.g., 'status:published+author:admin')
- `include` (optional): Related resources to include (e.g., 'tags,authors')

**Example LLM prompt**: "List all my published posts"

---

### ghost_get_post

**What it does**: Gets details about a specific post by its ID or slug.

**When to use**: Read full post details before editing or sharing.

**Arguments**:
- `post_id` (required): The post ID or slug
- `include` (optional): Related resources to include (e.g., 'tags,authors')

**Example LLM prompt**: "Get details for post my-blog-post"

---

### ghost_create_post

**What it does**: Creates a new post in Ghost.

**When to use**: Publish articles, blog posts, or content to your Ghost site.

**Arguments**:
- `title` (required): The post title
- `html` (optional): The post content in HTML
- `markdown` (optional): The post content in markdown
- `status` (optional): Publication status (draft, published, scheduled)
- `published_at` (optional): ISO 8601 datetime for scheduled publication
- `tags` (optional): Tags to assign to the post
- `authors` (optional): Authors to assign to the post

**Example LLM prompt**: "Create a new post titled 'Welcome to My Blog' with the content"

---

### ghost_update_post

**What it does**: Updates an existing post in Ghost.

**When to use**: Fix errors, add updates, change tags, or modify publish status.

**Arguments**:
- `post_id` (required): The post ID or slug
- `title` (optional): Updated post title
- `html` (optional): Updated post content in HTML
- `markdown` (optional): Updated post content in markdown
- `status` (optional): Updated publication status
- `tags` (optional): Updated tags

**Example LLM prompt**: "Update post my-blog-post to add the tag 'announcements'"

---

### ghost_delete_post

**What it does**: Deletes a post from Ghost.

**When to use**: Remove outdated or unwanted content.

**Arguments**:
- `post_id` (required): The post ID or slug to delete

**Example LLM prompt**: "Delete post my-old-post"

---

### ghost_list_pages

**What it does**: Retrieves a list of pages from the Ghost CMS.

**When to use**: Browse static pages like About, Contact, or legal pages.

**Arguments**:
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 25)
- `filter` (optional): Filter expression
- `include` (optional): Related resources to include

**Example LLM prompt**: "List all pages on my site"

---

### ghost_get_page

**What it does**: Gets details about a specific page by its ID or slug.

**When to use**: Read full page details before editing.

**Arguments**:
- `page_id` (required): The page ID or slug
- `include` (optional): Related resources to include

**Example LLM prompt**: "Get details for page about-us"

---

### ghost_list_members

**What it does**: Retrieves a list of members from the Ghost CMS.

**When to use**: View your audience, check subscription status, or find specific members.

**Arguments**:
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 25)
- `filter` (optional): Filter expression (e.g., 'subscribed:true')
- `include` (optional): Related resources to include

**Example LLM prompt**: "List all active members"

---

### ghost_get_member

**What it does**: Gets details about a specific member by their ID or email.

**When to use**: Look up a specific member's profile or subscription details.

**Arguments**:
- `member_id` (required): The member ID or email
- `include` (optional): Related resources to include

**Example LLM prompt**: "Get details for member john@example.com"

---

### ghost_list_tags

**What it does**: Retrieves a list of tags from the Ghost CMS.

**When to use**: View all tags, find tags for organizing content, or check tag usage.

**Arguments**:
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 25)
- `filter` (optional): Filter expression
- `include` (optional): Related resources to include

**Example LLM prompt**: "List all tags on my site"

---

## Ghost API Notes

- **Content Format**: Ghost supports both HTML and Markdown for post content
- **Status**: Posts can be draft, published, or scheduled for future publication
- **Members**: Ghost has built-in membership and subscription features
- **Tags**: Use tags to organize and categorize your content
- **Filter Syntax**: Use expressions like 'status:published', 'author:admin', or 'tags:[news,updates]'
- **Include**: Related resources like tags, authors, and tiers can be included in responses
