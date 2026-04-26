# Hotjar Tools

Provider: `hotjar` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Hotjar API. They allow AI agents to analyze user behavior through session recordings, heatmaps, conversion funnels, and surveys. Hotjar is a behavioral analytics tool that helps understand how users interact with your website.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Hotjar
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `hotjar.recordings.read`, `hotjar.heatmaps.read`, `hotjar.funnels.read`, `hotjar.surveys.read`, `hotjar.users.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `hotjar_list_recordings` | List session recordings | GET | /sites/{siteId}/recordings |
| `hotjar_get_recording` | Get recording details | GET | /sites/{siteId}/recordings/{recordingId} |
| `hotjar_list_heatmaps` | List heatmaps | GET | /sites/{siteId}/heatmaps |
| `hotjar_get_heatmap` | Get heatmap details | GET | /sites/{siteId}/heatmaps/{heatmapId} |
| `hotjar_list_funnels` | List conversion funnels | GET | /sites/{siteId}/funnels |
| `hotjar_get_funnel` | Get funnel details | GET | /sites/{siteId}/funnels/{funnelId} |
| `hotjar_list_surveys` | List surveys | GET | /surveys |
| `hotjar_get_survey` | Get survey details | GET | /surveys/{surveyId} |
| `hotjar_list_users` | List identified users | GET | /sites/{siteId}/users |
| `hotjar_get_user` | Get user details | GET | /sites/{siteId}/users/{userId} |

---

## Tool Details

### hotjar_list_recordings

**What it does**: Retrieves a list of session recordings with optional filtering and pagination.

**When to use**: Find session recordings to analyze user behavior patterns.

**Arguments**:
- `siteId` (required): Hotjar site ID
- `limit` (optional): Maximum number of recordings to return (default 50)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all session recordings for site 123"

---

### hotjar_get_recording

**What it does**: Retrieves details about a specific session recording.

**When to use**: Analyze an individual user's journey on your website.

**Arguments**:
- `siteId` (required): Hotjar site ID
- `recordingId` (required): Unique identifier of the recording

**Example LLM prompt**: "Get recording abc123 for site 123"

---

### hotjar_list_heatmaps

**What it does**: Retrieves a list of heatmaps for a site showing click and scroll patterns.

**When to use**: Find available heatmaps to understand user attention areas.

**Arguments**:
- `siteId` (required): Hotjar site ID
- `limit` (optional): Maximum number of heatmaps to return

**Example LLM prompt**: "List all heatmaps for site 123"

---

### hotjar_get_heatmap

**What it does**: Retrieves details about a specific heatmap including page URLs and data collection status.

**When to use**: Review heatmap configuration before analyzing user behavior.

**Arguments**:
- `siteId` (required): Hotjar site ID
- `heatmapId` (required): Unique identifier of the heatmap

**Example LLM prompt**: "Get heatmap 456 for site 123"

---

### hotjar_list_funnels

**What it does**: Retrieves a list of conversion funnels that track user journeys through predefined steps.

**When to use**: Find funnels to analyze where users drop off in conversion flows.

**Arguments**:
- `siteId` (required): Hotjar site ID

**Example LLM prompt**: "List all funnels for site 123"

---

### hotjar_get_funnel

**What it does**: Retrieves details about a specific funnel including conversion rates at each step.

**When to use**: Analyze where users abandon the conversion process.

**Arguments**:
- `siteId` (required): Hotjar site ID
- `funnelId` (required): Unique identifier of the funnel

**Example LLM prompt**: "Get funnel def456 for site 123"

---

### hotjar_list_surveys

**What it does**: Retrieves a list of surveys with optional filtering by status.

**When to use**: Browse available surveys to collect user feedback.

**Arguments**:
- `siteId` (required): Hotjar site ID
- `status` (optional): Filter by survey status (active, paused, closed)

**Example LLM prompt**: "List all active surveys"

---

### hotjar_get_survey

**What it does**: Retrieves details about a specific survey including questions and response counts.

**When to use**: Review survey configuration before analyzing results.

**Arguments**:
- `surveyId` (required): Unique identifier of the survey

**Example LLM prompt**: "Get details for survey 789"

---

### hotjar_list_users

**What it does**: Retrieves a list of identified users with their attributes and session history.

**When to use**: Find specific users to analyze their behavior across sessions.

**Arguments**:
- `siteId` (required): Hotjar site ID
- `limit` (optional): Maximum number of users to return

**Example LLM prompt**: "List identified users for site 123"

---

### hotjar_get_user

**What it does**: Retrieves details about a specific user including their session history and attributes.

**When to use**: Investigate a specific user's behavior across multiple sessions.

**Arguments**:
- `siteId` (required): Hotjar site ID
- `userId` (required): Unique identifier of the user

**Example LLM prompt**: "Get user ghi789 for site 123"

---

## Hotjar API Notes

- **Recordings**: Session recordings of individual user journeys on your website
- **Heatmaps**: Visual representation of where users click, scroll, and spend time
- **Funnels**: Conversion tracking showing drop-off points in user journeys
- **Surveys**: On-site feedback tools including polls and exit surveys
- **Users**: Identified users with consolidated behavior data across sessions
- **Site ID**: Required for most operations to scope data to a specific tracked website
- **Pagination**: Use limit and offset parameters for paginated results