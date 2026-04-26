# Twitter/X Tools

Provider: `twitter` | Engine: `nango` | Auth: OAuth (via Nango) | Requires: Elevated or Pro access

## Overview

These tools wrap the Twitter API v2. They allow AI agents to interact with tweets, users, timelines, and search on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `tweet.read`, `tweet.write`, `users.read`, `follows.read`, etc.

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `twitter_list_tweets` | List tweets from a user's timeline | GET | /2/users/{id}/tweets |
| `twitter_get_tweet` | Get a specific tweet by ID | GET | /2/tweets/{id} |
| `twitter_post_tweet` | Post a new tweet | POST | /2/tweets |
| `twitter_delete_tweet` | Delete a tweet | DELETE | /2/tweets/{id} |
| `twitter_list_timelines` | List home timeline tweets | GET | /2/timelines/reverse_chronological |
| `twitter_search_tweets` | Search recent tweets | GET | /2/tweets/search/recent |
| `twitter_list_users` | List users by IDs | GET | /2/users |
| `twitter_get_user` | Get user profile | GET | /2/users/{id} |
| `twitter_get_followers` | Get user's followers | GET | /2/users/{id}/followers |
| `twitter_get_following` | Get users this account follows | GET | /2/users/{id}/following |

---

## Tool Details

### twitter_list_tweets

**What it does**: Lists tweets from a specific user's timeline with engagement metrics.

**When to use**: View recent tweets from a specific account to understand their activity or content.

**Arguments**:
- `id` (required): User ID
- `max_results` (optional): Max tweets to return (default 10, max 100)
- `start_time` (optional): Start time (ISO 8601)
- `end_time` (optional): End time (ISO 8601)

**Example LLM prompt**: "List the last 20 tweets from @TechCrunch"

---

### twitter_get_tweet

**What it does**: Get a specific tweet by ID including text, author info, metrics, and media.

**When to use**: Read the full content of a tweet, check engagement metrics, or get media attached to a tweet.

**Arguments**:
- `id` (required): Tweet ID
- `tweet_fields` (optional): Fields to include (created_at, public_metrics, etc.)

**Example LLM prompt**: "Show me tweet ID 1234567890"

---

### twitter_post_tweet

**What it does**: Posts a new tweet from the authenticated user. Supports text, URLs, and media references.

**When to use**: Share updates, respond to conversations, or post announcements.

**Arguments**:
- `text` (required): Tweet text (max 280 chars)
- `reply_to` (optional): Tweet ID to reply to

**Example LLM prompt**: "Post a tweet saying 'Just launched our new feature!'"

---

### twitter_delete_tweet

**What it does**: Deletes a tweet by ID. Only the authenticated user's tweets can be deleted.

**When to use**: Remove outdated, incorrect, or sensitive tweets.

**Arguments**:
- `id` (required): Tweet ID to delete

**Example LLM prompt**: "Delete my tweet with ID 1234567890"

---

### twitter_list_timelines

**What it does**: Lists tweets from the authenticated user's home timeline (tweets from followed accounts).

**When to use**: See what tweets are trending in your network or get a personalized feed.

**Arguments**:
- `max_results` (optional): Max tweets to return (default 10, max 100)
- `start_time` (optional): Start time (ISO 8601)
- `end_time` (optional): End time (ISO 8601)

**Example LLM prompt**: "Show me my Twitter home timeline"

---

### twitter_search_tweets

**What it does**: Search for tweets matching a query. Only returns tweets from the last 7 days. Supports Twitter's search operators.

**When to use**: Find tweets about a topic, brand mentions, or trending discussions.

**Arguments**:
- `query` (required): Search query with Twitter operators (from:, @:, #:, etc.)
- `max_results` (optional): Max results (default 10, max 100)
- `start_time` (optional): Start time (ISO 8601)
- `end_time` (optional): End time (ISO 8601)

**Example LLM prompt**: "Search for recent tweets about #AI from the last 24 hours"

---

### twitter_list_users

**What it does**: List Twitter users by their IDs or usernames. Returns user objects with profile info.

**When to use**: Bulk lookup user information.

**Arguments**:
- `ids` (optional): Comma-separated list of user IDs (max 100)
- `usernames` (optional): Comma-separated list of usernames (max 100)

**Example LLM prompt**: "Get info for users with IDs 123 and 456"

---

### twitter_get_user

**What it does**: Get a Twitter user profile by user ID or username. Returns name, username, bio, follower count, etc.

**When to use**: Research a Twitter account before following, engaging, or analyzing.

**Arguments**:
- `id` (optional): User ID
- `username` (optional): Username (without @)

**Example LLM prompt**: "Get profile info for @elonmusk"

---

### twitter_get_followers

**What it does**: Get a user's followers. Returns user objects with follower counts.

**When to use**: Analyze an account's audience or find followers of a specific account.

**Arguments**:
- `id` (required): User ID
- `max_results` (optional): Max results (default 10, max 100)

**Example LLM prompt**: "List followers of @Twitter"

---

### twitter_get_following

**What it does**: Get accounts a user is following. Returns user objects.

**When to use**: See what accounts a user engages with or find similar accounts.

**Arguments**:
- `id` (required): User ID
- `max_results` (optional): Max results (default 10, max 100)

**Example LLM prompt**: "Show who @NASA follows on Twitter"

---

## Twitter API Reference

These tools use the Twitter API v2. See official docs for full details:
- https://developer.twitter.com/en/docs/twitter-api
- Rate limits: Vary by endpoint and access level (Free, Basic, Pro, Enterprise)
- Search: Only returns tweets from the last 7 days
- Pagination: Use `max_results` parameter
- All dates: ISO 8601 format

## Notes

- Twitter API v2 requires Elevated access or higher for most endpoints
- Some endpoints require specific permissions (tweet.write, like.write, etc.)
- Direct message endpoints require additional permissions
- Rate limits are per-app and per-user
