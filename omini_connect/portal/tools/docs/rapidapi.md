# Rapidapi Tools

Provider: `rapidapi` | Engine: `nango` | Auth: OAuth via Nango

## Overview

RapidAPI is an API marketplace that allows developers to discover, connect, and manage APIs. These tools allow AI agents to browse the API marketplace, manage subscriptions, and view usage analytics.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with RapidAPI
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `marketplace:read`, `subscriptions:read`, `subscriptions:write`, `usage:read`, `developer:read`, `analytics:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `rapidapi_list_apis` | List available APIs | GET | /marketplace/apis |
| `rapidapi_get_api` | Get API details | GET | /marketplace/apis/{apiId} |
| `rapidapi_list_subscriptions` | List API subscriptions | GET | /subscriptions |
| `rapidapi_subscribe_api` | Subscribe to an API | POST | /subscriptions |
| `rapidapi_unsubscribe_api` | Unsubscribe from an API | DELETE | /subscriptions/{subscriptionId} |
| `rapidapi_list_endpoints` | List API endpoints | GET | /apis/{apiId}/endpoints |
| `rapidapi_get_usage` | Get API usage statistics | GET | /usage |
| `rapidapi_list_my_apis` | List your published APIs | GET | /my/apis |
| `rapidapi_get_endpoint_usage` | Get endpoint usage stats | GET | /usage/endpoint/{endpointId} |
| `rapidapi_list_analytics` | Get analytics data | GET | /analytics |

---

## Tool Details

### rapidapi_list_apis

**What it does**: Returns a list of APIs available on RapidAPI marketplace.

**When to use**: Discover APIs for integration.

**Arguments**:
- `category` (optional): Filter by category
- `limit` (optional): Number of APIs (default 20)

**Example LLM prompt**: "List available AI APIs on RapidAPI"

---

### rapidapi_get_api

**What it does**: Gets details of a specific API.

**When to use**: Get API documentation, pricing, and subscription info.

**Arguments**:
- `apiId` (required): The API ID

**Example LLM prompt**: "Get details for the OpenAI API on RapidAPI"

---

### rapidapi_list_subscriptions

**What it does**: Returns your active API subscriptions.

**When to use**: View your subscribed APIs and their status.

**Arguments**:
- `limit` (optional): Number of subscriptions (default 50)

**Example LLM prompt**: "List all my API subscriptions"

---

### rapidapi_subscribe_api

**What it does**: Subscribes you to a specific API.

**When to use**: Get access to an API's endpoints.

**Arguments**:
- `apiId` (required): The API ID
- `plan` (optional): Subscription plan

**Example LLM prompt**: "Subscribe to the Weather API"

---

### rapidapi_unsubscribe_api

**What it does**: Cancels your subscription to an API.

**When to use**: Stop using an API and stop billing.

**Arguments**:
- `subscriptionId` (required): The subscription ID to cancel

**Example LLM prompt**: "Cancel my subscription to API123"

---

### rapidapi_list_endpoints

**What it does**: Lists all endpoints for a subscribed API.

**When to use**: See what API calls are available.

**Arguments**:
- `apiId` (required): The API ID

**Example LLM prompt**: "List all endpoints for the Weather API"

---

### rapidapi_get_usage

**What it does**: Returns your overall API usage statistics.

**When to use**: Monitor API consumption and stay within limits.

**Arguments**:
- `period` (optional): Time period (daily, monthly)

**Example LLM prompt**: "Get my monthly API usage"

---

### rapidapi_list_my_apis

**What it does**: Lists APIs you have published.

**When to use**: Manage your own APIs on RapidAPI.

**Arguments**:
- `limit` (optional): Number of APIs (default 20)

**Example LLM prompt**: "List my published APIs"

---

### rapidapi_get_endpoint_usage

**What it does**: Gets usage statistics for a specific endpoint.

**When to use**: Analyze which endpoints are most used.

**Arguments**:
- `endpointId` (required): The endpoint ID
- `period` (optional): Time period

**Example LLM prompt**: "Get usage for endpoint ep_123"

---

### rapidapi_list_analytics

**What it does**: Returns analytics data for your APIs.

**When to use**: Track API performance and adoption.

**Arguments**:
- `apiId` (optional): Filter by API
- `period` (optional): Time period (default 30d)

**Example LLM prompt**: "Get analytics for my APIs"

---

## RapidAPI Notes

- RapidAPI is an API marketplace - you can consume and publish APIs
- Each API subscription has its own usage limits and billing
- Analytics help track API performance and usage patterns
- API IDs and endpoint IDs are unique identifiers on the platform
