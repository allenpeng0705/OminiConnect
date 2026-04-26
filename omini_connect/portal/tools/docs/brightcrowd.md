# BrightCrowd Tools

Provider: `brightcrowd` | Engine: `nango` | Auth: OAuth2_CC via Nango (Client Credentials)

## Overview

These tools wrap the BrightCrowd API. They allow AI agents to manage organizations, members, badges, and endorsements for employee recognition and peer-to-peer acknowledgment platforms.

## Authentication

**Nango OAuth2 CC**:
- Uses Client Credentials flow (machine-to-machine)
- User provides client ID and client secret
- Token stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `brightcrowd_list_organizations` | List organizations | GET | /partner/organizations |
| `brightcrowd_get_organization` | Get organization details | GET | /partner/organizations/{orgId} |
| `brightcrowd_list_members` | List organization members | GET | /partner/organizations/{orgId}/members |
| `brightcrowd_get_member` | Get member details | GET | /partner/members/{memberId} |
| `brightcrowd_list_badges` | List badges | GET | /partner/badges |
| `brightcrowd_get_badge` | Get badge details | GET | /partner/badges/{badgeId} |
| `brightcrowd_list_endorsements` | List endorsements | GET | /partner/endorsements |
| `brightcrowd_get_endorsement` | Get endorsement details | GET | /partner/endorsements/{endorsementId} |
| `brightcrowd_list_activities` | List activities | GET | /partner/activities |
| `brightcrowd_get_insights` | Get organization insights | GET | /partner/insights |

---

## Tool Details

### brightcrowd_list_organizations

**What it does**: Lists all organizations in BrightCrowd.

**When to use**: Browse organizations, find specific orgs.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Organizations per page (default 20)

**Example LLM prompt**: "List all organizations"

---

### brightcrowd_get_organization

**What it does**: Gets details for a specific organization.

**When to use**: View organization settings, stats.

**Arguments**:
- `orgId` (required): Organization ID

**Example LLM prompt**: "Get details for organization O-123"

---

### brightcrowd_list_members

**What it does**: Lists members of an organization.

**When to use**: View organization members, find people.

**Arguments**:
- `orgId` (required): Organization ID
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Members per page (default 20)

**Example LLM prompt**: "List members of organization O-123"

---

### brightcrowd_get_member

**What it does**: Gets details for a specific member.

**When to use**: View member profile, badges earned.

**Arguments**:
- `memberId` (required): Member ID

**Example LLM prompt**: "Get details for member M-456"

---

### brightcrowd_list_badges

**What it does**: Lists all badges in the organization.

**When to use**: Browse available badges, recognition programs.

**Arguments**:
- `orgId` (optional): Organization ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all badges"

---

### brightcrowd_get_badge

**What it does**: Gets details for a specific badge.

**When to use**: Check badge criteria, recipients.

**Arguments**:
- `badgeId` (required): Badge ID

**Example LLM prompt**: "Get details for badge B-789"

---

### brightcrowd_list_endorsements

**What it does**: Lists all endorsements in the organization.

**When to use**: View peer recognition, endorsements given.

**Arguments**:
- `orgId` (optional): Organization ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all endorsements"

---

### brightcrowd_get_endorsement

**What it does**: Gets details for a specific endorsement.

**When to use**: View endorsement details, reasons.

**Arguments**:
- `endorsementId` (required): Endorsement ID

**Example LLM prompt**: "Get details for endorsement E-100"

---

### brightcrowd_list_activities

**What it does**: Lists all activities in the organization.

**When to use**: View activity feed, recent recognitions.

**Arguments**:
- `orgId` (optional): Organization ID
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Activities per page (default 20)

**Example LLM prompt**: "List recent activities"

---

### brightcrowd_get_insights

**What it does**: Gets insights and analytics for the organization.

**When to use**: Analyze recognition trends, engagement.

**Arguments**:
- `orgId` (required): Organization ID

**Example LLM prompt**: "Get insights for organization O-123"

---

## BrightCrowd API Notes

- **Organizations**: Companies or teams using BrightCrowd
- **Members**: Employees within organizations
- **Badges**: Recognition awards that can be earned
- **Endorsements**: Peer-to-peer acknowledgments
- **Activities**: Feed of recognition events
