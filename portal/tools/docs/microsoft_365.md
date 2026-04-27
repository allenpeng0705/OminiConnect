# Microsoft 365 Tools

Provider: `microsoft_365` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Microsoft Graph API. They allow AI agents to manage groups, users, files in OneDrive and SharePoint, calendar events, and email in Microsoft 365. Microsoft 365 is the dominant enterprise productivity suite used by millions of organizations.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `Group.Read.All`, `User.Read.All`, `Files.Read.All`, `Files.ReadWrite.All`, `Calendars.Read`, `Mail.Send`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `microsoft_365_list_groups` | List Microsoft 365 groups | GET | /groups |
| `microsoft_365_get_group` | Get group details | GET | /groups/{groupId} |
| `microsoft_365_list_group_members` | List group members | GET | /groups/{groupId}/members |
| `microsoft_365_get_user` | Get user details | GET | /users/{userId} |
| `microsoft_365_list_files` | List files in SharePoint/OneDrive | GET | /sites/{siteId}/drives/{driveId}/items/{itemId}/children |
| `microsoft_365_get_file` | Get file metadata | GET | /sites/{siteId}/drives/{driveId}/items/{itemId} |
| `microsoft_365_upload_file` | Upload a file | PUT | /sites/{siteId}/drives/{driveId}/items/{parentId}:/{fileName}:/content |
| `microsoft_365_list_calendar_events` | List calendar events | GET | /users/{userId}/calendar/events |
| `microsoft_365_get_calendar_event` | Get event details | GET | /users/{userId}/calendar/events/{eventId} |
| `microsoft_365_send_mail_message` | Send an email | POST | /users/{userId}/sendMail |

---

## Tool Details

### microsoft_365_list_groups

**What it does**: Lists Microsoft 365 groups in the organization. Supports filtering by group type, membership, and other properties.

**When to use**: Find groups in the organization, search for specific teams or distribution groups.

**Arguments**:
- `filter` (optional): OData filter expression
- `top` (optional): Max results (default 50)

**Example LLM prompt**: "List all security groups in the organization"

---

### microsoft_365_get_group

**What it does**: Gets detailed information about a specific Microsoft 365 group including settings, owners, and membership types.

**When to use**: View group settings, check group owners, understand group configuration.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "Get details for the engineering team group"

---

### microsoft_365_list_group_members

**What it does**: Lists all members of a specific Microsoft 365 group including users and other groups.

**When to use**: See who belongs to a group, verify group membership, find specific team members.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "List all members of the project-alpha group"

---

### microsoft_365_get_user

**What it does**: Gets detailed information about a Microsoft 365 user including profile, manager, and contact information.

**When to use**: Look up user details, find a user's manager, get contact information.

**Arguments**:
- `userId` (required): User ID or email

**Example LLM prompt**: "Get details for user john@company.com"

---

### microsoft_365_list_files

**What it does**: Lists files in SharePoint Online or OneDrive for Business. Supports filtering by folder and type.

**When to use**: Browse document libraries, find files in a specific folder, search for documents.

**Arguments**:
- `siteId` (required): SharePoint site ID
- `driveId` (optional): Drive ID (use 'root' for default)
- `itemId` (optional): Folder item ID (use 'root' for root)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all files in the root of the Marketing team SharePoint site"

---

### microsoft_365_get_file

**What it does**: Gets metadata for a specific file in SharePoint or OneDrive including size, created date, and sharing info.

**When to use**: Check file details before downloading, see who last modified a document.

**Arguments**:
- `siteId` (required): SharePoint site ID
- `driveId` (required): Drive ID
- `itemId` (required): File or folder item ID

**Example LLM prompt**: "Get metadata for the Q4 report in the Finance drive"

---

### microsoft_365_upload_file

**What it does**: Uploads a file to SharePoint Online or OneDrive for Business.

**When to use**: Save documents to SharePoint, upload reports, share files with team members.

**Arguments**:
- `siteId` (required): SharePoint site ID
- `driveId` (optional): Drive ID
- `parentId` (optional): Parent folder ID
- `fileName` (required): File name
- `content` (required): File content (base64 encoded)

**Example LLM prompt**: "Upload the monthly report to the Finance drive"

---

### microsoft_365_list_calendar_events

**What it does**: Lists calendar events for a user or group calendar. Supports filtering by time range and attendees.

**When to use**: Check meeting schedules, find upcoming events, see calendar availability.

**Arguments**:
- `userId` (required): User ID or email
- `startDateTime` (optional): Start date/time (ISO 8601)
- `endDateTime` (optional): End date/time (ISO 8601)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all meetings for john@company.com this week"

---

### microsoft_365_get_calendar_event

**What it does**: Gets detailed information about a specific calendar event including attendees, location, and attachments.

**When to use**: Get full meeting details, see attendee list, check video conference links.

**Arguments**:
- `userId` (required): User ID or email
- `eventId` (required): Event ID

**Example LLM prompt**: "Get details for the team standup meeting tomorrow"

---

### microsoft_365_send_mail_message

**What it does**: Sends an email message using Microsoft Graph API. Can send to internal or external recipients.

**When to use**: Send notifications, share documents, send automated reports.

**Arguments**:
- `userId` (required): Sender user ID or email
- `toRecipients` (required): Recipient email addresses (array)
- `subject` (required): Email subject
- `body` (required): Email body (plain text or HTML)
- `importance` (optional): Importance (low, normal, high, default normal)

**Example LLM prompt**: "Send an email to john@company.com about the project deadline"

---

## Microsoft 365 API Notes

- **Group IDs**: Microsoft 365 groups have unique GUID-based IDs
- **Drive IDs**: Each document library (including OneDrive) has a unique drive ID
- **Calendar Events**: Events use ISO 8601 datetime format for start/end times
- **Email Addresses**: Users can be identified by email address or GUID ID
- **Throttling**: Microsoft Graph has rate limits; implement retry logic for high-volume operations
- **Site ID Format**: SharePoint sites can use ID or server-relative path like `/sites/teamname`
