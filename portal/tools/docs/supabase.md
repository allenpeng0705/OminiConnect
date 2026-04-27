# Supabase Tools

Provider: `supabase` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Supabase API. They allow AI agents to manage projects, databases, tables, authentication users, and file storage. Supabase is an open-source Firebase alternative providing PostgreSQL databases, authentication, and scalable file storage.

## Authentication

**Nango API_KEY**:
- User authenticates via Nango Connect with Supabase
- API key stored in Nango, accessed via `connection_ref`
- Scopes: `api_key`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `supabase_list_projects` | List Supabase projects | GET | /projects |
| `supabase_get_project` | Get project details | GET | /projects/{ref} |
| `supabase_list_databases` | List project databases | GET | /projects/{ref}/databases |
| `supabase_query_table` | Query data from a table | POST | /projects/{ref}/databases/query |
| `supabase_list_tables` | List tables in a schema | GET | /projects/{ref}/tables |
| `supabase_get_table` | Get table schema | GET | /projects/{ref}/tables/{tableId} |
| `supabase_list_auth_users` | List authentication users | GET | /projects/{ref}/auth/users |
| `supabase_create_auth_user` | Create a new user | POST | /projects/{ref}/auth/users |
| `supabase_list_storage_buckets` | List storage buckets | GET | /storage/v1/buckets |
| `supabase_upload_file` | Upload file to storage | POST | /storage/v1/bucket/{bucketId}/file |

---

## Tool Details

### supabase_list_projects

**What it does**: Lists all Supabase projects the authenticated user has access to.

**When to use**: Discover available projects, choose which project to work with, list all your Supabase instances.

**Arguments**: None

**Example LLM prompt**: "List all my Supabase projects"

---

### supabase_get_project

**What it does**: Gets details about a specific Supabase project including region, status, and configuration.

**When to use**: Check project status, get project configuration, find database connection details.

**Arguments**:
- `ref` (required): Supabase project reference ID

**Example LLM prompt**: "Get details about project ref 'abc123xyz'"

---

### supabase_list_databases

**What it does**: Lists all databases in a Supabase project. Each project typically has a default PostgreSQL database.

**When to use**: Discover databases, understand project structure, list available data stores.

**Arguments**:
- `ref` (required): Supabase project reference ID

**Example LLM prompt**: "List all databases in my project"

---

### supabase_query_table

**What it does**: Executes a query against a PostgreSQL table in the Supabase database. Supports filtering, ordering, and pagination.

**When to use**: Retrieve data from tables, search records, aggregate information, perform data analysis.

**Arguments**:
- `ref` (required): Supabase project reference ID
- `query` (required): Query configuration object containing:
  - `table` (required): Table name to query
  - `columns` (optional): Columns to select (default: all)
  - `filters` (optional): Filter conditions
  - `order` (optional): Ordering configuration
  - `limit` (optional): Maximum rows to return
  - `offset` (optional): Number of rows to skip

**Example LLM prompt**: "Query the 'users' table to find all users created after January 2024"

---

### supabase_list_tables

**What it does**: Lists all tables in a specific schema of the Supabase database. Common schemas include 'public', 'storage', 'auth'.

**When to use**: Discover tables, understand database schema, find available data structures.

**Arguments**:
- `ref` (required): Supabase project reference ID
- `schema` (optional): Schema name (default: public)

**Example LLM prompt**: "List all tables in the public schema"

---

### supabase_get_table

**What it does**: Gets the schema and column definitions of a specific table.

**When to use**: Understand table structure, find column types and constraints, plan queries.

**Arguments**:
- `ref` (required): Supabase project reference ID
- `tableId` (required): Table name

**Example LLM prompt**: "Get the schema of the 'users' table"

---

### supabase_list_auth_users

**What it does**: Lists all users authenticated through Supabase Auth. Returns user metadata, creation time, and last sign-in.

**When to use**: User management, audit user activity, find specific users.

**Arguments**:
- `ref` (required): Supabase project reference ID
- `page` (optional): Page number for pagination (default: 1)
- `per_page` (optional): Users per page (default: 20)

**Example LLM prompt**: "List all users authenticated in my project"

---

### supabase_create_auth_user

**What it does**: Creates a new user in Supabase Auth. Sends a confirmation email to the user.

**When to use**: User registration, add new users to your application, invite users.

**Arguments**:
- `ref` (required): Supabase project reference ID
- `email` (required): User email address
- `password` (required): User password (min 6 characters)
- `user_metadata` (optional): Additional user metadata

**Example LLM prompt**: "Create a new user with email 'newuser@example.com' and password 'secure123'"

---

### supabase_list_storage_buckets

**What it does**: Lists all storage buckets in a Supabase project. Buckets contain files organized in folders.

**When to use**: Discover storage resources, find buckets for file operations, understand file organization.

**Arguments**: None

**Example LLM prompt**: "List all storage buckets in my project"

---

### supabase_upload_file

**What it does**: Uploads a file to a Supabase Storage bucket. Creates parent folders if they do not exist.

**When to use**: Store user uploads, save generated content, backup files, serve static assets.

**Arguments**:
- `bucketId` (required): Storage bucket ID
- `path` (required): File path within the bucket
- `file` (required): File content (base64 encoded)
- `contentType` (optional): MIME type of the file

**Example LLM prompt**: "Upload an image file to the 'avatars' bucket at path 'user123/profile.jpg'"

---

## Supabase API Notes

- **Project Ref**: Found in Supabase dashboard settings, format is alphanumeric string
- **Database ID**: Usually `default` for primary database
- **Schema**: Default schema is `public`, can use custom schemas for organization
- **RLS**: Row Level Security is enabled by default; ensure policies allow your operations
- **Auth**: Supabase Auth handles user authentication, separate from database permissions
- **Storage**: Files are organized in buckets, similar to AWS S3 buckets
- **Filters**: Supabase uses dot notation for operators (e.g., `id.eq.5`, `email.ilike.%@example.com`)
