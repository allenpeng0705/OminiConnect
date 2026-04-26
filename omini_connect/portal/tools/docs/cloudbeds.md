# Cloudbeds Integration

## Overview

Cloudbeds is a hotel management platform that helps hospitality businesses manage reservations, guests, rooms, and services. It provides a comprehensive suite of tools for running hotels, hostels, vacation rentals, and other accommodation businesses.

## Connection

Cloudbeds uses OAuth 2.0 for authentication. To connect your Cloudbeds account:

1. Log in to the Cloudbeds developer portal
2. Create a new application with required permissions
3. Configure the OAuth redirect URL
4. Authorize the application

## Available Scopes

| Scope | Description |
|-------|-------------|
| `reservations:read` | Read reservation data |
| `reservations:write` | Create and modify reservations |
| `guests:read` | Read guest information |
| `rooms:read` | Read room data |
| `services:read` | Read service information |
| `reports:read` | Read reports and analytics |

## Tools

### Reservations

#### List Reservations
Retrieve all reservations for a property.

**Endpoint:** `GET /reservations`

**Parameters:**
- `property_id` (string, required): The property identifier
- `start_date` (string): Filter from date (YYYY-MM-DD)
- `end_date` (string): Filter until date (YYYY-MM-DD)
- `limit` (integer): Maximum number of reservations to return

#### Get Reservation
Retrieve details of a specific reservation.

**Endpoint:** `GET /reservations/{reservation_id}`

**Parameters:**
- `reservation_id` (string, required): The reservation identifier

#### Create Reservation
Create a new reservation.

**Endpoint:** `POST /reservations`

**Parameters:**
- `property_id` (string, required): The property identifier
- `guest_id` (string, required): The guest identifier
- `room_id` (string, required): The room identifier
- `check_in` (string, required): Check-in date (YYYY-MM-DD)
- `check_out` (string, required): Check-out date (YYYY-MM-DD)

### Guests

#### List Guests
Retrieve all guests for a property.

**Endpoint:** `GET /guests`

**Parameters:**
- `property_id` (string, required): The property identifier
- `limit` (integer): Maximum number of guests to return
- `offset` (integer): Number of guests to skip for pagination

#### Get Guest
Retrieve details of a specific guest.

**Endpoint:** `GET /guests/{guest_id}`

**Parameters:**
- `guest_id` (string, required): The guest identifier

### Rooms

#### List Rooms
Retrieve all rooms for a property.

**Endpoint:** `GET /rooms`

**Parameters:**
- `property_id` (string, required): The property identifier
- `room_type` (string): Filter by room type
- `status` (string): Filter by status (available, occupied, maintenance)

#### Get Room
Retrieve details of a specific room.

**Endpoint:** `GET /rooms/{room_id}`

**Parameters:**
- `room_id` (string, required): The room identifier

### Services

#### List Services
Retrieve all services offered by a property.

**Endpoint:** `GET /services`

**Parameters:**
- `property_id` (string, required): The property identifier
- `category` (string): Filter by service category

#### Get Service
Retrieve details of a specific service.

**Endpoint:** `GET /services/{service_id}`

**Parameters:**
- `service_id` (string, required): The service identifier

### Reports

#### Get Report
Retrieve a specific report.

**Endpoint:** `GET /reports/{report_id}`

**Parameters:**
- `report_id` (string, required): The report identifier
- `start_date` (string): Report start date (YYYY-MM-DD)
- `end_date` (string): Report end date (YYYY-MM-DD)

## Use Cases

- **Reservation Management**: Create, modify, and track guest reservations
- **Guest Communication**: Manage guest profiles and preferences
- **Room Inventory**: Monitor room availability and status
- **Service Management**: Track additional services (spa, dining, tours)
- **Reporting**: Generate operational and financial reports
