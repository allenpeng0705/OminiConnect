# Adobe Tools

Provider: `adobe` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Adobe Experience Cloud APIs. They allow AI agents to manage digital assets in Adobe Experience Manager (AEM), work with Adobe Admin Console users, and retrieve analytics data. Adobe Creative Cloud and Experience Cloud are industry-leading tools for enterprise content creation and management.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Adobe
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `create`, `delete`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `adobe_list_assets` | List assets in AEM DAM | GET | /api/assets |
| `adobe_get_asset` | Get asset details | GET | /api/assets/{path} |
| `adobe_upload_asset` | Upload a new asset | POST | /api/assets/{folderPath} |
| `adobe_delete_asset` | Delete an asset | DELETE | /api/assets/{path} |
| `adobe_list_collections` | List asset collections | GET | /api/assets/collections |
| `adobe_get_collection` | Get collection details | GET | /api/assets/collections/{collectionId} |
| `adobe_create_collection` | Create a collection | POST | /api/assets/collections |
| `adobe_list_users` | List Adobe users | GET | /api/usermanagement/users |
| `adobe_get_user` | Get user details | GET | /api/usermanagement/users/{userId} |
| `adobe_get_analytics` | Get analytics data | GET | /api/analytics/{reportSuiteId} |

---

## Tool Details

### adobe_list_assets

**What it does**: Lists assets in Adobe Experience Manager (AEM) Digital Asset Management. Supports filtering by folder, MIME type, and more.

**When to use**: Browse and find assets like images, videos, documents, or templates stored in AEM.

**Arguments**:
- `folder` (optional): Folder path (e.g., `/content/dam`)
- `mimeType` (optional): Filter by MIME type (e.g., `image/jpeg`)
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "List all PNG images in /content/dam/marketing"

---

### adobe_get_asset

**What it does**: Gets detailed metadata about a specific AEM asset including renditions, metadata, and relations.

**When to use**: Get full asset details before downloading, updating, or sharing.

**Arguments**:
- `path` (required): Asset path (e.g., `/content/dam/images/photo.jpg`)

**Example LLM prompt**: "Get details for the hero banner image at /content/dam/hero.jpg"

---

### adobe_upload_asset

**What it does**: Uploads a new asset to Adobe Experience Manager DAM.

**When to use**: Add new images, documents, or other digital assets to the DAM library.

**Arguments**:
- `folderPath` (required): Destination folder path (e.g., `/content/dam/myproject`)
- `fileName` (required): File name for the asset
- `mimeType` (optional): MIME type (e.g., `image/png`)

**Example LLM prompt**: "Upload a new logo to /content/dam/brand"

---

### adobe_delete_asset

**What it does**: Permanently deletes an asset from Adobe Experience Manager including all its renditions.

**When to use**: Remove unwanted assets from the DAM. This cannot be undone.

**Arguments**:
- `path` (required): Asset path

**Example LLM prompt**: "Delete the old banner image at /content/dam/old-banner.jpg"

---

### adobe_list_collections

**What it does**: Lists all asset collections in Adobe Experience Manager. Collections group related assets for organized access.

**When to use**: Find existing collections to add assets to or see what asset groupings exist.

**Arguments**:
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "List all asset collections in AEM"

---

### adobe_get_collection

**What it does**: Gets details about a specific collection including its assets and metadata.

**When to use**: View all assets within a collection, understand collection structure.

**Arguments**:
- `collectionId` (required): Collection ID

**Example LLM prompt**: "Get details for the marketing campaign collection"

---

### adobe_create_collection

**What it does**: Creates a new asset collection in Adobe Experience Manager for grouping related assets.

**When to use**: Organize assets for a campaign, project, or team sharing.

**Arguments**:
- `name` (required): Collection name
- `description` (optional): Description of the collection

**Example LLM prompt**: "Create a collection called 'Q2 Campaign Assets'"

---

### adobe_list_users

**What it does**: Lists users in Adobe Admin Console with filtering by group and product access.

**When to use**: See all users in the organization, find specific users by group.

**Arguments**:
- `groupId` (optional): Filter by group ID
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "List all users in the Creative Cloud group"

---

### adobe_get_user

**What it does**: Gets detailed information about an Adobe user including their groups and product assignments.

**When to use**: Check user details, verify product access, or find contact information.

**Arguments**:
- `userId` (required): User ID or email

**Example LLM prompt**: "Get details for user john@company.com"

---

### adobe_get_analytics

**What it does**: Retrieves analytics data from Adobe Analytics including page views, visitors, and conversions.

**When to use**: Track website performance, measure campaign success, understand user behavior.

**Arguments**:
- `reportSuiteId` (required): Adobe Analytics report suite ID
- `dateFrom` (optional): Start date (YYYY-MM-DD)
- `dateTo` (optional): End date (YYYY-MM-DD)
- `metrics` (optional): Metrics to retrieve (comma-separated)

**Example LLM prompt**: "Get page views and conversions for report suite xyz for the last 30 days"

---

## Adobe API Notes

- **AEM Assets API**: Assets are stored in `/content/dam` with metadata in JCR properties
- **Renditions**: Images have multiple renditions (thumbnail, web, print) automatically generated
- **Adobe ID**: Users are identified by their Adobe ID (email) across Creative Cloud services
- **Admin Console**: User management happens in the Adobe Admin Console at admin console.adobe.io
- **Analytics**: Adobe Analytics requires a separate Analytics product assignment in Admin Console
