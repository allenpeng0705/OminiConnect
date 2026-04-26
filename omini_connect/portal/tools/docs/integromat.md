# Integromat Integration Tools

Integromat is a visual automation platform (now Make) that allows users to create workflows (scenarios) connecting apps and services via a drag-and-drop interface.

## Tools

### Scenario Management

| Tool | Description |
|------|-------------|
| `integromat_list_scenarios` | List all scenarios in a workspace |
| `integromat_get_scenario` | Get details of a specific scenario |
| `integromat_create_scenario` | Create a new scenario |
| `integromat_run_scenario` | Run a scenario manually |
| `integromat_start_scenario` | Start a scenario |
| `integromat_stop_scenario` | Stop a scenario |

### Data Management

| Tool | Description |
|------|-------------|
| `integromat_list_bundles` | List data bundles for a scenario |
| `integromat_get_bundle` | Get details of a specific bundle |

### Logging

| Tool | Description |
|------|-------------|
| `integromat_list_logs` | List execution logs for a scenario |
| `integromat_get_log` | Get details of a specific log entry |

## API Details

### Base URL
`https://www.integromat.com/api`

### Authentication
OAuth 2.0 with Bearer token

### Endpoints

#### list_scenarios
- **Method**: GET
- **Endpoint**: `/scenarios`
- **Scopes**: `scenarios:read`
- **Tags**: `scenarios`, `list`

#### get_scenario
- **Method**: GET
- **Endpoint**: `/scenarios/{scenario_id}`
- **Scopes**: `scenarios:read`
- **Tags**: `scenarios`, `retrieve`

#### create_scenario
- **Method**: POST
- **Endpoint**: `/scenarios`
- **Scopes**: `scenarios:write`
- **Tags**: `scenarios`, `create`

#### list_bundles
- **Method**: GET
- **Endpoint**: `/scenarios/{scenario_id}/bundles`
- **Scopes**: `bundles:read`
- **Tags**: `bundles`, `list`

#### get_bundle
- **Method**: GET
- **Endpoint**: `/bundles/{bundle_id}`
- **Scopes**: `bundles:read`
- **Tags**: `bundles`, `retrieve`

#### list_logs
- **Method**: GET
- **Endpoint**: `/scenarios/{scenario_id}/logs`
- **Scopes**: `logs:read`
- **Tags**: `logs`, `list`

#### get_log
- **Method**: GET
- **Endpoint**: `/logs/{log_id}`
- **Scopes**: `logs:read`
- **Tags**: `logs`, `retrieve`

#### run_scenario
- **Method**: POST
- **Endpoint**: `/scenarios/{scenario_id}/run`
- **Scopes**: `scenarios:execute`
- **Tags**: `scenarios`, `execute`

#### start_scenario
- **Method**: POST
- **Endpoint**: `/scenarios/{scenario_id}/start`
- **Scopes**: `scenarios:write`
- **Tags**: `scenarios`, `start`

#### stop_scenario
- **Method**: POST
- **Endpoint**: `/scenarios/{scenario_id}/stop`
- **Scopes**: `scenarios:write`
- **Tags**: `scenarios`, `stop`

## Usage Example

```typescript
// List all scenarios
const scenarios = await client.execute('integromat_list_scenarios', {
  workspace_id: 'ws_123'
});

// Get execution logs
const logs = await client.execute('integromat_list_logs', {
  scenario_id: 'scenario_456'
});
```
