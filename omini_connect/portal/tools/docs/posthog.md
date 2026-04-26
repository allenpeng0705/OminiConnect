# PostHog Tool Registry

This file defines the available tools for the PostHog integration in OminiConnect.

## Authentication

- **Auth Type**: Personal API Key
- **Scope**: `ph_api_full_access` - Full access to PostHog data

## Tools

### Analytics

- `posthog_list_events` - List events from PostHog
- `posthog_get_event` - Get a specific event by ID
- `posthog_track_event` - Track a custom event

### Person Management

- `posthog_list_persons` - List persons in PostHog
- `posthog_get_person` - Get a specific person by ID

### Trends

- `posthog_list_trends` - List trend calculations
- `posthog_get_trend` - Get a specific trend by ID

### Funnels

- `posthog_list_funnels` - List funnel analyses
- `posthog_get_funnel` - Get a specific funnel by ID

### Cohorts

- `posthog_list_cohorts` - List cohorts
- `posthog_get_cohort` - Get a specific cohort by ID

## Rate Limits

- API requests limited to 600 requests per minute on standard plan
- Rate limits vary by plan tier