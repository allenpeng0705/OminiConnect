# Portal Database Configuration

The OmniConnect Portal uses [sqlx](https://github.com/org-rs/sqlx) for database persistence, supporting multiple database backends through the `any` driver.

## Supported Databases

| Database | Connection URL | Notes |
|----------|---------------|-------|
| SQLite | `sqlite:portal.db` | Default, file-based, no setup |
| MySQL | `mysql://user:pass@localhost/omni_portal` | Requires MySQL 5.7+ |
| MariaDB | `mysql://user:pass@localhost/omni_portal` | MariaDB uses MySQL protocol (10.2+) |
| PostgreSQL | `postgres://user:pass@localhost/omni_portal` | Requires PostgreSQL 12+ |

## Quick Start

### SQLite (Default)

No configuration needed. The database file `portal.db` is created automatically in the project root:

```bash
cargo run -p omni-portal
```

For an absolute path:
```bash
DATABASE_URL=sqlite:///absolute/path/to/portal.db cargo run -p omni-portal
```

### MySQL

Create the database first:
```sql
CREATE DATABASE omni_portal CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE USER 'omni'@'localhost' IDENTIFIED BY 'your_password';
GRANT ALL PRIVILEGES ON omni_portal.* TO 'omni'@'localhost';
FLUSH PRIVILEGES;
```

Run the portal:
```bash
DATABASE_URL=mysql://omni:your_password@localhost/omni_portal cargo run -p omni-portal
```

### MariaDB

Create the database first:
```sql
CREATE DATABASE omni_portal CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE USER 'omni'@'localhost' IDENTIFIED BY 'your_password';
GRANT ALL PRIVILEGES ON omni_portal.* TO 'omni'@'localhost';
FLUSH PRIVILEGES;
```

Run the portal:
```bash
DATABASE_URL=mariadb://omni:your_password@localhost/omni_portal cargo run -p omni-portal
```

### PostgreSQL

Create the database first:
```sql
CREATE DATABASE omni_portal;
CREATE USER omni WITH ENCRYPTED PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE omni_portal TO omni;
```

Run the portal:
```bash
DATABASE_URL=postgres://omni:your_password@localhost/omni_portal cargo run -p omni-portal
```

## Schema

The portal automatically creates the following tables on first run:

```sql
-- Users table
CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Sessions table
CREATE TABLE sessions (
    session_id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL
);

-- API keys table (bcrypt hash stored, raw key never persisted)
CREATE TABLE api_keys (
    key_hash TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    label TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Connectors table
CREATE TABLE connectors (
    platform TEXT PRIMARY KEY,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    redirect_uri TEXT NOT NULL DEFAULT '',
    scopes TEXT NOT NULL DEFAULT '',
    enabled INTEGER NOT NULL DEFAULT 1
);

-- OAuth tokens table (for OAuthVault TokenStoreBackend)
CREATE TABLE oauth_tokens (
    platform TEXT NOT NULL,
    subject TEXT NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    expires_at TEXT,
    created_at TEXT NOT NULL,
    PRIMARY KEY (platform, subject)
);
```

## Default Admin User

On first run, the portal seeds a default admin user:
- **Username:** `admin`
- **Password:** `admin`

**Important:** Change this password in production!

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:portal.db` | Database connection URL |

## Production Considerations

1. **Use strong passwords** - The default admin password is `admin`
2. **Use TLS** - For MySQL/MariaDB/PostgreSQL in production, use `mysql://.../db?ssl-mode=REQUIRED`
3. **Connection pooling** - The portal uses a max of 5 connections (configured in `db/mod.rs`)
4. **Backups** - Regular backups are recommended for all database types
5. **Migration strategy** - Currently migrations run on every startup with `CREATE TABLE IF NOT EXISTS` (safe for existing tables)
