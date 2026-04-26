# Reddit Tools

Provider: `reddit` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Reddit API. They allow AI agents to browse subreddits, read and create posts, manage comments, and search for content. Reddit is the largest community discussion platform with millions of active communities.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Reddit
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `submit`, `subscribe`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `reddit_list_subreddits` | List popular subreddits | GET | /subreddits.json |
| `reddit_get_subreddit` | Get subreddit details | GET | /r/{subreddit}/about.json |
| `reddit_list_posts` | List posts in a subreddit | GET | /r/{subreddit}/{sort}.json |
| `reddit_get_post` | Get a specific post | GET | /r/{subreddit}/comments/{post_id}.json |
| `reddit_create_post` | Create a new post | POST | /api/submit.json |
| `reddit_list_comments` | List comments on a post | GET | /r/{subreddit}/comments/{post_id}.json |
| `reddit_get_comment` | Get a specific comment | GET | /comments/{comment_id}.json |
| `reddit_create_comment` | Create a comment | POST | /api/comment.json |
| `reddit_get_user_info` | Get user profile information | GET | /user/{username}/about.json |
| `reddit_search_posts` | Search posts by keyword | GET | /search.json |

---

## Tool Details

### reddit_list_subreddits

**What it does**: Lists popular subreddits. Returns a list of trending communities on Reddit.

**When to use**: Discover new communities, browse by interest, find trending topics.

**Arguments**:
- `limit` (optional): Number of results (max 100, default 25)
- `after` (optional): Fullname of the item after which to fetch results

**Example LLM prompt**: "Show me popular subreddits about technology"

---

### reddit_get_subreddit

**What it does**: Gets details about a specific subreddit including name, description, subscriber count, and rules.

**When to use**: Learn about a community before participating, understand community guidelines.

**Arguments**:
- `subreddit` (required): Subreddit name (without r/, e.g., 'technology')

**Example LLM prompt**: "Get info about the programming subreddit"

---

### reddit_list_posts

**What it does**: Lists posts in a subreddit. Returns the most recent or popular posts depending on the sort parameter.

**When to use**: Browse content in a community, find trending posts.

**Arguments**:
- `subreddit` (required): Subreddit name
- `sort` (optional): Sort order: hot, new, top, rising (default hot)
- `limit` (optional): Number of posts to return (max 100, default 25)
- `after` (optional): Fullname of the item after which to fetch results

**Example LLM prompt**: "List the top posts this week in askscience"

---

### reddit_get_post

**What it does**: Gets details of a specific post including title, body, author, score, and metadata.

**When to use**: Read the full content before commenting or analyzing.

**Arguments**:
- `subreddit` (required): Subreddit name
- `post_id` (required): Post ID (the alphanumeric code in the URL)

**Example LLM prompt**: "Get details for post abc123 in the technology subreddit"

---

### reddit_create_post

**What it does**: Creates a new post in a subreddit. Choose between a link post or a text post with a title.

**When to use**: Share content, ask questions, start discussions.

**Arguments**:
- `subreddit` (required): Subreddit name (without r/)
- `title` (required): Post title
- `kind` (optional): Post type: link or self (default self)
- `url` (optional): URL for link posts
- `text` (optional): Text content for self posts
- `resubmit` (optional): Allow resubmission if URL was posted before (default true)

**Example LLM prompt**: "Create a text post in test subreddit with title 'Hello World' and body 'This is my first post'"

---

### reddit_list_comments

**What it does**: Lists comments on a post. Returns the comment tree with nested replies.

**When to use**: Read discussions and community feedback.

**Arguments**:
- `subreddit` (required): Subreddit name
- `post_id` (required): Post ID
- `limit` (optional): Maximum number of comments to return (max 100, default 50)
- `depth` (optional): Comment depth limit (1-8, default 8)

**Example LLM prompt**: "List comments on post abc123 in the technology subreddit"

---

### reddit_get_comment

**What it does**: Gets details of a specific comment including body, author, score, and parent information.

**When to use**: Read a specific comment in a thread.

**Arguments**:
- `comment_id` (required): Comment ID

**Example LLM prompt**: "Get details for comment xyz789"

---

### reddit_create_comment

**What it does**: Creates a new comment on a post or reply to an existing comment.

**When to use**: Participate in discussions or respond to others.

**Arguments**:
- `parent` (required): Fullname of the thing being replied to (post or comment, e.g., t3_abc123)
- `text` (required): Comment text

**Example LLM prompt**: "Reply to post t3_abc123 with 'Great question!'"

---

### reddit_get_user_info

**What it does**: Gets a user's profile information including karma, cake day, and public stats.

**When to use**: Learn about someone before trusting their content.

**Arguments**:
- `username` (required): Reddit username (without u/)

**Example LLM prompt**: "Get profile info for user reddit_username"

---

### reddit_search_posts

**What it does**: Searches for posts by keyword across Reddit. Returns matching posts sorted by relevance or recency.

**When to use**: Find discussions, answers, or specific content.

**Arguments**:
- `q` (required): Search query
- `subreddit` (optional): Limit search to specific subreddit
- `sort` (optional): Sort by: relevance, hot, new, top (default relevance)
- `limit` (optional): Number of results (max 100, default 25)

**Example LLM prompt**: "Search for posts about 'machine learning' in the technology subreddit"

---

## Reddit API Notes

- **Rate Limits**: 60 requests per minute for authenticated users
- **Fullnames**: IDs must include type prefix (t3_ for posts, t1_ for comments)
- **Karma**: Post and comment scores affect user's karma
- **NSFW**: Some content is marked as mature
- **Mod Tools**: Moderation tools require additional permissions
