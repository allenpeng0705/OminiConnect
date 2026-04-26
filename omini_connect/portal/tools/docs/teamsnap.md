# TeamSnap Tools

Provider: `teamsnap` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the TeamSnap API. They allow AI agents to manage sports teams, members, events, and schedules. TeamSnap is a popular platform for managing recreational and competitive sports teams.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with TeamSnap
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `teams:r`, `teams:w`, `events:r`, `events:w`, `members:r`, `schedules:r`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `teamsnap_list_teams` | List all teams | GET | /teams |
| `teamsnap_get_team` | Get team details | GET | /teams/{teamId} |
| `teamsnap_create_team` | Create a new team | POST | /teams |
| `teamsnap_list_events` | List all events | GET | /teams/{teamId}/events |
| `teamsnap_get_event` | Get event details | GET | /events/{eventId} |
| `teamsnap_create_event` | Create a new event | POST | /teams/{teamId}/events |
| `teamsnap_list_members` | List team members | GET | /teams/{teamId}/members |
| `teamsnap_get_member` | Get member details | GET | /members/{memberId} |
| `teamsnap_list_schedules` | List schedules | GET | /teams/{teamId}/schedules |
| `teamsnap_get_schedule` | Get schedule details | GET | /schedules/{scheduleId} |

---

## Tool Details

### teamsnap_list_teams

**What it does**: Lists all teams for the authenticated user. Filter by sport, name, or member status.

**When to use**: Find teams to manage, see all teams you're involved with.

**Arguments**:
- `sport` (optional): Filter by sport type
- `name` (optional): Search by team name
- `member_id` (optional): Filter by member ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "Show me all my soccer teams"

---

### teamsnap_get_team

**What it does**: Gets detailed information about a specific team including members, events, and team settings.

**When to use**: Get team overview before managing members or events.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "What is the details of team 12345?"

---

### teamsnap_create_team

**What it does**: Creates a new team with name, sport type, and settings. Can optionally invite members during creation.

**When to use**: Start a new team for a season or league.

**Arguments**:
- `name` (required): Team name
- `sport` (required): Sport type (soccer, basketball, baseball, etc.)
- `location` (optional): Team location
- `timezone` (optional): Team timezone
- `division` (optional): Division or league
- `is_public` (optional): Public team visibility (default false)

**Example LLM prompt**: "Create a new youth soccer team called Riverside Wildcats"

---

### teamsnap_list_events

**What it does**: Lists all events for a team including games, practices, and other team activities.

**When to use**: See upcoming games, find practice schedule, check event locations.

**Arguments**:
- `teamId` (required): Team ID
- `from_date` (optional): Start date (YYYY-MM-DD)
- `to_date` (optional): End date (YYYY-MM-DD)
- `event_type` (optional): Filter by type (`game`, `practice`, `other`)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "Show me all games for the Wildcats this month"

---

### teamsnap_get_event

**What it does**: Gets detailed information about a specific event including location, time, and participant RSVPs.

**When to use**: Get full event details, find location, see who is coming.

**Arguments**:
- `eventId` (required): Event ID

**Example LLM prompt**: "What is the location of event 456?"

---

### teamsnap_create_event

**What it does**: Creates a new event for a team such as a game, practice, or team meeting.

**When to use**: Schedule a new practice, add a game, create a team meeting.

**Arguments**:
- `teamId` (required): Team ID
- `title` (required): Event title
- `event_type` (required): Event type (`game`, `practice`, `meeting`)
- `start_date` (required): Start date and time (ISO 8601)
- `end_date` (optional): End date and time (ISO 8601)
- `location` (optional): Event location
- `opponent` (optional): Opponent team name
- `notes` (optional): Event notes

**Example LLM prompt**: "Schedule a practice for the Wildcats next Tuesday at 6pm at the main field"

---

### teamsnap_list_members

**What it does**: Lists all members of a team including players, coaches, and staff with their roles and contact info.

**When to use**: See roster, find contact info for team members.

**Arguments**:
- `teamId` (required): Team ID
- `role` (optional): Filter by role (`player`, `coach`, `staff`)
- `status` (optional): Filter by status (`active`, `inactive`)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "Who are all the players on the Wildcats?"

---

### teamsnap_get_member

**What it does**: Gets detailed information about a specific team member including contact info, role, and availability.

**When to use**: Get member details, find contact information for a specific player.

**Arguments**:
- `memberId` (required): Member ID

**Example LLM prompt**: "What is the phone number for member 789?"

---

### teamsnap_list_schedules

**What it does**: Lists all schedules for a team including game schedules, practice schedules, and event calendars.

**When to use**: See overall team schedule, find recurring practice times.

**Arguments**:
- `teamId` (required): Team ID
- `schedule_type` (optional): Filter by type (`game`, `practice`, `all`)
- `from_date` (optional): Start date (YYYY-MM-DD)
- `to_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show me the practice schedule for the Wildcats this season"

---

### teamsnap_get_schedule

**What it does**: Gets detailed information about a specific schedule including all events and recurring schedule rules.

**When to use**: Understand recurring schedule patterns, see all events in a schedule.

**Arguments**:
- `scheduleId` (required): Schedule ID

**Example LLM prompt**: "What are the recurring times for the Tuesday practice schedule?"

---

## TeamSnap API Notes

- **Team IDs**: Numeric identifiers for teams (e.g., `12345`)
- **Member IDs**: Numeric identifiers for team members (e.g., `789`)
- **Event IDs**: Numeric identifiers for team events (e.g., `456`)
- **Schedule IDs**: Numeric identifiers for team schedules
- **ISO 8601**: All timestamps use ISO 8601 format (e.g., `2024-01-15T09:00:00Z`)
- **Event types**: Common types are `game`, `practice`, `meeting`, and `other`
- **Roles**: Members can have roles like `player`, `coach`, `manager`, `staff`
- **Sport types**: Soccer, basketball, baseball, football, hockey, volleyball, and more
- **RSVP tracking**: Events may have RSVP tracking for member attendance
- **Recurring schedules**: Schedules can have recurring rules for practices or regular games