# n8n Integration Tools

n8n is an open-source workflow automation platform that allows users to create automations with code flexibility, supporting custom nodes and integrations.

## Tools

### Workflow Management

| Tool | Description |
|------|-------------|
| `n8n_list_workflows` | List all workflows |
| `n8n_get_workflow` | Get details of a specific workflow |
| `n8n_create_workflow` | Create a new workflow |
| `n8n_update_workflow` | Update an existing workflow |
| `n8n_delete_workflow` | Delete a workflow |
| `n8n_activate_workflow` | Activate a workflow |
| `n8n_deactivate_workflow` | Deactivate a workflow |

### Execution Management

| Tool | Description |
|------|-------------|
| `n8n_list_executions` | List executions for a workflow |
| `n8n_get_execution` | Get details of a specific execution |
| `n8n_run_workflow` | Trigger a manual workflow execution |

## API Details

### Base URL
`https://your-n8n-instance.com/api/v1`

### Authentication
API key in header `X-N8N-API-KEY` or OAuth 2.0

### Endpoints

#### list_workflows
- **Method**: GET
- **Endpoint**: `/workflows`
- **Scopes**: `workflow:read`
- **Tags**: `workflows`, `list`

#### get_workflow
- **Method**: GET
- **Endpoint**: `/workflows/{workflow_id}`
- **Scopes**: `workflow:read`
- **Tags**: `workflows`, `retrieve`

#### create_workflow
- **Method**: POST
- **Endpoint**: `/workflows`
- **Scopes**: `workflow:write`
- **Tags**: `workflows`, `create`

#### update_workflow
- **Method**: PUT
- **Endpoint**: `/workflows/{workflow_id}`
- **Scopes**: `workflow:write`
- **Tags**: `workflows`, `update`

#### delete_workflow
- **Method**: DELETE
- **Endpoint**: `/workflows/{workflow_id}`
- **Scopes**: `workflow:delete`
- **Tags**: `workflows`, `delete`

#### list_executions
- **Method**: GET
- **Endpoint**: `/executions`
- **Scopes**: `execution:read`
- **Tags**: `executions`, `list`

#### get_execution
- **Method**: GET
- **Endpoint**: `/executions/{execution_id}`
- **Scopes**: `execution:read`
- **Tags**: `executions`, `retrieve`

#### run_workflow
- **Method**: POST
- **Endpoint**: `/workflows/{workflow_id}/run`
- **Scopes**: `workflow:execute`
- **Tags**: `workflows`, `execute`

#### activate_workflow
- **Method**: POST
- **Endpoint**: `/workflows/{workflow_id}/activate`
- **Scopes**: `workflow:write`
- **Tags**: `workflows`, `activate`

#### deactivate_workflow
- **Method**: POST
- **Endpoint**: `/workflows/{workflow_id}/deactivate`
- **Scopes**: `workflow:write`
- **Tags**: `workflows`, `deactivate`

## Usage Example

```typescript
// List all workflows
const workflows = await client.execute('n8n_list_workflows', {});

// Run a workflow manually
await client.execute('n8n_run_workflow', {
  workflow_id: 'workflow_789'
});
```

## Notes

- n8n supports both cloud and self-hosted deployments
- Workflows can contain multiple nodes with various integrations
- Execution history allows debugging and monitoring of workflow runs
