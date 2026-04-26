# Make Integration Tools

Make (formerly Integromat) is a visual automation platform that allows users to create workflows (scenarios) connecting apps and services.

## Tools

### Scenario Management

| Tool | Description |
|------|-------------|
| `make_list_scenarios` | List all scenarios in a workspace |
| `make_get_scenario` | Get details of a specific scenario |
| `make_create_scenario` | Create a new scenario |
| `make_update_scenario` | Update an existing scenario |
| `make_activate_scenario` | Activate a scenario |
| `make_deactivate_scenario` | Deactivate a scenario |

### Execution Management

| Tool | Description |
|------|-------------|
| `make_list_executions` | List recent executions for a scenario |
| `make_get_execution` | Get details of a specific execution |
| `make_run_scenario` | Trigger a manual scenario run |

### Workspace

| Tool | Description |
|------|-------------|
| `make_list_workspaces` | List all workspaces |

## API Details

### Base URL
`https://www.make.com/api`

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

#### update_scenario
- **Method**: PUT
- **Endpoint**: `/scenarios/{scenario_id}`
- **Scopes**: `scenarios:write`
- **Tags**: `scenarios`, `update`

#### list_executions
- **Method**: GET
- **Endpoint**: `/scenarios/{scenario_id}/executions`
- **Scopes**: `executions:read`
- **Tags**: `executions`, `list`

#### get_execution
- **Method**: GET
- **Endpoint**: `/executions/{execution_id}`
- **Scopes**: `executions:read`
- **Tags**: `executions`, `retrieve`

#### run_scenario
- **Method**: POST
- **Endpoint**: `/scenarios/{scenario_id}/run`
- **Scopes**: `scenarios:execute`
- **Tags**: `scenarios`, `execute`

#### activate_scenario
- **Method**: POST
- **Endpoint**: `/scenarios/{scenario_id}/activate`
- **Scopes**: `scenarios:write`
- **Tags**: `scenarios`, `activate`

#### deactivate_scenario
- **Method**: POST
- **Endpoint**: `/scenarios/{scenario_id}/deactivate`
- **Scopes**: `scenarios:write`
- **Tags**: `scenarios`, `deactivate`

#### list_workspaces
- **Method**: GET
- **Endpoint**: `/workspaces`
- **Scopes**: `workspaces:read`
- **Tags**: `workspaces`, `list`

## Usage Example

```typescript
// List all scenarios
const scenarios = await client.execute('make_list_scenarios', {
  workspace_id: 'ws_123'
});

// Run a scenario manually
await client.execute('make_run_scenario', {
  scenario_id: 'scenario_456'
});
```
