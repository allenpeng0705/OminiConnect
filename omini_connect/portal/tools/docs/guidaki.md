# Guidaki Tools

Provider: `guidaki` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Guidaki REST API. They allow AI agents to manage tours, checklists, users, analytics, and flows — Guidaki's user onboarding platform. Guidaki helps organizations create interactive product tours and onboarding checklists.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Guidaki
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `tours:read`, `tours:write`, `checklists:read`, `users:read`, `analytics:read`, `flows:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `guidaki_list_tours` | List tours | GET | /api/v1/tours |
| `guidaki_get_tour` | Get tour details | GET | /api/v1/tours/{tourId} |
| `guidaki_create_tour` | Create a tour | POST | /api/v1/tours |
| `guidaki_list_checklists` | List checklists | GET | /api/v1/checklists |
| `guidaki_get_checklist` | Get checklist details | GET | /api/v1/checklists/{checklistId} |
| `guidaki_list_users` | List users | GET | /api/v1/users |
| `guidaki_get_user` | Get user details | GET | /api/v1/users/{userId} |
| `guidaki_get_analytics` | Get analytics data | GET | /api/v1/analytics |
| `guidaki_list_flows` | List flows | GET | /api/v1/flows |
| `guidaki_get_flow` | Get flow details | GET | /api/v1/flows/{flowId} |

---

## Tool Details

### guidaki_list_tours

**What it does**: Lists all onboarding tours in your Guidaki account.

**When to use**: Browse available tours to understand user onboarding content.

**Arguments**:
- `status` (optional): Filter by status (active, archived, draft)
- `page_size` (optional): Number of results (default 50, max 200)

**Example LLM prompt**: "What tours are available in Guidaki?"

---

### guidaki_get_tour

**What it does**: Gets detailed information about a specific tour.

**When to use**: Understand tour content, steps, and targeting.

**Arguments**:
- `tourId` (required): Tour ID

**Example LLM prompt**: "Get details for tour ABC123"

---

### guidaki_create_tour

**What it does**: Creates a new onboarding tour in Guidaki.

**When to use**: Set up new user onboarding content for product education.

**Arguments**:
- `name` (required): Tour name
- `description` (optional): Tour description
- `steps` (optional): Array of step objects
- `status` (optional): Tour status (draft, published)

**Example LLM prompt**: "Create a tour called 'Getting Started' for new users"

---

### guidaki_list_checklists

**What it does**: Lists all checklists in your Guidaki account.

**When to use**: Browse available checklists for user onboarding tasks.

**Arguments**:
- `page_size` (optional): Number of results (default 50, max 200)

**Example LLM prompt**: "What checklists are available?"

---

### guidaki_get_checklist

**What it does**: Gets detailed information about a specific checklist.

**When to use**: Understand checklist items and completion requirements.

**Arguments**:
- `checklistId` (required): Checklist ID

**Example LLM prompt**: "Get details for checklist XYZ789"

---

### guidaki_list_users

**What it does**: Lists all users in your Guidaki account.

**When to use**: Browse user accounts and their onboarding progress.

**Arguments**:
- `page_size` (optional): Number of results (default 50, max 200)

**Example LLM prompt**: "List users in our Guidaki account"

---

### guidaki_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Understand user profile and their tour/checklist completion status.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get user details for user@example.com"

---

### guidaki_get_analytics

**What it does**: Gets analytics data for tours and user onboarding progress.

**When to use**: Measure onboarding effectiveness and identify drop-off points.

**Arguments**:
- `date_from` (optional): Start date (YYYY-MM-DD)
- `date_to` (optional): End date (YYYY-MM-DD)
- `tour_id` (optional): Filter by tour ID

**Example LLM prompt**: "Show analytics for the last 30 days"

---

### guidaki_list_flows

**What it does**: Lists all flows in your Guidaki account.

**When to use**: Browse available flows for user journey automation.

**Arguments**:
- `page_size` (optional): Number of results (default 50, max 200)

**Example LLM prompt**: "What flows are available in Guidaki?"

---

### guidaki_get_flow

**What it does**: Gets detailed information about a specific flow.

**When to use**: Understand flow triggers, conditions, and actions.

**Arguments**:
- `flowId` (required): Flow ID

**Example LLM prompt**: "Get details for flow ABC123"

---

## Guidaki API Notes

- **Tours**: Interactive step-by-step walkthroughs for product education
- **Checklists**: Task lists users complete during onboarding
- **Users**: End users who interact with tours and checklists
- **Analytics**: Usage metrics for tours, completions, and user progress
- **Flows**: Automation sequences triggered by user actions
