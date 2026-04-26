# Pinterest Tools

Provider: `pinterest` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Pinterest API. They allow AI agents to manage boards, pins, users, and advertising campaigns on Pinterest. Pinterest is a visual discovery platform for saving and discovering ideas.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Pinterest
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `pins:read`, `pins:write`, `boards:read`, `boards:write`, `user_accounts:read`, `ads:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pinterest_list_pins` | List pins for authenticated user | GET | /v5/pins |
| `pinterest_get_pin` | Get a specific pin by ID | GET | /v5/pins/{pin_id} |
| `pinterest_create_pin` | Create a new pin on a board | POST | /v5/pins |
| `pinterest_list_boards` | List boards for authenticated user | GET | /v5/boards |
| `pinterest_get_board` | Get a specific board by ID | GET | /v5/boards/{board_id} |
| `pinterest_create_board` | Create a new board | POST | /v5/boards |
| `pinterest_list_user_pins` | List pins on a specific board | GET | /v5/pins |
| `pinterest_get_user_info` | Get user profile information | GET | /v5/users/me |
| `pinterest_search_pins` | Search pins by keyword | GET | /v5/pins/search |
| `pinterest_list_ads` | List advertising campaigns | GET | /v5/ad_accounts/{ad_account_id}/ads |

---

## Tool Details

### pinterest_list_pins

**What it does**: Lists pins for the authenticated user. Returns a paginated list of pins the user has created or saved.

**When to use**: Browse your pin collection, find specific content, or search through your saved ideas.

**Arguments**:
- `page_size` (optional): Number of pins to return (max 100, default 25)
- `bookmark` (optional): Bookmark for pagination

**Example LLM prompt**: "List my Pinterest pins to find travel ideas"

---

### pinterest_get_pin

**What it does**: Gets details of a specific pin by its ID. Returns the pin's description, images, link, and board information.

**When to use**: Get full pin details before updating, sharing, or analyzing.

**Arguments**:
- `pin_id` (required): The unique identifier of the pin

**Example LLM prompt**: "Get details for pin abc123"

---

### pinterest_create_pin

**What it does**: Creates a new pin on a board. Include an image URL, title, description, and destination link.

**When to use**: Save new content to a board programmatically, automate pinning from external sources.

**Arguments**:
- `board_id` (required): Board ID where the pin will be created
- `title` (required): Pin title
- `description` (optional): Pin description
- `link` (optional): Destination URL when clicking the pin
- `image_url` (optional): URL of the image to pin

**Example LLM prompt**: "Create a pin on my Travel Ideas board with title 'Amazing Sunset' and image from https://example.com/sunset.jpg"

---

### pinterest_list_boards

**What it does**: Lists boards for the authenticated user. Returns boards the user owns or follows.

**When to use**: Find boards before creating pins, understand how content is organized.

**Arguments**:
- `page_size` (optional): Number of boards to return (max 100, default 25)
- `bookmark` (optional): Bookmark for pagination

**Example LLM prompt**: "List all my Pinterest boards"

---

### pinterest_get_board

**What it does**: Gets details of a specific board by its ID. Returns board name, description, and privacy settings.

**When to use**: Check board details before adding pins or learning about a collection.

**Arguments**:
- `board_id` (required): The unique identifier of the board

**Example LLM prompt**: "Get details for board xyz789"

---

### pinterest_create_board

**What it does**: Creates a new board. Include a name, optional description, and privacy setting.

**When to use**: Organize pins into specific collections, create new categories for content.

**Arguments**:
- `name` (required): Board name
- `description` (optional): Board description
- `privacy` (optional): public or private (default public)

**Example LLM prompt**: "Create a new board called 'Recipe Ideas' with description 'My favorite recipes from around the web'"

---

### pinterest_list_user_pins

**What it does**: Lists pins for a specific user on a specific board. Use this to view all content on a board.

**When to use**: Browse pins on a specific board, find content from a particular creator.

**Arguments**:
- `owner` (required): Owner ID (user or business)
- `board` (required): Board ID
- `page_size` (optional): Number of pins to return (max 100, default 25)
- `bookmark` (optional): Bookmark for pagination

**Example LLM prompt**: "List all pins on board abc123 owned by user xyz"

---

### pinterest_get_user_info

**What it does**: Gets the authenticated user's profile information including name, username, profile image, and counts.

**When to use**: Get context about the user before taking actions on their behalf.

**Arguments**: None

**Example LLM prompt**: "Get my Pinterest profile info"

---

### pinterest_search_pins

**What it does**: Searches pins by keyword. Returns pins that match the query across public content.

**When to use**: Find ideas, products, or inspiration on any topic.

**Arguments**:
- `query` (required): Search query
- `page_size` (optional): Number of results to return (max 100, default 25)
- `bookmark` (optional): Bookmark for pagination

**Example LLM prompt**: "Search for pins about 'Italian recipes'"

---

### pinterest_list_ads

**What it does**: Lists advertising campaigns for the authenticated user. Returns ad campaign details and performance metrics.

**When to use**: Monitor advertising performance, view campaign status.

**Arguments**:
- `ad_account_id` (required): Ad Account ID
- `page_size` (optional): Number of ads to return (max 100, default 25)
- `bookmark` (optional): Bookmark for pagination

**Example LLM prompt**: "List all ads in my ad account"

---

## Pinterest API Notes

- **Rate Limits**: Varies by endpoint and plan tier
- **Boards**: Can be public or private visibility
- **Pins**: Can include image from URL or upload
- **Ads**: Requires Pinterest Ads account
- **Analytics**: Pin engagement metrics available via separate analytics endpoints
