# Twitter (OAuth2 CC) Tools

Provider: `twitter-oauth2-cc` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

Twitter API with OAuth2 Client Credentials flow for app-only access. **Requires oauth2 client credentials via nango.**

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client credentials flow for app-level access
- Token stored in Nango, accessed via `connection_ref`
- Scopes: tweet.read, tweet.write, users.read, offline.access

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `twitter_list_tweets` | List tweets from timeline | GET | /2/tweets |
| `twitter_get_tweet` | Get tweet details | GET | /2/tweets/{id} |
| `twitter_create_tweet` | Post a new tweet | POST | /2/tweets |
| `twitter_delete_tweet` | Delete a tweet | DELETE | /2/tweets/{id} |
| `twitter_list_users` | List Twitter users | GET | /2/users |
| `twitter_get_user` | Get user profile | GET | /2/users/{id} |
| `twitter_search_tweets` | Search for tweets | GET | /2/tweets/search/recent |
| `twitter_get_likes` | Get tweets liked by a user | GET | /2/users/{id}/liked_tweets |
| `twitter_like_tweet` | Like a tweet | POST | /2/users/{id}/likes |
| `twitter_unlike_tweet` | Unlike a tweet | DELETE | /2/users/{id}/likes/{tweet_id} |

---

## Tool Details

### twitter_list_tweets

**What it does**: List tweets from timeline

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_get_tweet

**What it does**: Get tweet details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_create_tweet

**What it does**: Post a new tweet

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_delete_tweet

**What it does**: Delete a tweet

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_list_users

**What it does**: List Twitter users

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_get_user

**What it does**: Get user profile

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_search_tweets

**What it does**: Search for tweets

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_get_likes

**What it does**: Get tweets liked by a user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_like_tweet

**What it does**: Like a tweet

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_unlike_tweet

**What it does**: Unlike a tweet

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.twitter.com`
- Docs: https://nango.dev/docs/integrations/all/twitter-oauth2-cc
