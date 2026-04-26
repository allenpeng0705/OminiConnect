# Anchor Tools

Provider: `anchor` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Anchor/Spotify for Podcasters API. They allow AI agents to manage podcasts, episodes, analytics, and distribution across podcast directories.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Anchor
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `podcasts-read`, `podcasts-write`, `episodes-read`, `episodes-write`, `analytics-read`, `distributions-read`, `distributions-write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `anchor_list_podcasts` | List user's podcasts | GET | /podcasts |
| `anchor_get_podcast` | Get podcast details | GET | /podcasts/{podcast_id} |
| `anchor_create_podcast` | Create a podcast | POST | /podcasts |
| `anchor_list_episodes` | List podcast episodes | GET | /podcasts/{podcast_id}/episodes |
| `anchor_get_episode` | Get episode details | GET | /episodes/{episode_id} |
| `anchor_create_episode` | Create an episode | POST | /episodes |
| `anchor_get_analytics` | Get podcast analytics | GET | /analytics/{podcast_id} |
| `anchor_list_distributions` | List distribution status | GET | /distributions |
| `anchor_distribute_episode` | Distribute an episode | POST | /episodes/{episode_id}/distribute |
| `anchor_get_distribution_status` | Get distribution status | GET | /episodes/{episode_id}/distribution |

---

## Tool Details

### anchor_list_podcasts

**What it does**: Returns a paginated list of all podcasts owned by the authenticated user with title, description, and episode count.

**When to use**: Browse podcast portfolio, manage multiple shows, or select podcast for operations.

**Arguments**:
- `limit` (optional): Max podcasts (default 50, max 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all my podcasts on Anchor"

---

### anchor_get_podcast

**What it does**: Gets detailed information about a specific podcast including metadata, artwork, and settings.

**When to use**: Get podcast configuration, view analytics overview, or manage settings.

**Arguments**:
- `podcast_id` (required): Podcast ID

**Example LLM prompt**: "Get details for podcast xyz789"

---

### anchor_create_podcast

**What it does**: Creates a new podcast show with title and optional description.

**When to use**: Start a new podcast, create show configuration.

**Arguments**:
- `title` (required): Podcast title
- `description` (optional): Podcast description
- `language` (optional): Language code (default en)
- `category` (optional): Podcast category

**Example LLM prompt**: "Create a new podcast called 'Tech Talk'"

---

### anchor_list_episodes

**What it does**: Returns a paginated list of episodes for a specific podcast with title, description, publish date, and duration.

**When to use**: Browse podcast episodes, manage content library, or track publication history.

**Arguments**:
- `podcast_id` (required): Podcast ID
- `limit` (optional): Max episodes (default 50, max 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all episodes for podcast xyz789"

---

### anchor_get_episode

**What it does**: Gets detailed information about a specific episode including title, description, audio URL, and metadata.

**When to use**: Get full episode details before editing, promoting, or analyzing performance.

**Arguments**:
- `episode_id` (required): Episode ID

**Example LLM prompt**: "Get details for episode abc123"

---

### anchor_create_episode

**What it does**: Creates a new episode for a podcast with title, description, and audio.

**When to use**: Publish new podcast episodes, schedule future releases, or create drafts.

**Arguments**:
- `podcast_id` (required): Podcast ID
- `title` (required): Episode title
- `description` (optional): Episode description
- `audio_url` (optional): URL to audio file
- `scheduled_publish_time` (optional): ISO 8601 publish time

**Example LLM prompt**: "Create a new episode titled 'Episode 42: Interview with Guest' for podcast xyz789"

---

### anchor_get_analytics

**What it does**: Gets analytics data for a podcast including listen counts, audience demographics, and engagement metrics.

**When to use**: Track podcast performance, analyze listener engagement, or generate reports.

**Arguments**:
- `podcast_id` (required): Podcast ID
- `start_date` (optional): Start date (ISO 8601)
- `end_date` (optional): End date (ISO 8601)

**Example LLM prompt**: "Show analytics for podcast xyz789 for the last 30 days"

---

### anchor_list_distributions

**What it does**: Lists distribution destinations configured for a podcast. Returns supported platforms and their current status.

**When to use**: Check which platforms are enabled, review distribution configuration.

**Arguments**:
- `podcast_id` (optional): Podcast ID

**Example LLM prompt**: "List all distribution platforms for my podcast"

---

### anchor_distribute_episode

**What it does**: Triggers distribution of an episode to all configured podcast directories and platforms.

**When to use**: Publish an episode to Spotify, Apple Podcasts, and other platforms.

**Arguments**:
- `episode_id` (required): Episode ID

**Example LLM prompt**: "Distribute episode abc123 to all platforms"

---

### anchor_get_distribution_status

**What it does**: Gets the distribution status for an episode across all platforms. Returns per-platform status and any errors.

**When to use**: Check if episode was successfully published to all platforms, troubleshoot distribution issues.

**Arguments**:
- `episode_id` (required): Episode ID

**Example LLM prompt**: "Check distribution status for episode abc123"

---

## Anchor API Notes

- **Podcasts**: Anchor organizes content under podcasts (shows)
- **Episodes**: Episodes belong to a podcast and have publish status
- **Distribution**: Episodes can be distributed to Spotify, Apple Podcasts, Google Podcasts, and more
- **Analytics**: Provides listen counts, audience demographics, and engagement metrics
- **Scheduling**: Supports scheduling episodes for future publication