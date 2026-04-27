# Health Gorilla Integration

Health Gorilla is a healthcare API platform that provides access to patient records, appointments, practitioners, and lab results through a unified FHIR-compliant interface.

## Authentication

Health Gorilla uses OAuth 2.0 with SMART on FHIR protocol. Contact Health Gorilla for API credentials and onboarding.

## Base URL

```
https://api.healthgorilla.com/fhir/v1
```

## Rate Limits

- Default rate limit: 60 requests per minute
- Bulk export: 10 requests per hour

## Available Tools

### Patients

| Tool | Description |
|------|-------------|
| `health_gorilla_list_patients` | Retrieve a list of all patients |
| `health_gorilla_get_patient` | Get details of a specific patient |
| `health_gorilla_create_patient` | Create a new patient record |

### Appointments

| Tool | Description |
|------|-------------|
| `health_gorilla_list_appointments` | Retrieve a list of all appointments |
| `health_gorilla_get_appointment` | Get details of a specific appointment |
| `health_gorilla_create_appointment` | Create a new appointment |

### Practitioners

| Tool | Description |
|------|-------------|
| `health_gorilla_list_practitioners` | Retrieve a list of all practitioners |
| `health_gorilla_get_practitioner` | Get details of a specific practitioner |

### Labs

| Tool | Description |
|------|-------------|
| `health_gorilla_list_labs` | Retrieve a list of all lab results |
| `health_gorilla_get_lab_result` | Get details of a specific lab result |

## Appointment Status Values

| Value | Status |
|-------|--------|
| scheduled | Appointment is scheduled |
| completed | Appointment has been completed |
| cancelled | Appointment was cancelled |

## Common Use Cases

- **Patient Management**: Create and manage patient records
- **Appointment Scheduling**: Schedule and track patient appointments
- **Practitioner Directory**: Access healthcare provider information
- **Lab Results Access**: Retrieve patient lab results
- **Healthcare Workflows**: Integrate healthcare data into applications

## Notes

- All dates use ISO 8601 format (YYYY-MM-DD for dates, full timestamp for date-time)
- Patient creation requires name, date of birth, and at least one contact method
- Appointment filtering can be done by patient, practitioner, or status
- Lab results are associated with specific patients
- HIPAA compliance is required for all API operations
