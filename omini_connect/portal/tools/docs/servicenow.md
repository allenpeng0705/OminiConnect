# ServiceNow Tools

Provider: `servicenow` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the ServiceNow REST API. They allow AI agents to manage incidents, problems, change requests, and users in ServiceNow ITSM. ServiceNow is the leading enterprise IT service management platform used by large organizations for tracking incidents, problems, changes, and service requests.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with ServiceNow
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `incident.read`, `incident.write`, `problem.read`, `change.read`, `user.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `servicenow_list_incidents` | List IT incidents | GET | /api/now/table/incident |
| `servicenow_get_incident` | Get incident details | GET | /api/now/table/incident/{sysId} |
| `servicenow_create_incident` | Create a new incident | POST | /api/now/table/incident |
| `servicenow_update_incident` | Update an incident | PUT | /api/now/table/incident/{sysId} |
| `servicenow_list_problems` | List IT problems | GET | /api/now/table/problem |
| `servicenow_get_problem` | Get problem details | GET | /api/now/table/problem/{sysId} |
| `servicenow_list_changes` | List change requests | GET | /api/now/table/change_request |
| `servicenow_get_change` | Get change request details | GET | /api/now/table/change_request/{sysId} |
| `servicenow_list_users` | List ServiceNow users | GET | /api/now/table/sys_user |
| `servicenow_get_user` | Get user details | GET | /api/now/table/sys_user/{sysId} |

---

## Tool Details

### servicenow_list_incidents

**What it does**: Lists IT incidents in ServiceNow. Supports filtering by state, priority, assignment, and more.

**When to use**: Find open incidents, track incidents by assignee, search for specific issue types.

**Arguments**:
- `state` (optional): Incident state (1=new, 2=in progress, 3=awaiting, 6=resolved, 7=closed)
- `priority` (optional): Priority (1=critical, 2=high, 3=standard, 4=low, 5=planning)
- `assignedTo` (optional): Assignee user ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all high priority incidents that are still open"

---

### servicenow_get_incident

**What it does**: Gets detailed information about a specific ServiceNow incident including work notes and activities.

**When to use**: Read full incident details, understand current state before updating.

**Arguments**:
- `sysId` (required): Incident sys_id

**Example LLM prompt**: "Get details for incident with sys_id abc123"

---

### servicenow_create_incident

**What it does**: Creates a new IT incident in ServiceNow. Set description, impact, urgency, and assignment.

**When to use**: Log a new IT issue, report an outage, or request a service.

**Arguments**:
- `shortDescription` (required): Short description of the incident
- `description` (optional): Detailed description
- `impact` (optional): Impact (1=high, 2=medium, 3=low)
- `urgency` (optional): Urgency (1=high, 2=medium, 3=low)
- `assignedTo` (optional): Assignee user ID
- `callerId` (optional): Caller user ID

**Example LLM prompt**: "Create an incident for the email system being down with high impact and urgency"

---

### servicenow_update_incident

**What it does**: Updates an existing ServiceNow incident. Modify state, priority, assignment, work notes, and more.

**When to use**: Change incident status, reassign to another technician, add work notes.

**Arguments**:
- `sysId` (required): Incident sys_id
- `state` (optional): Incident state
- `priority` (optional): Priority
- `workNotes` (optional): Work notes
- `assignedTo` (optional): Assignee user ID

**Example LLM prompt**: "Update incident abc123 to in progress and assign to john.smith"

---

### servicenow_list_problems

**What it does**: Lists IT problems in ServiceNow. Filter by state, priority, and assignment.

**When to use**: Track known errors, find problems related to incidents, monitor problem management.

**Arguments**:
- `state` (optional): Problem state
- `priority` (optional): Priority
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all known problems with high priority"

---

### servicenow_get_problem

**What it does**: Gets detailed information about a specific ServiceNow problem including related incidents.

**When to use**: Understand problem details, see linked incidents, track problem resolution.

**Arguments**:
- `sysId` (required): Problem sys_id

**Example LLM prompt**: "Get details for problem PRB0012345"

---

### servicenow_list_changes

**What it does**: Lists IT change requests in ServiceNow. Filter by state, risk, type, and assignment.

**When to use**: Track upcoming changes, review change requests, monitor change pipeline.

**Arguments**:
- `state` (optional): Change state (1=new, 2=accepted, 3=in progress, 4=review, 5=closed)
- `risk` (optional): Risk level (low, medium, high, critical)
- `type` (optional): Change type (normal, standard, emergency)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all emergency changes from this month"

---

### servicenow_get_change

**What it does**: Gets detailed information about a specific ServiceNow change request including tasks and approvals.

**When to use**: Review change details, check approval status, see affected CIs.

**Arguments**:
- `sysId` (required): Change request sys_id

**Example LLM prompt**: "Get details for change request CHG0012345"

---

### servicenow_list_users

**What it does**: Lists users in ServiceNow. Filter by department, role, location, and active status.

**When to use**: Find IT staff, look up department members, check who has specific roles.

**Arguments**:
- `department` (optional): Department name
- `role` (optional): User role
- `active` (optional): Active status (default true)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all active users in the IT department"

---

### servicenow_get_user

**What it does**: Gets detailed information about a ServiceNow user including roles, groups, and manager.

**When to use**: Check user details, verify permissions, find contact information.

**Arguments**:
- `sysId` (required): User sys_id

**Example LLM prompt**: "Get details for user with sys_id abc123"

---

## ServiceNow API Notes

- **sys ID**: Every record in ServiceNow has a unique sys ID for precise identification
- **State values**: Incidents typically use 1=New, 2=In Progress, 3=On Hold, 6=Resolved, 7=Closed
- **Priority matrix**: ServiceNow calculates priority based on impact and urgency values
- **REST API versioning**: Use `/api/now/table` format for table access
- **Change Risk Levels**: low, medium, high, critical - determine approval workflow
- **User Roles**: Users can have multiple roles that control access and permissions
