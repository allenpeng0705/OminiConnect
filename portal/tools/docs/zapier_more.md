# Zapier Additional Tools Documentation

Zapier More provides additional tools for managing Zaps beyond the basic integrations. These tools allow for detailed Zap management, execution monitoring, and history tracking.

## Tools

### Zap Management

| Tool | Description |
|------|-------------|
| `zapier_more_list_zaps` | List all Zaps in the account |
| `zapier_more_get_zap` | Get details of a specific Zap |
| `zapier_more_create_zap` | Create a new Zap |
| `zapier_more_update_zap` | Update an existing Zap |
| `zapier_more_enable_zap` | Enable a Zap |
| `zapier_more_disable_zap` | Disable a Zap |

### Execution and Runs

| Tool | Description |
|------|-------------|
| `zapier_more_list_runs` | List recent runs for a Zap |
| `zapier_more_get_run` | Get details of a specific run |

### History

| Tool | Description |
|------|-------------|
| `zapier_more_list_history` | List full execution history |
| `zapier_more_get_history_item` | Get details of a history item |

## API Details

### Base URL
`https://api.zapier.com/v2`

### Authentication
OAuth 2.0 or API key

### Endpoints

#### list_zaps
- **Method**: GET
- **Endpoint**: `/zaps`
- **Scopes**: `zaps:read`
- **Tags**: `zaps`, `list`

#### get_zap
- **Method**: GET
- **Endpoint**: `/zaps/{zap_id}`
- **Scopes**: `zaps:read`
- **Tags**: `zaps`, `retrieve`

#### create_zap
- **Method**: POST
- **Endpoint**: `/zaps`
- **Scopes**: `zaps:write`
- **Tags**: `zaps`, `create`

#### update_zap
- **Method**: PUT
- **Endpoint**: `/zaps/{zap_id}`
- **Scopes**: `zaps:write`
- **Tags**: `zaps`, `update`

#### list_runs
- **Method**: GET
- **Endpoint**: `/zaps/{zap_id}/runs`
- **Scopes**: `runs:read`
- **Tags**: `runs`, `list`

#### get_run
- **Method**: GET
- **Endpoint**: `/runs/{run_id}`
- **Scopes**: `runs:read`
- **Tags**: `runs`, `retrieve`

#### list_history
- **Method**: GET
- **Endpoint**: `/history`
- **Scopes**: `history:read`
- **Tags**: `history`, `list`

#### get_history_item
- **Method**: GET
- **Endpoint**: `/history/{history_id}`
- **Scopes**: `history:read`
- **Tags**: `history`, `retrieve`

#### enable_zap
- **Method**: POST
- **Endpoint**: `/zaps/{zap_id}/enable`
- **Scopes**: `zaps:write`
- **Tags**: `zaps`, `activate`

#### disable_zap
- **Method**: POST
- **Endpoint**: `/zaps/{zap_id}/disable`
- **Scopes**: `zaps:write`
- **Tags**: `zaps`, `deactivate`

## Usage Example

```typescript
// List all Zaps
const zaps = await client.execute('zapier_more_list_zaps', {});

// Get specific Zap details
const zap = await client.execute('zapier_more_get_zap', {
  zap_id: ' zap_123'
});

// Enable a Zap
await client.execute('zapier_more_enable_zap', {
  zap_id: 'zap_456'
});

// List execution history
const history = await client.execute('zapier_more_list_history', {
  limit: 100,
  start_date: '2024-01-01'
});
```
