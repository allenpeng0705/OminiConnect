# Twitter V2 Tools

Provider: `twitter-v2` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Twitter API v2 with OAuth2 user context. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Twitter V2
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
| `twitter_get_followers` | Get user followers | GET | /2/users/{id}/followers |
| `twitter_get_following` | Get users followed by a user | GET | /2/users/{id}/following |
| `twitter_retweet` | Retweet a tweet | POST | /2/users/{id}/retweets |

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

### twitter_get_followers

**What it does**: Get user followers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_get_following

**What it does**: Get users followed by a user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitter_retweet

**What it does**: Retweet a tweet

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.twitter.com`
- Docs: https://nango.dev/docs/integrations/all/twitter-v2
