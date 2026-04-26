# WalkMe Tools

Provider: `walkme` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the WalkMe REST API. They allow AI agents to manage guides, steps, users, analytics, and campaigns — WalkMe's digital adoption platform for user guidance. WalkMe helps organizations improve user onboarding and product adoption.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with WalkMe
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `guides:read`, `guides:write`, `users:read`, `analytics:read`, `campaigns:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `walkme_list_guides` | List guides | GET | /api/v1/guides |
| `walkme_get_guide` | Get guide details | GET | /api/v1/guides/{guideId} |
| `walkme_create_guide` | Create a guide | POST | /api/v1/guides |
| `walkme_list_steps` | List steps in guide | GET | /api/v1/guides/{guideId}/steps |
| `walkme_get_step` | Get step details | GET | /api/v1/steps/{stepId} |
| `walkme_list_users` | List users | GET | /api/v1/users |
| `walkme_get_user` | Get user details | GET | /api/v1/users/{userId} |
| `walkme_get_analytics` | Get analytics data | GET | /api/v1/analytics |
| `walkme_list_campaigns` | List campaigns | GET | /api/v1/campaigns |
| `walkme_get_campaign` | Get campaign details | GET | /api/v1/campaigns/{campaignId} |

---

## Tool Details

### walkme_list_guides

**What it does**: Lists all guides in your WalkMe system.

**When to use**: Browse available guides to understand user guidance content.

**Arguments**:
- `status` (optional): Filter by status (active, archived, draft)
- `category_id` (optional): Filter by category ID

**Example LLM prompt**: "What guides are available in WalkMe?"

---

### walkme_get_guide

**What it does**: Gets detailed information about a specific guide.

**When to use**: Understand guide content, steps, and targeting rules.

**Arguments**:
- `guideId` (required): Guide ID

**Example LLM prompt**: "Get details for guide ABC123"

---

### walkme_create_guide

**What it does**: Creates a new guide in WalkMe.

**When to use**: Set up new user guidance content for onboarding or feature adoption.

**Arguments**:
- `title` (required): Guide title
- `description` (optional): Guide description
- `category_id` (optional): Category ID
- `status` (optional): Guide status (draft, published)

**Example LLM prompt**: "Create a guide called 'Getting Started' for new users"

---

### walkme_list_steps

**What it does**: Lists all steps in a guide.

**When to use**: Understand the flow of a multi-step guide.

**Arguments**:
- `guideId` (required): Guide ID

**Example LLM prompt**: "What steps are in guide ABC123?"

---

### walkme_get_step

**What it does**: Gets detailed information about a specific step.

**When to use**: Understand step content, targeting, and behavior.

**Arguments**:
- `stepId` (required): Step ID

**Example LLM prompt**: "Get details for step XYZ789"

---

### walkme_list_users

**What it does**: Lists all users in your WalkMe system.

**When to use**: Browse user accounts and their guide progress.

**Arguments**:
- `page_size` (optional): Number of results (default 50, max 200)

**Example LLM prompt**: "List users in our WalkMe account"

---

### walkme_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Understand user profile and their guide completion status.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get user details for user@example.com"

---

### walkme_get_analytics

**What it does**: Gets analytics data for guides and user interactions.

**When to use**: Measure guide effectiveness and user engagement.

**Arguments**:
- `date_from` (optional): Start date (YYYY-MM-DD)
- `date_to` (optional): End date (YYYY-MM-DD)
- `guide_id` (optional): Filter by guide ID

**Example LLM prompt**: "Show analytics for the last 30 days"

---

### walkme_list_campaigns

**What it does**: Lists all campaigns in your WalkMe system.

**When to use**: Browse campaigns to understand how guides are promoted to users.

**Arguments**:
- `status` (optional): Filter by status (active, archived, draft)

**Example LLM prompt**: "What active campaigns are running?"

---

### walkme_get_campaign

**What it does**: Gets detailed information about a specific campaign.

**When to use**: Understand campaign targeting and performance.

**Arguments**:
- `campaignId` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign 12345"

---

## WalkMe API Notes

- **Guides**: Multi-step walkthroughs that guide users through tasks
- **Steps**: Individual steps within a guide (tooltip, modal, hotspot)
- **Users**: End users who interact with guides
- **Analytics**: Usage metrics for guides, completions, drop-offs
- **Campaigns**: Mechanisms to target users with specific guides
