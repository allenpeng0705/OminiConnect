# Productboard Tools

Provider: `productboard` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Productboard API. They allow AI agents to manage features, insights, users, and portals. Productboard is a product planning platform that helps teams gather and prioritize customer feedback.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Productboard
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `features:read`, `features:write`, `insights:read`, `insights:write`, `users:read`, `portals:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `productboard_list_features` | List features | GET | /api/v1/features |
| `productboard_get_feature` | Get feature details | GET | /api/v1/features/{id} |
| `productboard_create_feature` | Create a new feature | POST | /api/v1/features |
| `productboard_list_insights` | List insights | GET | /api/v1/insights |
| `productboard_get_insight` | Get insight details | GET | /api/v1/insights/{id} |
| `productboard_create_insight` | Create a new insight | POST | /api/v1/insights |
| `productboard_list_users` | List users | GET | /api/v1/users |
| `productboard_get_user` | Get user details | GET | /api/v1/users/{id} |
| `productboard_list_portals` | List portals | GET | /api/v1/portals |
| `productboard_get_portal` | Get portal details | GET | /api/v1/portals/{id} |

---

## Tool Details

### productboard_list_features

**What it does**: Lists all features with optional filtering by product, status, or keyword.

**When to use**: Browse features, find features for planning, search by keyword.

**Arguments**:
- `product_id` (optional): Filter by product ID
- `status` (optional): Filter by status
- `keyword` (optional): Search keyword
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all features for product abc-123"

---

### productboard_get_feature

**What it does**: Gets detailed information about a specific feature including name, description, status, and associated insights.

**When to use**: Understand feature details, see related customer feedback.

**Arguments**:
- `id` (required): Feature ID

**Example LLM prompt**: "Get details for feature xyz-456"

---

### productboard_create_feature

**What it does**: Creates a new feature with name, product association, and description.

**When to use**: Add new features to the planning backlog, capture new ideas.

**Arguments**:
- `name` (required): Feature name
- `product_id` (required): Product ID
- `description` (optional): Feature description
- `feature_type` (optional): Feature type (enhancement, bug, etc.)
- `status` (optional): Initial status

**Example LLM prompt**: "Create a feature named 'User Dashboard' for product abc-123"

---

### productboard_list_insights

**What it does**: Lists all insights with optional filtering by product, source, or status.

**When to use**: Browse customer feedback, find insights for prioritization.

**Arguments**:
- `product_id` (optional): Filter by product ID
- `source` (optional): Filter by source (feedback, survey, etc.)
- `status` (optional): Filter by status
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all insights for product abc-123"

---

### productboard_get_insight

**What it does**: Gets detailed information about a specific insight including title, description, source, and related features.

**When to use**: Understand customer feedback in detail, see what features address this insight.

**Arguments**:
- `id` (required): Insight ID

**Example LLM prompt**: "Get details for insight def-789"

---

### productboard_create_insight

**What it does**: Creates a new insight with title, description, source, and product association.

**When to use**: Log customer feedback, capture survey results.

**Arguments**:
- `title` (required): Insight title
- `description` (optional): Insight description
- `product_id` (required): Product ID
- `source` (optional): Source of insight

**Example LLM prompt**: "Create an insight titled 'Customers want dark mode' for product abc-123"

---

### productboard_list_users

**What it does**: Lists all users with optional filtering by product, role, or status.

**When to use**: Find team members, see who has access to a product.

**Arguments**:
- `product_id` (optional): Filter by product ID
- `role` (optional): Filter by role
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all users for product abc-123"

---

### productboard_get_user

**What it does**: Gets detailed information about a specific user including name, email, and role.

**When to use**: Check user details, verify permissions.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user uuu-999"

---

### productboard_list_portals

**What it does**: Lists all portals with optional filtering by product or status.

**When to use**: View available portals, find public product boards.

**Arguments**:
- `product_id` (optional): Filter by product ID
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all portals for product abc-123"

---

### productboard_get_portal

**What it does**: Gets detailed information about a specific portal including name, settings, and visibility.

**When to use**: Understand portal configuration, check public access settings.

**Arguments**:
- `id` (required): Portal ID

**Example LLM prompt**: "Get details for portal ppp-111"

---

## Productboard API Notes

- **Features**: Product capabilities that are planned or implemented. Linked to insights and can have custom types.
- **Insights**: Customer feedback and requests. Can come from various sources and are linked to features.
- **Users**: Team members with roles and permissions. Can be assigned to features and insights.
- **Portals**: Public or private product boards where stakeholders can view feature status and provide feedback.
- **Products**: The top-level organizational unit that contains features, insights, and users.
- **Feature Types**: Custom categories like enhancement, bug, or custom types defined by the team.