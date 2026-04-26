# Aha Tools

Provider: `aha` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Aha! API. They allow AI agents to manage ideas, features, releases, and users. Aha! is a product planning and roadmap platform for product managers.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Aha!
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `ideas:read`, `ideas:write`, `features:read`, `features:write`, `releases:read`, `releases:write`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `aha_list_ideas` | List ideas | GET | /api/v1/ideas |
| `aha_get_idea` | Get idea details | GET | /api/v1/ideas/{id} |
| `aha_create_idea` | Create a new idea | POST | /api/v1/ideas |
| `aha_list_features` | List features | GET | /api/v1/features |
| `aha_get_feature` | Get feature details | GET | /api/v1/features/{id} |
| `aha_create_feature` | Create a new feature | POST | /api/v1/features |
| `aha_list_releases` | List releases | GET | /api/v1/releases |
| `aha_get_release` | Get release details | GET | /api/v1/releases/{id} |
| `aha_list_users` | List users | GET | /api/v1/users |
| `aha_get_user` | Get user details | GET | /api/v1/users/{id} |

---

## Tool Details

### aha_list_ideas

**What it does**: Lists all ideas with optional filtering by product, status, or keyword.

**When to use**: Browse customer feedback, find ideas for feature prioritization.

**Arguments**:
- `product_id` (optional): Filter by product ID
- `status` (optional): Filter by status
- `keyword` (optional): Search keyword
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all ideas for product abc-123"

---

### aha_get_idea

**What it does**: Gets detailed information about a specific idea including title, description, status, and votes.

**When to use**: Understand idea details, see customer feedback context.

**Arguments**:
- `id` (required): Idea ID

**Example LLM prompt**: "Get details for idea xyz-456"

---

### aha_create_idea

**What it does**: Creates a new idea with title, description, and product association.

**When to use**: Capture customer feedback, log feature requests.

**Arguments**:
- `title` (required): Idea title
- `description` (optional): Idea description
- `product_id` (required): Product ID
- `status` (optional): Initial status

**Example LLM prompt**: "Create an idea titled 'Dark mode support' for product abc-123"

---

### aha_list_features

**What it does**: Lists all features with optional filtering by product, release, status, or keyword.

**When to use**: Browse features, find features for a release, search by keyword.

**Arguments**:
- `product_id` (optional): Filter by product ID
- `release_id` (optional): Filter by release ID
- `status` (optional): Filter by status
- `keyword` (optional): Search keyword
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all features in release xyz-456"

---

### aha_get_feature

**What it does**: Gets detailed information about a specific feature including name, description, status, and requirements.

**When to use**: Understand feature details, see implementation requirements.

**Arguments**:
- `id` (required): Feature ID

**Example LLM prompt**: "Get details for feature def-789"

---

### aha_create_feature

**What it does**: Creates a new feature with name, product association, and description.

**When to use**: Add new features to the roadmap, capture user requests.

**Arguments**:
- `name` (required): Feature name
- `product_id` (required): Product ID
- `release_id` (optional): Associated release ID
- `description` (optional): Feature description
- `status` (optional): Initial status

**Example LLM prompt**: "Create a feature named User Dashboard for product abc-123"

---

### aha_list_releases

**What it does**: Lists all releases with optional filtering by product, status, or date range.

**When to use**: View roadmap, find releases at a certain stage, see upcoming releases.

**Arguments**:
- `product_id` (optional): Filter by product ID
- `status` (optional): Filter by status
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all releases for product abc-123"

---

### aha_get_release

**What it does**: Gets detailed information about a specific release including name, status, dates, and associated features.

**When to use**: Understand release scope, see what features are planned.

**Arguments**:
- `id` (required): Release ID

**Example LLM prompt**: "Get details for release xyz-456"

---

### aha_list_users

**What it does**: Lists all users with optional filtering by product, role, or activity status.

**When to use**: Find team members, see who has access to a product.

**Arguments**:
- `product_id` (optional): Filter by product ID
- `role` (optional): Filter by role
- `active` (optional): Filter by active status
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all users for product abc-123"

---

### aha_get_user

**What it does**: Gets detailed information about a specific user including name, email, role, and permissions.

**When to use**: Check user details, verify permissions.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user uuu-999"

---

## Aha! API Notes

- **Ideas**: Customer feedback and suggestions that can be converted into features. Support title, description, status, and voting.
- **Features**: Product capabilities that are planned or implemented. Can be assigned to releases and have requirements.
- **Releases**: Time-based milestones that group features. Can have target dates and release phases.
- **Users**: Team members with roles and permissions. Can be assigned to features and ideas.
- **Statuses**: Feature and release statuses track progress (concept, planning, in development, shipped, etc.)
- **Hierarchy**: Ideas -> Features -> Releases -> Products