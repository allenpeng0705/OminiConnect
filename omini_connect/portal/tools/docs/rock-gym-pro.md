# Rock-gym-pro Tools

Provider: `rock-gym-pro` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Rock Gym Pro is a gym management platform for fitness centers. These tools allow AI agents to manage members, memberships, class bookings, visits, staff, and revenue reports.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Rock Gym Pro
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `members:read`, `members:write`, `memberships:read`, `memberships:write`, `visits:read`, `classes:read`, `bookings:write`, `staff:read`, `reports:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `rock-gym-pro_list_members` | List all members | GET | /v1/members |
| `rock-gym-pro_get_member` | Get member details | GET | /v1/members/{memberId} |
| `rock-gym-pro_create_member` | Create a member | POST | /v1/members |
| `rock-gym-pro_list_memberships` | List memberships | GET | /v1/memberships |
| `rock-gym-pro_create_membership` | Create a membership | POST | /v1/memberships |
| `rock-gym-pro_list_visits` | List member visits | GET | /v1/visits |
| `rock-gym-pro_list_classes` | List classes | GET | /v1/classes |
| `rock-gym-pro_create_booking` | Create a class booking | POST | /v1/bookings |
| `rock-gym-pro_list_staff` | List staff members | GET | /v1/staff |
| `rock-gym-pro_get_revenue_report` | Get revenue report | GET | /v1/reports/revenue |

---

## Tool Details

### rock-gym-pro_list_members

**What it does**: Returns a list of all gym members.

**When to use**: View member roster, find active members.

**Arguments**:
- `limit` (optional): Number of members (default 50)
- `status` (optional): Filter by status (active, inactive, paused)

**Example LLM prompt**: "List all active members"

---

### rock-gym-pro_get_member

**What it does**: Gets details of a specific member.

**When to use**: Get member info, membership status, contact details.

**Arguments**:
- `memberId` (required): The member ID

**Example LLM prompt**: "Get details for member mb_abc123"

---

### rock-gym-pro_create_member

**What it does**: Creates a new gym member.

**When to use**: Register new gym members.

**Arguments**:
- `firstName` (required): First name
- `lastName` (required): Last name
- `email` (required): Email address
- `phone` (optional): Phone number

**Example LLM prompt**: "Create a new member for John Smith with email john@example.com"

---

### rock-gym-pro_list_memberships

**What it does**: Returns a list of membership plans.

**When to use**: View available membership types.

**Arguments**:
- `limit` (optional): Number of memberships (default 50)

**Example LLM prompt**: "List all membership plans"

---

### rock-gym-pro_create_membership

**What it does**: Creates a new membership for a member.

**When to use**: Assign membership to new member.

**Arguments**:
- `memberId` (required): Member ID
- `planId` (required): Membership plan ID
- `startDate` (optional): Start date

**Example LLM prompt**: "Create membership for member mb_123 with plan pl_456"

---

### rock-gym-pro_list_visits

**What it does**: Returns a list of member visits.

**When to use**: Track gym attendance.

**Arguments**:
- `memberId` (optional): Filter by member
- `limit` (optional): Number of visits (default 50)

**Example LLM prompt**: "List visits for member mb_abc123"

---

### rock-gym-pro_list_classes

**What it does**: Returns a list of available classes.

**When to use**: View class schedule.

**Arguments**:
- `limit` (optional): Number of classes (default 50)
- `date` (optional): Filter by date

**Example LLM prompt**: "List all classes for today"

---

### rock-gym-pro_create_booking

**What it does**: Books a member into a class.

**When to use**: Reserve spot in fitness class.

**Arguments**:
- `memberId` (required): Member ID
- `classId` (required): Class ID

**Example LLM prompt**: "Book member mb_123 into class cl_456"

---

### rock-gym-pro_list_staff

**What it does**: Returns a list of staff members.

**When to use**: View gym staff and trainers.

**Arguments**:
- `limit` (optional): Number of staff (default 50)

**Example LLM prompt**: "List all gym staff"

---

### rock-gym-pro_get_revenue_report

**What it does**: Returns revenue and financial data.

**When to use**: Track gym finances and membership revenue.

**Arguments**:
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Get revenue report for this month"

---

## Rock Gym Pro Notes

- Members have memberships with different plans
- Visits track gym check-ins
- Classes include fitness courses with schedules
- Bookings reserve spots in classes
- Revenue reports show financial performance
