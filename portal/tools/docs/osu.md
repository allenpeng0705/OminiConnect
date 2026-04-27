# Osu Tools

Provider: `osu` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Osu! gaming platform API. They allow AI agents to browse user profiles, beatmaps, scores, and multiplayer matches. **Requires Osu! OAuth authentication.**

## Authentication

**OAuth2**:
- User authenticates via Nango Connect with Osu!
- Authorization URL: `https://osu.ppy.sh/oauth/authorize`
- Token URL: `https://osu.ppy.sh/oauth/token`
- Default scope: `identify`
- Base URL: `https://osu.ppy.sh`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `osu_get_me` | Get current user | GET | /me |
| `osu_get_user` | Get user details | GET | /users/{user_id} |
| `osu_list_beatmapsets` | List beatmapsets | GET | /beatmapsets |
| `osu_get_beatmapset` | Get beatmapset details | GET | /beatmapsets/{beatmapset_id} |
| `osu_list_scores` | List scores | GET | /beatmaps/{beatmap_id}/scores |
| `osu_get_score` | Get score details | GET | /scores/{score_id} |
| `osu_list_matches` | List matches | GET | /matches |
| `osu_get_match` | Get match details | GET | /matches/{match_id} |
| `osu_list_user_scores` | List user scores | GET | /users/{user_id}/scores |
| `osu_get_recent_activity` | Get recent activity | GET | /users/{user_id}/recent_activity |

---

## Tool Details

### osu_get_me

**What it does**: Gets current authenticated user's profile.

**When to use**: Get your own profile, verify authentication.

**Arguments**: None

**Example LLM prompt**: "Get my Osu profile"

---

### osu_get_user

**What it does**: Gets detailed information for a specific user.

**When to use**: View player stats, rankings, profile.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get stats for user 1234567"

---

### osu_list_beatmapsets

**What it does**: Lists beatmapsets (song packages).

**When to use**: Browse available songs, find maps to play.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List recent ranked beatmapsets"

---

### osu_get_beatmapset

**What it does**: Gets detailed information for a specific beatmapset.

**When to use**: View song details, difficulty ratings.

**Arguments**:
- `beatmapset_id` (required): Beatmapset ID

**Example LLM prompt**: "Get details for beatmapset 12345"

---

### osu_list_scores

**What it does**: Lists scores on a beatmap.

**When to use**: View leaderboard, compare scores.

**Arguments**:
- `beatmap_id` (required): Beatmap ID

**Example LLM prompt**: "List top scores on beatmap 1234"

---

### osu_get_score

**What it does**: Gets detailed information for a specific score.

**When to use**: View score details, replay info.

**Arguments**:
- `score_id` (required): Score ID

**Example LLM prompt**: "Get details for score 9876543"

---

### osu_list_matches

**What it does**: Lists recent multiplayer matches.

**When to use**: Find games, view recent play.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List recent multiplayer matches"

---

### osu_get_match

**What it does**: Gets detailed information for a specific match.

**When to use**: View match results, player scores.

**Arguments**:
- `match_id` (required): Match ID

**Example LLM prompt**: "Get details for match 123456"

---

### osu_list_user_scores

**What it does**: Lists scores for a specific user.

**When to use**: View user's play history, best performances.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "List scores for user 1234567"

---

### osu_get_recent_activity

**What it does**: Gets recent activity for the authenticated user.

**When to use**: View recent plays, matches.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get recent activity for user 1234567"

---

## Osu Notes

- **Gaming platform**: Osu! is a rhythm game platform
- **Beatmaps**: Song/difficulty packages
- **Scores**: Player performance on beatmaps
- **Matches**: Multiplayer game sessions
- **Rankings**: Global and country leaderboards
