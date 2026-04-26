# Freshmarketer Tools

Provider: `freshmarketer` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Freshmarketer REST API. They allow AI agents to manage visitors, events, funnels, campaigns, and analytics — Freshmarketer's marketing automation capabilities. Freshmarketer helps track website visitors and optimize conversion funnels.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Freshmarketer
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `visitors:read`, `events:read`, `funnels:read`, `reports:read`, `analytics:read`, `campaigns:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `freshmarketer_list_visitors` | List visitors | GET | /api/v1/visitors |
| `freshmarketer_get_visitor` | Get visitor details | GET | /api/v1/visitors/{visitorId} |
| `freshmarketer_list_events` | List events | GET | /api/v1/events |
| `freshmarketer_get_event` | Get event details | GET | /api/v1/events/{eventId} |
| `freshmarketer_list_funnels` | List funnels | GET | /api/v1/funnels |
| `freshmarketer_get_funnel` | Get funnel details | GET | /api/v1/funnels/{funnelId} |
| `freshmarketer_get_conversion_report` | Get conversion report | GET | /api/v1/reports/conversion |
| `freshmarketer_list_page_analytics` | List page analytics | GET | /api/v1/analytics/pages |
| `freshmarketer_get_page_analytics` | Get page analytics | GET | /api/v1/analytics/pages/{pageId} |
| `freshmarketer_list_campaigns` | List campaigns | GET | /api/v1/campaigns |

---

## Tool Details

### freshmarketer_list_visitors

**What it does**: Lists all website visitors in your Freshmarketer account.

**When to use**: Browse visitor data to understand website traffic.

**Arguments**:
- `page_size` (optional): Number of results (default 50, max 200)
- `date_from` (optional): Filter from date (YYYY-MM-DD)
- `date_to` (optional): Filter to date (YYYY-MM-DD)

**Example LLM prompt**: "Who have been the recent visitors to our website?"

---

### freshmarketer_get_visitor

**What it does**: Gets detailed information about a specific visitor.

**When to use**: Understand individual visitor behavior and attributes.

**Arguments**:
- `visitorId` (required): Visitor ID

**Example LLM prompt**: "Get details for visitor ABC123"

---

### freshmarketer_list_events

**What it does**: Lists all tracked events in your Freshmarketer account.

**When to use**: Browse available event types being tracked.

**Arguments**:
- `page_size` (optional): Number of results (default 50, max 200)
- `event_type` (optional): Filter by event type

**Example LLM prompt**: "What events are being tracked?"

---

### freshmarketer_get_event

**What it does**: Gets detailed information about a specific event.

**When to use**: Understand event structure and properties.

**Arguments**:
- `eventId` (required): Event ID

**Example LLM prompt**: "Get details for event XYZ789"

---

### freshmarketer_list_funnels

**What it does**: Lists all conversion funnels in your Freshmarketer account.

**When to use**: Browse available funnels to track conversion paths.

**Arguments**:
- `page_size` (optional): Number of results (default 50, max 200)

**Example LLM prompt**: "What conversion funnels are set up?"

---

### freshmarketer_get_funnel

**What it does**: Gets detailed information about a specific funnel.

**When to use**: Understand funnel steps and conversion rates.

**Arguments**:
- `funnelId` (required): Funnel ID

**Example LLM prompt**: "Get details for funnel ABC123"

---

### freshmarketer_get_conversion_report

**What it does**: Gets conversion report data for your marketing campaigns.

**When to use**: Measure campaign effectiveness and ROI.

**Arguments**:
- `date_from` (optional): Start date (YYYY-MM-DD)
- `date_to` (optional): End date (YYYY-MM-DD)
- `campaign_id` (optional): Filter by campaign ID

**Example LLM prompt**: "Show conversion report for the last 30 days"

---

### freshmarketer_list_page_analytics

**What it does**: Lists page analytics data for your website.

**When to use**: Understand which pages perform best.

**Arguments**:
- `date_from` (optional): Start date (YYYY-MM-DD)
- `date_to` (optional): End date (YYYY-MM-DD)
- `page_size` (optional): Number of results (default 50, max 200)

**Example LLM prompt**: "Show page analytics for the homepage"

---

### freshmarketer_get_page_analytics

**What it does**: Gets detailed analytics for a specific page.

**When to use**: Deep dive into page-level metrics and user behavior.

**Arguments**:
- `pageId` (required): Page ID or URL identifier
- `date_from` (optional): Start date (YYYY-MM-DD)
- `date_to` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get detailed analytics for the pricing page"

---

### freshmarketer_list_campaigns

**What it does**: Lists all marketing campaigns in your Freshmarketer account.

**When to use**: Browse active campaigns and their status.

**Arguments**:
- `status` (optional): Filter by status (active, paused, completed)
- `page_size` (optional): Number of results (default 50, max 200)

**Example LLM prompt**: "What active campaigns are running?"

---

## Freshmarketer API Notes

- **Visitors**: Website visitors tracked via cookies or forms
- **Events**: User actions on the website (page views, clicks, form submissions)
- **Funnels**: Multi-step conversion paths to track user journeys
- **Page Analytics**: Metrics for individual pages (views, bounce rate, time on page)
- **Campaigns**: Marketing initiatives to track effectiveness
