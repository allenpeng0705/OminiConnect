# Substack Tools

Provider: `substack` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Substack API. They allow AI agents to manage newsletters, posts, subscribers, and email campaigns. Substack is a newsletter platform known for independent journalism, paid subscriptions, and direct reader relationships.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Substack
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `substack_list_publications` | List publications | GET | /publications |
| `substack_get_publication` | Get publication details | GET | /publications/{publication_id} |
| `substack_list_posts` | List posts | GET | /publications/{publication_id}/posts |
| `substack_get_post` | Get post details | GET | /posts/{post_id} |
| `substack_create_post` | Create a new post | POST | /publications/{publication_id}/posts |
| `substack_list_subscribers` | List subscribers | GET | /publications/{publication_id}/subscribers |
| `substack_get_subscriber` | Get subscriber details | GET | /subscribers/{subscriber_id} |
| `substack_send_email` | Send an email | POST | /publications/{publication_id}/emails |
| `substack_get_email_stats` | Get email statistics | GET | /emails/{email_id}/stats |
| `substack_list_free_posts` | List free posts | GET | /publications/{publication_id}/posts/free |

---

## Tool Details

### substack_list_publications

**What it does**: Retrieves a list of publications the user is associated with.

**When to use**: Find all your publications or switch between different newsletters.

**Arguments**:
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "List all my Substack publications"

---

### substack_get_publication

**What it does**: Gets details about a specific publication.

**When to use**: Review publication settings, subscriber counts, or publication details.

**Arguments**:
- `publication_id` (required): The publication ID

**Example LLM prompt**: "Get details for publication abc123"

---

### substack_list_posts

**What it does**: Retrieves a list of posts for a publication.

**When to use**: Browse your archive, find posts to promote, or check publishing history.

**Arguments**:
- `publication_id` (required): The publication ID
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "List all posts in my newsletter"

---

### substack_get_post

**What it does**: Gets details about a specific post.

**When to use**: Read a specific post before sharing or analyzing its content.

**Arguments**:
- `post_id` (required): The post ID

**Example LLM prompt**: "Get details for post xyz789"

---

### substack_create_post

**What it does**: Creates a new post in a publication.

**When to use**: Publish articles, announcements, or newsletters to your subscribers.

**Arguments**:
- `publication_id` (required): The publication ID
- `title` (required): The post title
- `content` (required): The post content in HTML or markdown
- `is_paid` (optional): Whether the post is for paid subscribers only
- `send_email` (optional): Whether to send the post as an email
- `scheduled_at` (optional): ISO 8601 datetime to schedule publication

**Example LLM prompt**: "Create a new post titled 'Weekly Update' with content about product launches"

---

### substack_list_subscribers

**What it does**: Retrieves a list of subscribers for a publication.

**When to use**: Review your audience, see subscriber growth, or find specific subscribers.

**Arguments**:
- `publication_id` (required): The publication ID
- `status` (optional): Filter by subscription status (active, cancelled, expired, all)
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "List all active subscribers"

---

### substack_get_subscriber

**What it does**: Gets details about a specific subscriber.

**When to use**: Look up a specific subscriber's details or subscription tier.

**Arguments**:
- `subscriber_id` (required): The subscriber ID

**Example LLM prompt**: "Get details for subscriber john@example.com"

---

### substack_send_email

**What it does**: Sends an email to subscribers of a publication.

**When to use**: Send newsletters, announcements, or post notifications to your audience.

**Arguments**:
- `publication_id` (required): The publication ID
- `post_id` (required): The post ID to send as email
- `recipient_type` (optional): Which subscribers to send to (all, free, paid)

**Example LLM prompt**: "Send post xyz789 as an email to all subscribers"

---

### substack_get_email_stats

**What it does**: Gets statistics for a sent email.

**When to use**: Measure email performance, open rates, and subscriber engagement.

**Arguments**:
- `email_id` (required): The email ID

**Example LLM prompt**: "Get stats for email abc123 to see open rates"

---

### substack_list_free_posts

**What it does**: Retrieves a list of free (non-paid) posts for a publication.

**When to use**: Find accessible content to share or preview your free offerings.

**Arguments**:
- `publication_id` (required): The publication ID
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "List all free posts in my publication"

---

## Substack API Notes

- **Free vs Paid**: Posts can be free for all subscribers or restricted to paid subscribers
- **Email Delivery**: Posts can be sent immediately or scheduled for email delivery
- **Subscribers**: Subscriber status can be active, cancelled, or expired
- **Email Stats**: Track open rates, click rates, and subscriber engagement
- **Publications**: Each Substack account can have multiple publications
