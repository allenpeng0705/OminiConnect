# Gong Integration

Gong is a revenue intelligence platform that captures and analyzes customer interactions across calls, deals, and contacts.

## Authentication

Gong uses Bearer token authentication. The API token can be obtained from Gong Settings > API.

## Base URL

```
https://api.gong.io/v2
```

## Rate Limits

- Default rate limit: 100 requests per second
- Burst limit: 200 requests per second

## Available Tools

### Calls

| Tool | Description |
|------|-------------|
| `gong_list_calls` | Retrieve a list of all recorded calls |
| `gong_get_call` | Get details of a specific call with transcript |

### Deals

| Tool | Description |
|------|-------------|
| `gong_list_deals` | Retrieve a list of all deals |
| `gong_get_deal` | Get details of a specific deal |

### Contacts

| Tool | Description |
|------|-------------|
| `gong_list_contacts` | Retrieve a list of all contacts |
| `gong_get_contact` | Get details of a specific contact |

### Users

| Tool | Description |
|------|-------------|
| `gong_list_users` | Retrieve a list of all team members |
| `gong_get_user` | Get details of a specific user |

### Tracks

| Tool | Description |
|------|-------------|
| `gong_list_tracks` | Retrieve a list of all deal tracking stages |
| `gong_get_track` | Get details of a specific track |

## Common Use Cases

- **Call Analytics**: Analyze recorded calls for insights
- **Deal Tracking**: Monitor deal progress through stages
- **Contact Management**: Maintain customer contact information
- **Team Performance**: Track sales team activities and results
- **Revenue Intelligence**: Gather data for forecasting and analysis

## Notes

- Date filtering uses ISO 8601 format
- Page size maximum is 200 items
- Call transcripts are available via the get_call endpoint
- Track data includes deal stage progression information
