# PRTG Classic Tools

Provider: `prtg-classic` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the PRTG Classic API. They allow AI agents to monitor sensors, devices, groups, alerts, and tickets. PRTG is a network monitoring platform. **Requires PRTG API Key authentication.**

## Authentication

**Nango API Key**:
- Uses apitoken query parameter
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://{host}
- Requires host in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `prtg_list_sensors` | List sensors | GET | /api/table.json |
| `prtg_get_sensor` | Get sensor details | GET | /api/sensors/{sensorId} |
| `prtg_list_devices` | List devices | GET | /api/table.json |
| `prtg_get_device` | Get device details | GET | /api/devices/{deviceId} |
| `prtg_list_groups` | List groups | GET | /api/table.json |
| `prtg_get_group` | Get group details | GET | /api/groups/{groupId} |
| `prtg_list_alerts` | List alerts | GET | /api/alarms.json |
| `prtg_get_ticket` | Get ticket details | GET | /api/tickets/{ticketId} |
| `prtg_list_channels` | List channels | GET | /api/channels/{sensorId} |
| `prtg_get_system_status` | Get system status | GET | /api/status.json |

---

## Tool Details

### prtg_list_sensors

**What it does**: Lists all sensors in the monitoring system.

**When to use**: Browse sensor overview.

**Arguments**:
- `content` (optional): Content type (default: sensors)
- `count` (optional): Number of results (default: 50)

**Example LLM prompt**: "List all sensors"

---

### prtg_get_sensor

**What it does**: Gets detailed information about a specific sensor.

**When to use**: Get sensor status, metrics.

**Arguments**:
- `sensorId` (required): Sensor ID

**Example LLM prompt**: "Get details for sensor 12345"

---

### prtg_list_devices

**What it does**: Lists all devices in the monitoring system.

**When to use**: Browse device inventory.

**Arguments**:
- `content` (optional): Content type (default: devices)
- `count` (optional): Number of results (default: 50)

**Example LLM prompt**: "List all devices"

---

### prtg_get_device

**What it does**: Gets detailed information about a specific device.

**When to use**: Get device status, sensors.

**Arguments**:
- `deviceId` (required): Device ID

**Example LLM prompt**: "Get details for device 67890"

---

### prtg_list_groups

**What it does**: Lists all groups in the monitoring system.

**When to use**: Browse group hierarchy.

**Arguments**:
- `content` (optional): Content type (default: groups)
- `count` (optional): Number of results (default: 50)

**Example LLM prompt**: "List all groups"

---

### prtg_get_group

**What it does**: Gets detailed information about a specific group.

**When to use**: Get group status, members.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "Get details for group 11111"

---

### prtg_list_alerts

**What it does**: Lists all active alerts.

**When to use**: Monitor alerting.

**Arguments**:
- `count` (optional): Number of results (default: 50)

**Example LLM prompt**: "List all active alerts"

---

### prtg_get_ticket

**What it does**: Gets detailed information about a specific ticket.

**When to use**: Get ticket details.

**Arguments**:
- `ticketId` (required): Ticket ID

**Example LLM prompt**: "Get details for ticket 22222"

---

### prtg_list_channels

**What it does**: Lists all channels for a sensor.

**When to use**: Get sensor data channels.

**Arguments**:
- `sensorId` (required): Sensor ID

**Example LLM prompt**: "List channels for sensor 12345"

---

### prtg_get_system_status

**What it does**: Gets PRTG system status.

**When to use**: Check monitoring health.

**Arguments**: None

**Example LLM prompt**: "Get system status"

---

## PRTG Classic API Notes

- **API Key**: Uses apitoken query parameter
- **Network Monitoring**: Sensors, devices, alerts
- **Rate limits**: Apply to API calls
