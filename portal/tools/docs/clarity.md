# Clarity Tools

Provider: `clarity` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Microsoft Clarity API. They allow AI agents to analyze user sessions, view heatmaps, track events, manage users, and analyze A/B testing contrasts. Microsoft Clarity is a free behavioral analytics tool that provides session recordings and heatmaps.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Microsoft Clarity
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `clarity.sessions.read`, `clarity.events.read`, `clarity.heatmaps.read`, `clarity.users.read`, `clarity.contrasts.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `clarity_list_sessions` | List user sessions | GET | /v1/sessions |
| `clarity_get_session` | Get session details | GET | /v1/sessions/{sessionId} |
| `clarity_list_events` | List user events | GET | /v1/events |
| `clarity_get_event` | Get event details | GET | /v1/events/{eventId} |
| `clarity_list_heatmaps` | List heatmaps | GET | /v1/heatmaps |
| `clarity_get_heatmap` | Get heatmap details | GET | /v1/heatmaps/{heatmapId} |
| `clarity_list_users` | List identified users | GET | /v1/users |
| `clarity_get_user` | Get user details | GET | /v1/users/{userId} |
| `clarity_list_contrasts` | List A/B testing contrasts | GET | /v1/contrasts |
| `clarity_get_contrast` | Get contrast details | GET | /v1/contrasts/{contrastId} |

---

## Tool Details

### clarity_list_sessions

**What it does**: Retrieves a list of user sessions with filtering and pagination options.

**When to use**: Browse recent user sessions to analyze behavior patterns.

**Arguments**:
- `projectId` (required): Microsoft Clarity project ID
- `limit` (optional): Maximum number of sessions to return (default 50)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List recent sessions for project abc123"

---

### clarity_get_session

**What it does**: Retrieves details about a specific session including pages viewed, events, and user actions.

**When to use**: Analyze an individual user's journey on your website.

**Arguments**:
- `sessionId` (required): Unique identifier of the session
- `projectId` (required): Microsoft Clarity project ID

**Example LLM prompt**: "Get session sess456 for project abc123"

---

### clarity_list_events

**What it does**: Retrieves a list of user interaction events with optional session filtering.

**When to use**: Find specific user actions tracked in your application.

**Arguments**:
- `projectId` (required): Microsoft Clarity project ID
- `sessionId` (optional): Filter events by session ID
- `limit` (optional): Maximum number of events to return

**Example LLM prompt**: "List events for project abc123"

---

### clarity_get_event

**What it does**: Retrieves details about a specific event including properties and occurrence context.

**When to use**: Analyze a specific user action and its attributes.

**Arguments**:
- `eventId` (required): Unique identifier of the event
- `projectId` (required): Microsoft Clarity project ID

**Example LLM prompt**: "Get event evt789 for project abc123"

---

### clarity_list_heatmaps

**What it does**: Retrieves a list of heatmaps showing user click and scroll patterns.

**When to use**: Find available heatmaps to visualize user attention.

**Arguments**:
- `projectId` (required): Microsoft Clarity project ID
- `limit` (optional): Maximum number of heatmaps to return

**Example LLM prompt**: "List all heatmaps for project abc123"

---

### clarity_get_heatmap

**What it does**: Retrieves details about a specific heatmap including page URLs and data status.

**When to use**: Review heatmap configuration before analyzing user engagement.

**Arguments**:
- `heatmapId` (required): Unique identifier of the heatmap
- `projectId` (required): Microsoft Clarity project ID

**Example LLM prompt**: "Get heatmap heat123 for project abc123"

---

### clarity_list_users

**What it does**: Retrieves a list of identified users with their attributes and behavior data.

**When to use**: Find specific users to analyze their behavior across sessions.

**Arguments**:
- `projectId` (required): Microsoft Clarity project ID
- `limit` (optional): Maximum number of users to return

**Example LLM prompt**: "List identified users for project abc123"

---

### clarity_get_user

**What it does**: Retrieves details about a specific user including their session history and attributes.

**When to use**: Investigate a specific user's behavior across multiple sessions.

**Arguments**:
- `userId` (required): Unique identifier of the user
- `projectId` (required): Microsoft Clarity project ID

**Example LLM prompt**: "Get user usr456 for project abc123"

---

### clarity_list_contrasts

**What it does**: Retrieves a list of A/B testing contrasts with optional status filtering.

**When to use**: Find active or completed experiments to analyze.

**Arguments**:
- `projectId` (required): Microsoft Clarity project ID
- `status` (optional): Filter by contrast status (active, completed)

**Example LLM prompt**: "List all active contrasts for project abc123"

---

### clarity_get_contrast

**What it does**: Retrieves details about a specific A/B test contrast including variant performance metrics.

**When to use**: Analyze the results of an A/B test experiment.

**Arguments**:
- `contrastId` (required): Unique identifier of the contrast
- `projectId` (required): Microsoft Clarity project ID

**Example LLM prompt**: "Get contrast exp789 for project abc123"

---

## Clarity API Notes

- **Sessions**: Individual user browsing sessions with full interaction data
- **Events**: User actions including clicks, page views, form submissions
- **Heatmaps**: Visual representation of where users click and scroll on pages
- **Users**: Identified users consolidated across multiple sessions
- **Contrasts**: A/B test experiments comparing different page variants
- **Project ID**: Required for most operations to scope data to a specific Clarity project
- **Free tool**: Microsoft Clarity is free to use with unlimited projects
- **Pagination**: Use limit and offset parameters for paginated results
- **Session filtering**: Events can be filtered by specific sessions