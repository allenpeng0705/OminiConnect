# Apify

Web scraping and automation platform for running actors, tasks, and managing datasets.

## Provider Details

- **Provider Key**: `apify`
- **Authentication**: API token
- **API Docs**: https://docs.apify.com/

## Tool Registry

### Actors

| Tool | Description |
|------|-------------|
| `apify_list_actors` | Retrieve a list of all actors in your account |
| `apify_get_actor` | Get details of a specific actor by ID or name |
| `apify_start_actor_run` | Start a new run of an actor with optional input |
| `apify_get_run` | Get details and status of a specific actor run |

### Tasks

| Tool | Description |
|------|-------------|
| `apify_list_tasks` | Retrieve a list of all tasks in your account |
| `apify_get_task` | Get details of a specific task by ID or name |
| `apify_start_task` | Start a new run of a task with optional input |

### Datasets

| Tool | Description |
|------|-------------|
| `apify_list_datasets` | Retrieve a list of all datasets in your account |
| `apify_get_dataset` | Get details of a specific dataset by ID or name |
| `apify_get_storage` | Get details of a key-value store or dataset storage |

## Scopes

The following scopes are used by Apify tools:

- `actor:read` - Read actors and tasks
- `actor:write` - Run actors and tasks
- `dataset:read` - Read datasets
- `storage:read` - Read storage

## API Base URL

`https://api.apify.com`

## Notes

- Actor runs are asynchronous; use `apify_get_run` to poll for completion
- Datasets store structured data from actor runs
- Key-value stores hold temporary data like results and inputs
- Tasks are pre-configured actor setups with fixed input schemas