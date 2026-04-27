# Lattice Integration

Lattice is a performance management platform that helps organizations manage goals, reviews, and employee development.

## Authentication

Lattice uses OAuth 2.0 for authentication. Configure the following scopes in your Lattice OAuth application:

- `goals:read` - Read access to goals
- `goals:write` - Write access to goals
- `reviews:read` - Read access to reviews
- `reviews:write` - Write access to reviews
- `users:read` - Read access to users
- `feed:read` - Read access to activity feeds

## Base URL

```
https://api.lattice.com/v1
```

## Rate Limits

- Standard rate limit: 1000 requests per minute
- Write operations: 100 requests per minute

## Tools

### Goals

| Tool | Description |
|------|-------------|
| `lattice_list_goals` | Retrieve a list of all goals in the organization |
| `lattice_get_goal` | Retrieve details of a specific goal by ID |
| `lattice_create_goal` | Create a new goal in the organization |
| `lattice_update_goal` | Update an existing goal's details |
| `lattice_list_feeds` | Retrieve a list of activity feed items |

### Reviews

| Tool | Description |
|------|-------------|
| `lattice_list_reviews` | Retrieve a list of all performance reviews |
| `lattice_get_review` | Retrieve details of a specific review by ID |
| `lattice_create_review` | Create a new performance review |

### Users

| Tool | Description |
|------|-------------|
| `lattice_list_users` | Retrieve a list of all users in the organization |
| `lattice_get_user` | Retrieve details of a specific user by ID |

## Tool Details

### lattice_list_goals

Retrieve a list of all goals in the organization.

**Endpoint:** `GET /goals`

**Scopes:** `goals:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | string | No | Filter goals by status (e.g., active, completed) |
| `owner_id` | string | No | Filter goals by owner ID |

### lattice_get_goal

Retrieve details of a specific goal by ID.

**Endpoint:** `GET /goals/{goal_id}`

**Scopes:** `goals:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `goal_id` | string | Yes | The unique identifier of the goal |

### lattice_create_goal

Create a new goal in the organization.

**Endpoint:** `POST /goals`

**Scopes:** `goals:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | The name of the goal |
| `description` | string | No | The description of the goal |
| `owner_id` | string | No | The ID of the user who owns the goal |
| `due_date` | string | No | The due date for the goal (ISO 8601 format) |

### lattice_update_goal

Update an existing goal's details.

**Endpoint:** `PUT /goals/{goal_id}`

**Scopes:** `goals:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `goal_id` | string | Yes | The unique identifier of the goal |
| `name` | string | No | The updated name of the goal |
| `description` | string | No | The updated description |
| `status` | string | No | The updated status |
| `due_date` | string | No | The updated due date (ISO 8601 format) |

### lattice_list_reviews

Retrieve a list of all performance reviews.

**Endpoint:** `GET /reviews`

**Scopes:** `reviews:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | string | No | Filter reviews by status |
| `reviewer_id` | string | No | Filter reviews by reviewer ID |

### lattice_get_review

Retrieve details of a specific review by ID.

**Endpoint:** `GET /reviews/{review_id}`

**Scopes:** `reviews:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `review_id` | string | Yes | The unique identifier of the review |

### lattice_create_review

Create a new performance review.

**Endpoint:** `POST /reviews`

**Scopes:** `reviews:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `reviewee_id` | string | Yes | The ID of the user being reviewed |
| `reviewer_id` | string | Yes | The ID of the user conducting the review |
| `review_type` | string | No | The type of review (e.g., annual, quarterly) |
| `due_date` | string | No | The due date for completing the review |

### lattice_list_users

Retrieve a list of all users in the organization.

**Endpoint:** `GET /users`

**Scopes:** `users:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `role` | string | No | Filter users by role |
| `department` | string | No | Filter users by department |

### lattice_get_user

Retrieve details of a specific user by ID.

**Endpoint:** `GET /users/{user_id}`

**Scopes:** `users:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `user_id` | string | Yes | The unique identifier of the user |

### lattice_list_feeds

Retrieve a list of activity feed items.

**Endpoint:** `GET /feeds`

**Scopes:** `feed:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `feed_type` | string | No | Filter by feed type (e.g., updates, comments) |
| `user_id` | string | No | Filter feed items by user ID |
