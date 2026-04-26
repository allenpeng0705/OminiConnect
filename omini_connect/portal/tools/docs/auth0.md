# Auth0

Identity and access management platform for authentication, user management, and security rules.

## Provider Details

- **Provider Key**: `auth0`
- **Authentication**: OAuth2/Machine-to-machine
- **API Docs**: https://auth0.com/docs/api

## Tool Registry

### Users

| Tool | Description |
|------|-------------|
| `auth0_list_users` | Retrieve a list of all users in your tenant |
| `auth0_get_user` | Get details of a specific user by ID |
| `auth0_create_user` | Create a new user in your tenant |
| `auth0_update_user` | Update an existing user's information |

### Clients

| Tool | Description |
|------|-------------|
| `auth0_list_clients` | Retrieve a list of all clients (applications) |
| `auth0_get_client` | Get details of a specific client by ID |

### Connections

| Tool | Description |
|------|-------------|
| `auth0_list_connections` | Retrieve a list of all database and social connections |
| `auth0_get_connection` | Get details of a specific connection by ID |

### Logs and Rules

| Tool | Description |
|------|-------------|
| `auth0_get_logs` | Retrieve log entries for your tenant |
| `auth0_get_rules` | Retrieve all rules configured in your tenant |

## Scopes

The following scopes are used by Auth0 tools:

- `read:users` - List and view users
- `create:users` - Create new users
- `update:users` - Update user information
- `read:clients` - List and view clients
- `read:connections` - List and view connections
- `read:logs` - Access authentication logs
- `read:rules` - View rules configuration

## API Base URL

`https://{tenant}.auth0.com/api/v2`

## Notes

- Auth0 uses a tenant-specific domain (e.g., `mytenant.auth0.com`)
- Machine-to-machine tokens are used for backend API access
- Logs contain authentication events and can be filtered by type
- Rules are executed during the authentication pipeline