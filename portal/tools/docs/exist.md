# Exist Tools

Provider: `exist` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Exist.io API. They allow AI agents to track habits, view attributes, and analyze personal data. Exist.io is a self-tracking platform for habit formation and life analysis.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Exist.io
- Token stored in Nango, accessed via `connection_ref`
- OAuth2 with refresh token support

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `exist_get_user` | Get current user | GET | /api/v1/user/ |
| `exist_list_attributes` | List all attributes | GET | /api/v1/attributes/ |
| `exist_get_attribute` | Get attribute data | GET | /api/v1/attributes/{name}/ |
| `exist_list_habits` | List all habits | GET | /api/v1/habits/ |
| `exist_get_habit` | Get habit data | GET | /api/v1/habits/{id}/ |
| `exist_list_tracks` | List tracked data | GET | /api/v1/tracks/ |
| `exist_get_track` | Get track data | GET | /api/v1/tracks/{id}/ |
| `exist_list_tags` | List all tags | GET | /api/v1/tags/ |
| `exist_get_insights` | Get insights | GET | /api/v1/insights/ |
| `exist_search` | Search for attributes | GET | /api/v1/search/ |

---

## Tool Details

### exist_get_user

**What it does**: Gets current authenticated user information.

**When to use**: Verify authentication, get user profile.

**Arguments**: None

**Example LLM prompt**: "Get my Exist.io profile"

---

### exist_list_attributes

**What it does**: Lists all available tracking attributes.

**When to use**: Browse available data types to track.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all available attributes I can track"

---

### exist_get_attribute

**What it does**: Gets data for a specific attribute over time.

**When to use**: View historical data for a tracked attribute.

**Arguments**:
- `name` (required): Attribute name
- `date_from` (optional): Start date (YYYY-MM-DD)
- `date_to` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get my sleep data for the past week"

---

### exist_list_habits

**What it does**: Lists all habits being tracked.

**When to use**: View current habits, see habit list.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all my habits"

---

### exist_get_habit

**What it does**: Gets data for a specific habit.

**When to use**: View habit progress, check streaks.

**Arguments**:
- `id` (required): Habit ID
- `date_from` (optional): Start date
- `date_to` (optional): End date

**Example LLM prompt**: "Get my meditation habit data for this month"

---

### exist_list_tracks

**What it does**: Lists all tracked data entries.

**When to use**: Browse raw tracking data.

**Arguments**:
- `attribute` (optional): Filter by attribute name
- `date_from` (optional): Start date
- `date_to` (optional): End date

**Example LLM prompt**: "List my tracked activities this week"

---

### exist_get_track

**What it does**: Gets a specific track entry.

**When to use**: Get details of a specific data point.

**Arguments**:
- `id` (required): Track ID

**Example LLM prompt**: "Get track entry 123"

---

### exist_list_tags

**What it does**: Lists all tags used for categorization.

**When to use**: See how data is organized.

**Arguments**: None

**Example LLM prompt**: "List all my tags"

---

### exist_get_insights

**What it does**: Gets insights and analysis of tracking data.

**When to use**: Get AI-generated insights about patterns.

**Arguments**:
- `date_from` (optional): Start date
- `date_to` (optional): End date

**Example LLM prompt**: "Get insights for the past month"

---

### exist_search

**What it does**: Searches for attributes by name.

**When to use**: Find specific attributes to track.

**Arguments**:
- `q` (required): Search query

**Example LLM prompt**: "Search for attributes related to exercise"

---

## Exist.io API Notes

- **Attributes**: Trackable data types (sleep, mood, steps, etc.)
- **Habits**: Specific habits built around attributes
- **Tracks**: Individual data entries
- **Tags**: Categorization for attributes
- **Insights**: Analysis of tracking patterns
- **Pagination**: Use page parameter for large result sets
