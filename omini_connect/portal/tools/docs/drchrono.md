# DrChrono Tools

Provider: `drchrono` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the DrChrono API. They allow AI agents to manage patients, appointments, calendar entries, insurance providers, and offices. DrChrono is an electronic health record (EHR) and practice management platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with DrChrono
- Token stored in Nango, accessed via `connection_ref`
- Supports refresh token flow

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `drchrono_list_patients` | List patients | GET | /api/patients |
| `drchrono_get_patient` | Get patient details | GET | /api/patients/{patient_id} |
| `drchrono_list_appointments` | List appointments | GET | /api/appointments |
| `drchrono_get_appointment` | Get appointment details | GET | /api/appointments/{appointment_id} |
| `drchrono_list_calendar_entries` | List calendar entries | GET | /api/calendar_entries |
| `drchrono_get_calendar_entry` | Get calendar entry details | GET | /api/calendar_entries/{entry_id} |
| `drchrono_list_insurance` | List insurance providers | GET | /api/insurance_providers |
| `drchrono_get_insurance` | Get insurance details | GET | /api/insurance_providers/{insurance_id} |
| `drchrono_list_offices` | List offices | GET | /api/offices |
| `drchrono_get_office` | Get office details | GET | /api/offices/{office_id} |

---

## Tool Details

### drchrono_list_patients

**What it does**: Lists all patients with pagination.

**When to use**: Browse patient database, find patients, manage patient records.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all patients"

---

### drchrono_get_patient

**What it does**: Gets detailed patient information.

**When to use**: Review patient details, check patient history, verify patient demographics.

**Arguments**:
- `patient_id` (required): Patient ID

**Example LLM prompt**: "Get details for patient p-123"

---

### drchrono_list_appointments

**What it does**: Lists all appointments with optional date filtering.

**When to use**: View scheduled appointments, find appointments by date, manage schedule.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `date` (optional): Filter by date (YYYY-MM-DD)

**Example LLM prompt**: "List all appointments for today"

---

### drchrono_get_appointment

**What it does**: Gets detailed appointment information.

**When to use**: Check appointment details, verify appointment status, review patient for appointment.

**Arguments**:
- `appointment_id` (required): Appointment ID

**Example LLM prompt**: "Get details for appointment a-456"

---

### drchrono_list_calendar_entries

**What it does**: Lists all calendar entries with optional date filtering.

**When to use**: View calendar, check availability, manage schedule blocks.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `start_date` (optional): Filter by start date (YYYY-MM-DD)

**Example LLM prompt**: "List all calendar entries for this week"

---

### drchrono_get_calendar_entry

**What it does**: Gets detailed calendar entry information.

**When to use**: Review calendar block details, check entry settings, verify availability.

**Arguments**:
- `entry_id` (required): Calendar Entry ID

**Example LLM prompt**: "Get details for calendar entry ce-789"

---

### drchrono_list_insurance

**What it does**: Lists all insurance providers.

**When to use**: Browse insurance list, find insurance companies, manage insurance data.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all insurance providers"

---

### drchrono_get_insurance

**What it does**: Gets detailed insurance provider information.

**When to use**: Check insurance details, verify coverage, review insurance policies.

**Arguments**:
- `insurance_id` (required): Insurance Provider ID

**Example LLM prompt**: "Get details for insurance i-101"

---

### drchrono_list_offices

**What it does**: Lists all offices in the practice.

**When to use**: View office locations, find specific offices, manage practice locations.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all offices"

---

### drchrono_get_office

**What it does**: Gets detailed office information.

**When to use**: Check office details, verify office settings, review office configuration.

**Arguments**:
- `office_id` (required): Office ID

**Example LLM prompt**: "Get details for office o-202"

---

## DrChrono API Notes

- **EHR/EMR**: Electronic Health Record platform for medical practices
- **Patients**: Patient demographics and medical records
- **Appointments**: Scheduled patient visits
- **Calendar Entries**: Practice calendar blocks and availability
- **Insurance**: Insurance provider and coverage information
- **Offices**: Practice location management
- **HIPAA**: Contains protected health information (PHI)
