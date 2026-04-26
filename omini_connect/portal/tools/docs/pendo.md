# Pendo Tools

Provider: `pendo` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Pendo REST API. They allow AI agents to manage features, events, pages, guides, and users — Pendo's core product analytics building blocks. Pendo helps product teams understand how users interact with their products.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Pendo
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `features:read`, `events:read`, `pages:read`, `guides:read`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pendo_list_features` | List features | GET | /api/v1/features |
| `pendo_get_feature` | Get feature details | GET | /api/v1/features/{featureId} |
| `pendo_list_events` | List events | GET | /api/v1/events |
| `pendo_get_event` | Get event details | GET | /api/v1/events/{eventId} |
| `pendo_list_pages` | List pages | GET | /api/v1/pages |
| `pendo_get_page` | Get page details | GET | /api/v1/pages/{pageId} |
| `pendo_list_guides` | List guides | GET | /api/v1/guides |
| `pendo_get_guide` | Get guide details | GET | /api/v1/guides/{guideId} |
| `pendo_list_users` | List users | GET | /api/v1/users |
| `pendo_get_user` | Get user details | GET | /api/v1/users/{userId} |

---

## Tool Details

### pendo_list_features

**What it does**: Lists all features in your Pendo product.

**When to use**: Browse available features to understand product functionality.

**Arguments**:
- `productId` (optional): Filter by product ID

**Example LLM prompt**: "What features are tracked in our Pendo account?"

---

### pendo_get_feature

**What it does**: Gets detailed information about a specific feature.

**When to use**: Understand feature usage and configuration.

**Arguments**:
- `featureId` (required): Feature ID

**Example LLM prompt**: "Get details for feature ABC123"

---

### pendo_list_events

**What it does**: Lists all events tracked in your Pendo product.

**When to use**: Browse tracked user events for analytics.

**Arguments**:
- `productId` (optional): Filter by product ID
- `pageSize` (optional): Number of results (default 50, max 200)

**Example LLM prompt**: "What events are being tracked?"

---

### pendo_get_event

**What it does**: Gets detailed information about a specific event.

**When to use**: Understand event structure and properties.

**Arguments**:
- `eventId` (required): Event ID

**Example LLM prompt**: "Get details for event XYZ789"

---

### pendo_list_pages

**What it does**: Lists all pages in your Pendo product.

**When to use**: See what pages are being tracked.

**Arguments**:
- `productId` (optional): Filter by product ID

**Example LLM prompt**: "What pages are tracked in Pendo?"

---

### pendo_get_page

**What it does**: Gets detailed information about a specific page.

**When to use**: Understand page analytics and configuration.

**Arguments**:
- `pageId` (required): Page ID

**Example LLM prompt**: "Get details for page 12345"

---

### pendo_list_guides

**What it does**: Lists all guides in your Pendo product.

**When to use**: Browse available in-app guides.

**Arguments**:
- `productId` (optional): Filter by product ID
- `status` (optional): Filter by status (active, archived, draft)

**Example LLM prompt**: "List all active guides in Pendo"

---

### pendo_get_guide

**What it does**: Gets detailed information about a specific guide.

**When to use**: Understand guide content and targeting.

**Arguments**:
- `guideId` (required): Guide ID

**Example LLM prompt**: "Get details for guide 67890"

---

### pendo_list_users

**What it does**: Lists all users in your Pendo product.

**When to use**: Browse user accounts and attributes.

**Arguments**:
- `productId` (optional): Filter by product ID
- `pageSize` (optional): Number of results (default 50, max 200)

**Example LLM prompt**: "List users in our Pendo account"

---

### pendo_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Understand user profile and activity.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get user details for user@example.com"

---

## Pendo API Notes

- **Features**: Trackable UI elements (buttons, menus, pages)
- **Events**: User actions tracked within the product
- **Pages**: Application pages being tracked
- **Guides**: In-app walkthroughs and announcements
- **Users**: End users of your product being tracked
