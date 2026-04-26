# ModMed Tools

Provider: `modmed` | Engine: `nango` | Auth: TWO_STEP via Nango

## Overview

These tools wrap the ModMed API. They allow AI agents to manage patients, appointments, practices, providers, and medical records. **Requires ModMed credentials.**

## Authentication

**Nango TWO_STEP**:
- Uses username/password and API key via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://${connectionConfig.environment}/firm/${connectionConfig.firmUrlPrefix}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `modmed_list_patients` | List patients | GET | /ema/api/v1/patients |
| `modmed_get_patient` | Get patient details | GET | /ema/api/v1/patients/{patientId} |
| `modmed_list_appointments` | List appointments | GET | /ema/api/v1/appointments |
| `modmed_get_appointment` | Get appointment details | GET | /ema/api/v1/appointments/{appointmentId} |
| `modmed_list_practices` | List practices | GET | /ema/api/v1/practices |
| `modmed_get_practice` | Get practice details | GET | /ema/api/v1/practices/{practiceId} |
| `modmed_list_providers` | List providers | GET | /ema/api/v1/providers |
| `modmed_get_provider` | Get provider details | GET | /ema/api/v1/providers/{providerId} |
| `modmed_list_medical_records` | List medical records | GET | /ema/api/v1/patients/{patientId}/records |
| `modmed_get_record` | Get medical record details | GET | /ema/api/v1/records/{recordId} |

---

## Tool Details

### modmed_list_patients

**What it does**: Lists all patients in ModMed.

**When to use**: Find patients, browse patient list.

**Arguments**:
- `search` (optional): Search by name or patient ID
- `practice_id` (optional): Filter by practice ID
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "Find patient with name John Doe"

---

### modmed_get_patient

**What it does**: Gets details of a specific patient.

**When to use**: Get patient demographics, history.

**Arguments**:
- `patientId` (required): Patient ID

**Example LLM prompt**: "Get details for patient 12345"

---

### modmed_list_appointments

**What it does**: Lists appointments in ModMed.

**When to use**: View schedules, find appointments.

**Arguments**:
- `start_date` (optional): Start date (ISO 8601)
- `end_date` (optional): End date (ISO 8601)
- `provider_id` (optional): Filter by provider ID
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List appointments for today"

---

### modmed_get_appointment

**What it does**: Gets details of a specific appointment.

**When to use**: Check appointment details, patient info.

**Arguments**:
- `appointmentId` (required): Appointment ID

**Example LLM prompt**: "Get details for appointment 12345"

---

### modmed_list_practices

**What it does**: Lists all practices in ModMed.

**When to use**: Find practice locations.

**Arguments**: None

**Example LLM prompt**: "List all practices"

---

### modmed_get_practice

**What it does**: Gets details of a specific practice.

**When to use**: Get practice information, location.

**Arguments**:
- `practiceId` (required): Practice ID

**Example LLM prompt**: "Get details for practice 12345"

---

### modmed_list_providers

**What it does**: Lists all healthcare providers in ModMed.

**When to use**: Find physicians, specialists.

**Arguments**:
- `practice_id` (optional): Filter by practice ID

**Example LLM prompt**: "List all providers"

---

### modmed_get_provider

**What it does**: Gets details of a specific provider.

**When to use**: Get provider credentials, specialty.

**Arguments**:
- `providerId` (required): Provider ID

**Example LLM prompt**: "Get details for provider 12345"

---

### modmed_list_medical_records

**What it does**: Lists medical records for a patient.

**When to use**: Access patient history, charts.

**Arguments**:
- `patientId` (required): Patient ID
- `record_type` (optional): Filter by record type
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all records for patient 12345"

---

### modmed_get_record

**What it does**: Gets details of a specific medical record.

**When to use**: Read record content, check diagnosis.

**Arguments**:
- `recordId` (required): Record ID

**Example LLM prompt**: "Get details for record 12345"

---

## ModMed Notes

- **Medical platform**: Specialty EHR/EMR
- **Patients**: Patient demographics and records
- **Appointments**: Scheduling and visits
- **Providers**: Healthcare professionals
- **Medical records**: Clinical documentation
