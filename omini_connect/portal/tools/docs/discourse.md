# Discourse

Community forum platform for discussions, categories, users, and groups.

## Authentication

Discourse uses cookie-based authentication. The Nango integration handles session management automatically.

## API Endpoints

Discourse API base URL: `https://{your-discourse-instance.com}`

## Tools

### Categories

#### `discourse_list_categories`
List all categories in the forum.

**Endpoint:** `GET /categories.json`

**Scopes:** `read`

#### `discourse_get_category`
Get details of a specific category.

**Endpoint:** `GET /c/{id}/show.json`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | integer | The category ID |

### Posts

#### `discourse_list_posts`
List posts with optional filtering.

**Endpoint:** `GET /posts.json`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| topic_id | integer | Filter by topic ID |
| category | string | Filter by category slug |
| page | integer | Page number for pagination |

#### `discourse_get_post`
Get a specific post by ID.

**Endpoint:** `GET /posts/{id}.json`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | integer | The post ID |

#### `discourse_create_post`
Create a new post or topic.

**Endpoint:** `POST /posts.json`

**Scopes:** `write`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| topic_id | integer | The topic ID to post in |
| title | string | Title for new topic |
| raw | string | The post content |
| category | string | Category slug for new topic |

### Users

#### `discourse_list_users`
List users in the forum.

**Endpoint:** `GET /admin/users/list/{flag}.json`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| flag | string | User type: active, new, staff, suspended |
| filter | string | Search filter for username or email |
| page | integer | Page number |

#### `discourse_get_user`
Get details of a specific user.

**Endpoint:** `GET /u/{username}.json`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| username | string | The username |

### Groups

#### `discourse_list_groups`
List all groups in the forum.

**Endpoint:** `GET /groups.json`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| type | string | Filter by type: public, private, automatic |

#### `discourse_get_group`
Get details of a specific group.

**Endpoint:** `GET /groups/{id}.json`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | integer | The group ID |

### Notifications

#### `discourse_list_notifications`
List notifications for the authenticated user.

**Endpoint:** `GET /notifications.json`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| limit | integer | Maximum notifications to return (default: 50) |
| offset | integer | Offset for pagination |

## Rate Limits

Discourse implements rate limiting. The API allows approximately 1000 requests per minute for read operations and 200 requests per minute for write operations.

## Common Use Cases

1. **Community Management**: Monitor new users, manage groups, and track activity
2. **Content Aggregation**: Pull posts from specific categories for external dashboards
3. **User Engagement**: Track notifications and user participation metrics
