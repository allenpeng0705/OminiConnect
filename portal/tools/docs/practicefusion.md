# PracticeFusion Tools

Provider: `practicefusion` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the PracticeFusion API. They allow AI agents to manage patients, appointments, medications, allergies, conditions, and lab results. PracticeFusion is an EHR platform. **Requires PracticeFusion OAuth2 authentication.**

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with PracticeFusion
- Token stored in Nango, accessed via `connection_ref`
- Base URL: Configured via connectionConfig.baseUrl
- Requires baseUrl and launch in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `practicefusion_list_patients` | List patients | GET | /v1/patients |
| `practicefusion_get_patient` | Get patient details | GET | /v1/patients/{patientId} |
| `practicefusion_list_appointments` | List appointments | GET | /v1/appointments |
| `practicefusion_get_appointment` | Get appointment details | GET | /v1/appointments/{appointmentId} |
| `practicefusion_list_medications` | List medications | GET | /v1/medications |
| `practicefusion_get_medication` | Get medication details | GET | /v1/medications/{medicationId} |
| `practicefusion_list_allergies` | List allergies | GET | /v1/allergies |
| `practicefusion_list_conditions` | List conditions | GET | /v1/conditions |
| `practicefusion_list_lab_results` | List lab results | GET | /v1/lab-results |
| `practicefusion_get_practice` | Get practice info | GET | /v1/practice |

---

## Tool Details

### practicefusion_list_patients

**What it does**: Lists all patients in the practice.

**When to use**: Browse patient directory.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active patients"

---

### practicefusion_get_patient

**What it does**: Gets detailed information about a specific patient.

**When to use**: Get patient chart, history.

**Arguments**:
- `patientId` (required): Patient ID

**Example LLM prompt**: "Get details for patient 12345"

---

### practicefusion_list_appointments

**What it does**: Lists all appointments.

**When to use**: Browse appointment schedule.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)
- `patientId` (optional): Filter by patient

**Example LLM prompt**: "Show appointments for today"

---

### practicefusion_get_appointment

**What it does**: Gets detailed information about a specific appointment.

**When to use**: Get appointment details.

**Arguments**:
- `appointmentId` (required): Appointment ID

**Example LLM prompt**: "Get details for appointment 67890"

---

### practicefusion_list_medications

**What it does**: Lists all medications.

**When to use**: Browse medication records.

**Arguments**:
- `patientId` (optional): Filter by patient

**Example LLM prompt**: "List medications for patient 12345"

---

### practicefusion_get_medication

**What it does**: Gets detailed information about a specific medication.

**When to use**: Get medication details, dosage.

**Arguments**:
- `medicationId` (required): Medication ID

**Example LLM prompt**: "Get details for medication 11111"

---

### practicefusion_list_allergies

**What it does**: Lists all allergies.

**When to use**: Browse allergy records.

**Arguments**:
- `patientId` (optional): Filter by patient

**Example LLM prompt**: "List allergies for patient 12345"

---

### practicefusion_list_conditions

**What it does**: Lists all conditions/diagnoses.

**When to use**: Browse patient conditions.

**Arguments**:
- `patientId` (optional): Filter by patient

**Example LLM prompt**: "List conditions for patient 12345"

---

### practicefusion_list_lab_results

**What it does**: Lists all lab results.

**When to use**: Browse lab records.

**Arguments**:
- `patientId` (optional): Filter by patient

**Example LLM prompt**: "List lab results for patient 12345"

---

### practicefusion_get_practice

**What it does**: Gets practice information.

**When to use**: Get practice settings.

**Arguments**: None

**Example LLM prompt**: "Get practice information"

---

## PracticeFusion API Notes

- **OAuth2**: Requires user authentication via OAuth flow
- **SMART on FHIR**: Healthcare data standard
- **HIPAA**: Handle patient data with care
