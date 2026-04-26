# ClickHouse Integration

## Overview

ClickHouse is a high-performance column-oriented SQL database management system designed for online analytical processing (OLAP). It enables fast queries on large datasets and is commonly used for analytics, logging, and data warehousing applications.

## Connection

ClickHouse uses HTTP interface with authentication. To connect:

1. Ensure the ClickHouse server is accessible via HTTP
2. Configure appropriate user credentials
3. Set up the connection with the correct host and port

## Available Scopes

| Scope | Description |
|-------|-------------|
| `database:read` | Read database information |
| `table:read` | Read table schema and data |
| `user:read` | Read user information |
| `user:write` | Create and modify users |
| `query:execute` | Execute SQL queries |
| `query:read` | Read query results |
| `dictionary:read` | Read dictionary data |

## Tools

### Databases

#### List Databases
Retrieve all databases on the server.

**Endpoint:** `GET /`

#### Get Database
Retrieve details of a specific database.

**Endpoint:** `GET /database/{database}`

**Parameters:**
- `database` (string, required): The database name

### Tables

#### List Tables
Retrieve all tables in a database.

**Endpoint:** `GET /database/{database}/tables`

**Parameters:**
- `database` (string, required): The database name

#### Get Table
Retrieve details and schema of a specific table.

**Endpoint:** `GET /database/{database}/tables/{table}`

**Parameters:**
- `database` (string, required): The database name
- `table` (string, required): The table name

### Users

#### List Users
Retrieve all users on the server.

**Endpoint:** `GET /users`

#### Get User
Retrieve details of a specific user.

**Endpoint:** `GET /users/{user}`

**Parameters:**
- `user` (string, required): The username

#### Create User
Create a new user on the server.

**Endpoint:** `POST /users`

**Parameters:**
- `name` (string, required): The username
- `password` (string, required): The user password
- `quotas` (array): List of quotas to assign

### Queries

#### Execute Query
Execute a SQL query on the server.

**Endpoint:** `POST /query`

**Parameters:**
- `query` (string, required): The SQL query to execute
- `database` (string): The database to execute on
- `default_format` (string): Output format (JSON, CSV, etc.)

#### Get Query Result
Retrieve results of a previously executed query.

**Endpoint:** `GET /query/{query_id}`

**Parameters:**
- `query_id` (string, required): The query identifier

### Dictionaries

#### List Dictionaries
Retrieve all dictionaries on the server.

**Endpoint:** `GET /dictionaries`

**Parameters:**
- `database` (string): Filter by database name

## Use Cases

- **Analytics Queries**: Run complex analytical queries on large datasets
- **Log Analysis**: Store and query application logs at scale
- **Business Intelligence**: Generate reports and dashboards from raw data
- **Data Exploration**: Explore data schemas and table structures
