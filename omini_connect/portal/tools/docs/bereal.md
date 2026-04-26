# BeReal Tools

Provider: `bereal` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the BeReal API. They allow AI agents to view and post moments, manage friends, react to friends' content, check notifications, and retrieve user profiles. BeReal is a mobile-first social platform focused on spontaneous, authentic photo sharing.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with BeReal
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `moments:read`, `moments:write`, `friends:read`, `friends:write`, `notifications:read`, `user:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bereal_list_moments` | List BeReal moments | GET | /v1/moments |
| `bereal_get_moment` | Get moment details | GET | /v1/moments/{moment_id} |
| `bereal_post_moment` | Post a new moment | POST | /v1/moments |
| `bereal_list_friends` | List friends | GET | /v1/friends |
| `bereal_get_friend` | Get friend details | GET | /v1/friends/{user_id} |
| `bereal_add_friend` | Send a friend request | POST | /v1/friends |
| `bereal_react_to_moment` | React to a moment | POST | /v1/moments/{moment_id}/reactions |
| `bereal_list_notifications` | List notifications | GET | /v1/notifications |
| `bereal_get_user_profile` | Get user profile | GET | /v1/users/me |

---

## Tool Details

### bereal_list_moments

**What it does**: Lists BeReal moments from the authenticated user or their friends. Returns photos, captions, location, and reaction counts for each moment.

**When to use**: Browse your friends' recent moments, view your own moment history, or check a specific user's moments.

**Arguments**:
- `user_id` (optional): Filter moments by user ID (omit for friends feed)
- `feed_type` (optional): `friends` or `user` (default: friends)
- `page` (optional): Page number (default: 1)
- `limit` (optional): Number of results, max 20 (default: 10)

**Example LLM prompt**: "Show me my friends' recent moments"

---

### bereal_get_moment

**What it does**: Gets details of a specific BeReal moment including photos, caption, location, and reactions.

**When to use**: View a specific moment in full detail, check reactions, see location information.

**Arguments**:
- `moment_id` (required): Moment ID

**Example LLM prompt**: "Get details for moment 12345"

---

### bereal_post_moment

**What it does**: Posts a new BeReal moment with front and back camera photos. Includes optional caption and location data.

**When to use**: Share an authentic moment with friends, post a late BeReal if you missed the notification window.

**Arguments**:
- `front_photo` (required): Base64 encoded front camera photo
- `back_photo` (required): Base64 encoded back camera photo
- `caption` (optional): Moment caption (max 150 characters)
- `latitude` (optional): Location latitude
- `longitude` (optional): Location longitude
- `is_late` (optional): Mark as late BeReal (default: false)

**Example LLM prompt**: "Post a new moment with the front and back photos I just took"

---

### bereal_list_friends

**What it does**: Lists all friends of the authenticated user. Returns friend profiles with their current status and last moment time.

**When to use**: View your friends list, see who among your friends has posted recently, check pending friend requests.

**Arguments**:
- `status` (optional): `accepted`, `pending`, `incoming`, or `outgoing`
- `page` (optional): Page number (default: 1)
- `limit` (optional): Number of results (default: 20)

**Example LLM prompt**: "Show me all my friends who have posted recently"

---

### bereal_get_friend

**What it does**: Gets a specific friend's profile including their BeReal stats and last posted moment.

**When to use**: Check if a friend has posted recently, view their profile and statistics.

**Arguments**:
- `user_id` (required): User ID of the friend

**Example LLM prompt**: "Get profile for friend 67890"

---

### bereal_add_friend

**What it does**: Sends a friend request to another BeReal user. Use this to connect with friends and see their moments.

**When to use**: Add new friends, respond to incoming friend requests.

**Arguments**:
- `user_id` (required): User ID to send friend request to
- `username` (optional): Or username of the user

**Example LLM prompt**: "Send a friend request to user 67890"

---

### bereal_react_to_moment

**What it does**: Adds a reaction emoji to a friend's BeReal moment. Reactions appear alongside the moment for all friends to see.

**When to use**: Respond to friends' moments with fun emoji reactions.

**Arguments**:
- `moment_id` (required): Moment ID to react to
- `emoji` (required): Reaction emoji: `haha`, `wow`, `sad`, `angry`, `love`, `fire`, `clap`, or `100`

**Example LLM prompt**: "React with fire emoji to moment 12345"

---

### bereal_list_notifications

**What it does**: Lists notifications for the authenticated user including friend requests, reactions, and comments.

**When to use**: Stay updated on interactions with your moments and profile.

**Arguments**:
- `type` (optional): `friend_request`, `reaction`, `comment`, `mention`, or `all` (default: all)
- `unread_only` (optional): Only show unread notifications (default: false)
- `page` (optional): Page number (default: 1)
- `limit` (optional): Number of results (default: 20)

**Example LLM prompt**: "Show me all my unread notifications"

---

### bereal_get_user_profile

**What it does**: Gets the authenticated user's profile including username, display name, bio, avatar, and statistics.

**When to use**: View or manage your own profile information, check your account stats.

**Arguments**: None

**Example LLM prompt**: "Show me my BeReal profile"

---

## BeReal API Notes

- **Authenticity**: BeReal emphasizes real, unfiltered moments over polished content
- **Dual Camera**: Moments always include both front and back camera photos
- **Spontaneity**: BeReal notifications prompt users to post within a time window
- **Late BeReal**: Users can post late, but it's marked as such to friends
- **Friends-only**: Content is primarily shared with friends, not public
- **Reactions**: Emoji reactions are the primary way to engage with friends' moments
- **Location**: Moments often include location data (when permissions allow)
