# Lucky Orange Tools

Provider: `luckyorange` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Lucky Orange API. They allow AI agents to monitor website visitors in real-time, analyze session recordings, view heatmaps, manage live chats, and track conversion funnels. Lucky Orange is a web analytics platform focused on real-time visitor insights.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Lucky Orange
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `luckyorange.recordings.read`, `luckyorange.heatmaps.read`, `luckyorange.funnels.read`, `luckyorange.chats.read`, `luckyorange.visits.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `luckyorange_list_recordings` | List session recordings | GET | /sites/{siteId}/recordings |
| `luckyorange_get_recording` | Get recording details | GET | /sites/{siteId}/recordings/{recordingId} |
| `luckyorange_list_heatmaps` | List heatmaps | GET | /sites/{siteId}/heatmaps |
| `luckyorange_get_heatmap` | Get heatmap details | GET | /sites/{siteId}/heatmaps/{heatmapId} |
| `luckyorange_list_chats` | List live chat conversations | GET | /sites/{siteId}/chats |
| `luckyorange_get_chat` | Get chat details | GET | /sites/{siteId}/chats/{chatId} |
| `luckyorange_list_funnels` | List conversion funnels | GET | /sites/{siteId}/funnels |
| `luckyorange_get_funnel` | Get funnel details | GET | /sites/{siteId}/funnels/{funnelId} |
| `luckyorange_list_visits` | List website visits | GET | /sites/{siteId}/visits |
| `luckyorange_get_visit` | Get visit details | GET | /sites/{siteId}/visits/{visitId} |

---

## Tool Details

### luckyorange_list_recordings

**What it does**: Retrieves a list of session recordings with filtering and pagination options.

**When to use**: Find session recordings to watch actual user sessions.

**Arguments**:
- `siteId` (required): Lucky Orange site ID
- `limit` (optional): Maximum number of recordings to return (default 50)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all recordings for site abc123"

---

### luckyorange_get_recording

**What it does**: Retrieves details about a specific session recording including duration, pages visited, and events.

**When to use**: Watch a specific user session to understand their behavior.

**Arguments**:
- `siteId` (required): Lucky Orange site ID
- `recordingId` (required): Unique identifier of the recording

**Example LLM prompt**: "Get recording rec123 for site abc123"

---

### luckyorange_list_heatmaps

**What it does**: Retrieves a list of heatmaps showing where users click and scroll on your pages.

**When to use**: Find available heatmaps to visualize user attention patterns.

**Arguments**:
- `siteId` (required): Lucky Orange site ID
- `limit` (optional): Maximum number of heatmaps to return

**Example LLM prompt**: "List all heatmaps for site abc123"

---

### luckyorange_get_heatmap

**What it does**: Retrieves details about a specific heatmap including page URLs and data collection status.

**When to use**: Review heatmap configuration before analyzing user engagement.

**Arguments**:
- `siteId` (required): Lucky Orange site ID
- `heatmapId` (required): Unique identifier of the heatmap

**Example LLM prompt**: "Get heatmap heat123 for site abc123"

---

### luckyorange_list_chats

**What it does**: Retrieves a list of live chat conversations with visitors.

**When to use**: Find and review conversations between visitors and your team.

**Arguments**:
- `siteId` (required): Lucky Orange site ID
- `status` (optional): Filter by chat status (open, closed)

**Example LLM prompt**: "List all open chats for site abc123"

---

### luckyorange_get_chat

**What it does**: Retrieves details about a specific chat conversation including messages and metadata.

**When to use**: Review a specific customer interaction.

**Arguments**:
- `siteId` (required): Lucky Orange site ID
- `chatId` (required): Unique identifier of the chat

**Example LLM prompt**: "Get chat chat456 for site abc123"

---

### luckyorange_list_funnels

**What it does**: Retrieves a list of conversion funnels that track user journeys through key steps.

**When to use**: Find funnels to analyze where visitors drop off.

**Arguments**:
- `siteId` (required): Lucky Orange site ID

**Example LLM prompt**: "List all funnels for site abc123"

---

### luckyorange_get_funnel

**What it does**: Retrieves details about a specific funnel including conversion metrics at each step.

**When to use**: Analyze the effectiveness of your conversion paths.

**Arguments**:
- `siteId` (required): Lucky Orange site ID
- `funnelId` (required): Unique identifier of the funnel

**Example LLM prompt**: "Get funnel def789 for site abc123"

---

### luckyorange_list_visits

**What it does**: Retrieves a list of website visits with visitor information including location and device.

**When to use**: Browse recent visitors to your site, see who's currently online.

**Arguments**:
- `siteId` (required): Lucky Orange site ID
- `limit` (optional): Maximum number of visits to return

**Example LLM prompt**: "List recent visits for site abc123"

---

### luckyorange_get_visit

**What it does**: Retrieves details about a specific visit including pages viewed and duration.

**When to use**: Investigate a specific visitor's session on your website.

**Arguments**:
- `siteId` (required): Lucky Orange site ID
- `visitId` (required): Unique identifier of the visit

**Example LLM prompt**: "Get visit vis123 for site abc123"

---

## Lucky Orange API Notes

- **Recordings**: Session recordings of individual user journeys on your website
- **Heatmaps**: Visual representation of user clicks, scrolls, and mouse movement
- **Chats**: Live chat conversations between visitors and your support team
- **Funnels**: Conversion tracking showing drop-off points in user journeys
- **Visits**: Individual browsing sessions with visitor context (device, location, referrer)
- **Site ID**: Required for all operations to scope data to a specific tracked website
- **Real-time**: Lucky Orange provides real-time visitor data and live chat capabilities
- **Pagination**: Use limit and offset parameters for paginated results