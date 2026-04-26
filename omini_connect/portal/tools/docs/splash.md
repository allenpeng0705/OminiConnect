# Splash Tools

Provider: `splash` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Splash API. They allow AI agents to manage events, guests, invites, templates, and analytics. Splash is an event marketing platform focused on creating branded event experiences with guest management and engagement tracking.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Splash
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `events:read`, `events:write`, `guests:read`, `guests:write`, `templates:read`, `analytics:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `splash_list_events` | List all events | GET | /api/events |
| `splash_get_event` | Get event details | GET | /api/events/{event_id} |
| `splash_create_event` | Create a new event | POST | /api/events |
| `splash_list_guests` | List event guests | GET | /api/events/{event_id}/guests |
| `splash_get_guest` | Get guest details | GET | /api/guests/{guest_id} |
| `splash_invite_guest` | Create/invite guest | POST | /api/events/{event_id}/guests |
| `splash_send_invite` | Send/resend invite | POST | /api/guests/{guest_id}/send_invite |
| `splash_list_templates` | List templates | GET | /api/templates |
| `splash_get_template` | Get template details | GET | /api/templates/{template_id} |
| `splash_get_event_analytics` | Get event analytics | GET | /api/events/{event_id}/analytics |

---

## Tool Details

### splash_list_events

**What it does**: Retrieves all events created in Splash for the authenticated organization.

**When to use**: Get overview of all events, filter by status, or find events by date range.

**Arguments**:
- `status` (optional): Filter by status (draft, published, archived)
- `limit` (optional): Maximum number of events to return (default: 50)
- `offset` (optional): Number of events to skip for pagination

**Example LLM prompt**: "List all published events in my Splash account"

---

### splash_get_event

**What it does**: Retrieves detailed information about a specific event including times, location, and settings.

**When to use**: Get full event details, venue information, or event configuration.

**Arguments**:
- `event_id` (required): The unique identifier of the event

**Example LLM prompt**: "Get details for event ID 12345"

---

### splash_create_event

**What it does**: Creates a new event in Splash with details like name, timing, location, and branding.

**When to use**: Set up a new event with guest management and invitations.

**Arguments**:
- `name` (required): The name/title of the event
- `start_at` (required): Start datetime in ISO 8601 format
- `end_at` (optional): End datetime in ISO 8601 format
- `timezone` (optional): Timezone (e.g., America/New_York)
- `address` (optional): Street address of the event venue
- `city` (optional): City name
- `country` (optional): Country code (e.g., US)
- `event_type` (optional): Type of event (in_person, virtual, hybrid)
- `capacity` (optional): Maximum number of guests allowed

**Example LLM prompt**: "Create a new product launch event on June 15th at 10am Pacific with capacity of 200"

---

### splash_list_guests

**What it does**: Retrieves all guests for a specific event with their RSVP and attendance status.

**When to use**: View guest list, track RSVPs, or prepare for check-in.

**Arguments**:
- `event_id` (required): The unique identifier of the event
- `rsvp_status` (optional): Filter by RSVP status (pending, going, not_going)
- `check_in_status` (optional): Filter by check-in status (checked_in, not_checked_in)

**Example LLM prompt**: "List all guests who have RSVPed yes for the product launch"

---

### splash_get_guest

**What it does**: Retrieves detailed information about a specific guest including their RSVP details.

**When to use**: Get guest profile, RSVP status, or contact information.

**Arguments**:
- `guest_id` (required): The unique identifier of the guest

**Example LLM prompt**: "Get details for guest ID 67890"

---

### splash_invite_guest

**What it does**: Creates or updates a guest record and sends an invitation for a specific event.

**When to use**: Add guests to an event and send invitations.

**Arguments**:
- `event_id` (required): The unique identifier of the event
- `email` (required): Email address of the guest
- `first_name` (optional): First name of the guest
- `last_name` (optional): Last name of the guest
- `send_invite` (optional): Whether to send an invitation email immediately

**Example LLM prompt**: "Invite john@example.com to the product launch event and send the invitation now"

---

### splash_send_invite

**What it does**: Sends or resends an invitation email to a specific guest for an event.

**When to use**: Resend lost invitations or send reminders.

**Arguments**:
- `guest_id` (required): The unique identifier of the guest

**Example LLM prompt**: "Resend the invitation to guest ID 67890"

---

### splash_list_templates

**What it does**: Retrieves all email or page templates available in Splash for event communications.

**When to use**: Browse available templates for event pages, invitation emails, or follow-ups.

**Arguments**:
- `template_type` (optional): Filter by type (email, page, invitation)

**Example LLM prompt**: "List all invitation email templates available in Splash"

---

### splash_get_template

**What it does**: Retrieves the content and settings of a specific template.

**When to use**: Preview template design, get template variables, or understand template structure.

**Arguments**:
- `template_id` (required): The unique identifier of the template

**Example LLM prompt**: "Get details for template ID 11111"

---

### splash_get_event_analytics

**What it does**: Retrieves analytics and metrics for an event including views, RSVPs, and attendance data.

**When to use**: Measure event performance, track guest engagement, or generate reports.

**Arguments**:
- `event_id` (required): The unique identifier of the event

**Example LLM prompt**: "Get analytics for the product launch event"

---

## Splash API Notes

- **Event Types**: Splash supports in_person, virtual, and hybrid event formats
- **Guest Management**: Comprehensive RSVP tracking with pending, going, and not_going statuses
- **Check-in Tracking**: Real-time attendance tracking with check-in timestamps
- **Templates**: Reusable templates for event pages, invitation emails, and follow-up communications
- **Analytics**: Engagement metrics including page views, invitation opens, and RSVP conversion
- **Webhooks**: Splash supports webhooks for guest and RSVP events (future expansion)
