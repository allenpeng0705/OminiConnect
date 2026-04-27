# Google Analytics Tools

Provider: `google-analytics` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Analytics Data API. They allow AI agents to run reports, view properties, dimensions, metrics, and realtime data. **Requires Google OAuth2 with Analytics permissions.**

## Authentication

**Nango OAUTH2 (Google Analytics)**:
- User authenticates via OAuth2 with Analytics scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://analyticsdata.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_analytics_list_reports` | Run Analytics reports | POST | /v1beta/properties/{property_id}:runReport |
| `google_analytics_get_property` | Get property details | GET | /v1beta/properties/{property_id} |
| `google_analytics_list_properties` | List Analytics properties | GET | /v1beta/accountSummaries |
| `google_analytics_list_dimensions` | List available dimensions | GET | /v1beta/properties/{property_id}/dimensions |
| `google_analytics_list_metrics` | List available metrics | GET | /v1beta/properties/{property_id}/metrics |
| `google_analytics_run_realtime_report` | Run realtime report | POST | /v1beta/properties/{property_id}:runRealtimeReport |
| `google_analytics_list_goals` | List conversion goals | GET | /v1beta/properties/{property_id}/conversionGoals |
| `google_analytics_list_user_link` | List user links | GET | /v1beta/properties/{property_id}/userLinks |
| `google_analytics_get_audience` | Get audience insights | POST | /v1beta/properties/{property_id}:runReport |
| `google_analytics_list_event` | List events | GET | /v1beta/properties/{property_id}/events |

---

## Tool Details

### google_analytics_list_reports

**What it does**: Runs a report query on Google Analytics data.

**When to use**: Get analytics data for a date range.

**Arguments**:
- `property_id` (required): Analytics property ID
- `start_date` (required): Start date (YYYY-MM-DD)
- `end_date` (required): End date (YYYY-MM-DD)
- `metrics` (optional): Metrics to retrieve
- `dimensions` (optional): Dimensions to group by

**Example LLM prompt**: "Run a report for property 123 from 2024-01-01 to 2024-01-31"

---

### google_analytics_get_property

**What it does**: Gets detailed information about an Analytics property.

**When to use**: Get property settings.

**Arguments**:
- `property_id` (required): Analytics property ID

**Example LLM prompt**: "Get details for property 123"

---

### google_analytics_list_properties

**What it does**: Lists all Analytics properties.

**When to use**: Browse available properties.

**Arguments**: None

**Example LLM prompt**: "List all my Analytics properties"

---

### google_analytics_list_dimensions

**What it does**: Lists available dimensions in Analytics.

**When to use**: See available grouping options.

**Arguments**:
- `property_id` (required): Analytics property ID

**Example LLM prompt**: "List available dimensions for property 123"

---

### google_analytics_list_metrics

**What it does**: Lists available metrics in Analytics.

**When to use**: See available metric values.

**Arguments**:
- `property_id` (required): Analytics property ID

**Example LLM prompt**: "List available metrics for property 123"

---

### google_analytics_run_realtime_report

**What it does**: Runs a realtime report on current activity.

**When to use**: See current user activity.

**Arguments**:
- `property_id` (required): Analytics property ID
- `metrics` (optional): Metrics to retrieve
- `dimensions` (optional): Dimensions to group by

**Example LLM prompt**: "Run realtime report for property 123"

---

### google_analytics_list_goals

**What it does**: Lists conversion goals for a property.

**When to use**: See tracking goals.

**Arguments**:
- `property_id` (required): Analytics property ID

**Example LLM prompt**: "List conversion goals for property 123"

---

### google_analytics_list_user_link

**What it does**: Lists user links for a property.

**When to use**: See who has access.

**Arguments**:
- `property_id` (required): Analytics property ID

**Example LLM prompt**: "List user links for property 123"

---

### google_analytics_get_audience

**What it does**: Gets audience insights for a property.

**When to use**: Analyze audience segments.

**Arguments**:
- `property_id` (required): Analytics property ID
- `audience_id` (optional): Audience ID

**Example LLM prompt**: "Get audience insights for property 123"

---

### google_analytics_list_event

**What it does**: Lists events for a property.

**When to use**: See event data.

**Arguments**:
- `property_id` (required): Analytics property ID
- `date` (optional): Date (YYYY-MM-DD)

**Example LLM prompt**: "List events for property 123 on 2024-01-15"

---

## Google Analytics API Notes

- **Property ID**: Format is "properties/123456789"
- **Dimensions**: Group data (country, device, page)
- **Metrics**: Quantitative measures (sessions, users, bounce rate)
- **Date format**: YYYY-MM-DD
- **GA4**: This is the GA4 (Google Analytics 4) API
