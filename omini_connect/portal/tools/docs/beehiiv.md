# Beehiiv Tools

Provider: `beehiiv` | Engine: `nango` | Auth: API Key (via Nango)

## Overview

These tools wrap the Beehiiv API v2. They allow AI agents to manage publications, posts, subscribers, campaigns, and analytics for the authenticated account.

## Authentication

**Nango (API Key)**:
- User provides Beehiiv API key
- Token stored in Nango, accessed via `connection_ref`
- Requires publication ID in connection config
- Headers: `Authorization: Bearer {apiKey}` set automatically

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `beehiiv_list_publications` | List publications in the account | GET | /publications |
| `beehiiv_get_publication` | Get details of a specific publication | GET | /publications/{publication_id} |
| `beehiiv_list_posts` | List posts in a publication | GET | /posts |
| `beehiiv_get_post` | Get details of a specific post | GET | /posts/{post_id} |
| `beehiiv_create_post` | Create a new post | POST | /posts |
| `beehiiv_list_subscribers` | List subscribers | GET | /subscribers |
| `beehiiv_get_subscriber` | Get details of a specific subscriber | GET | /subscribers/{subscriber_id} |
| `beehiiv_create_campaign` | Create a new email campaign | POST | /campaigns |
| `beehiiv_get_analytics` | Get analytics for a publication | GET | /analytics |
| `beehiiv_get_report` | Get detailed report data | GET | /reports |

---

## Tool Details

### beehiiv_list_publications

**What it does**: List all publications in the Beehiiv account.

**When to use**: Find available publications before managing posts, subscribers, or campaigns.

**Arguments**: None

**Example LLM prompt**: "List all my publications"

---

### beehiiv_get_publication

**What it does**: Get details of a specific publication including name, settings, and growth statistics.

**When to use**: Check publication settings or view growth metrics.

**Arguments**:
- `publication_id` (required): Publication ID

**Example LLM prompt**: "Get details for publication pub_abc123"

---

### beehiiv_list_posts

**What it does**: List all posts in a publication with title, status, and publish date.

**When to use**: Browse published content or find drafts to edit.

**Arguments**:
- `publication_id` (required): Publication ID
- `status` (optional): Filter by status: draft, scheduled, published
- `limit` (optional, max 100): default 25
- `offset` (optional): default 0

**Example LLM prompt**: "List all published posts for publication pub_abc123"

---

### beehiiv_get_post

**What it does**: Get details of a specific post including content, status, and engagement metrics.

**When to use**: Read full post content or check performance.

**Arguments**:
- `post_id` (required): Post ID

**Example LLM prompt**: "Get details for post post_xyz789"

---

### beehiiv_create_post

**What it does**: Create a new post in a publication as draft or scheduled.

**When to use**: Write and schedule newsletter content.

**Arguments**:
- `publication_id` (required): Publication ID
- `title` (required): Post title
- `body` (optional): Post content (HTML or plain text)
- `status` (optional): draft or scheduled, default draft
- `scheduled_at` (optional): Publish date/time (ISO 8601) if scheduling
- `tags` (optional): Array of tag names

**Example LLM prompt**: "Create a draft post titled 'April Newsletter' for publication pub_abc123"

---

### beehiiv_list_subscribers

**What it does**: List all subscribers in a publication with email, status, and subscription date.

**When to use**: Review audience growth or find specific subscribers.

**Arguments**:
- `publication_id` (required): Publication ID
- `status` (optional): Filter by status: active, unsubscribed, bounced
- `limit` (optional, max 100): default 25
- `offset` (optional): default 0

**Example LLM prompt**: "List all active subscribers for publication pub_abc123"

---

### beehiiv_get_subscriber

**WhatWhat it does**: Get details of a specific subscriber including email, status, and engagement metrics.

**When to use**: Check subscriber profile or subscription history.

**Arguments**:
- `subscriber_id` (required): Subscriber ID

**Example LLM prompt**: "Get details for subscriber sub_def456"

---

### beehiiv_create_campaign

**What it does**: Create a new email campaign for a publication.

**When to use**: Send newsletters or promotional emails to subscribers.

**Arguments**:
- `publication_id` (required): Publication ID
- `subject` (required): Email subject line
- `body` (optional): Email content (HTML)
- `send_at` (optional): Send date/time (ISO 8601)
- `segment_id` (optional): Target segment ID

**Example LLM prompt**: "Create a campaign for publication pub_abc123 with subject 'April Update'"

---

### beehiiv_get_analytics

**What it does**: Get analytics for a publication including subscriber growth, open rates, and click rates.

**When to use**: Track publication performance or review engagement metrics.

**Arguments**:
- `publication_id` (required): Publication ID
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get analytics for publication pub_abc123 for the last 30 days"

---

### beehiiv_get_report

**What it does**: Get detailed report data including subscriber demographics and engagement breakdown.

**When to use**: Generate detailed reports or analyze audience segments.

**Arguments**:
- `publication_id` (required): Publication ID
- `report_type` (optional): subscribers, engagement, or revenue, default subscribers

**Example LLM prompt**: "Get subscriber report for publication pub_abc123"

---

## Beehiiv API Reference

These tools use the Beehiiv API v2. See official docs for full details:
- https://api.beehiiv.com/docs/v2
- Rate limits: Vary by plan
- Pagination: Use `limit` and `offset` parameters
- All dates: ISO 8601 format (UTC)
