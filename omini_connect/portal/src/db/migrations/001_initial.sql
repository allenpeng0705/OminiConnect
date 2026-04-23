-- Initial schema: users, sessions, api_keys, connectors, oauth_tokens

-- Users table
CREATE TABLE IF NOT EXISTS users (
    username TEXT PRIMARY KEY,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    session_id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL
);

-- API keys table (key_hash → bcrypt hash, raw key never stored)
CREATE TABLE IF NOT EXISTS api_keys (
    key_hash TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    label TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Connectors table
CREATE TABLE IF NOT EXISTS connectors (
    platform TEXT PRIMARY KEY,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    redirect_uri TEXT NOT NULL DEFAULT '',
    scopes TEXT NOT NULL DEFAULT '',
    engine TEXT NOT NULL DEFAULT 'omini_connect_native',
    provider_key TEXT NOT NULL DEFAULT '',
    connection_ref TEXT NOT NULL DEFAULT '',
    agent_id TEXT NOT NULL DEFAULT '',
    enabled INTEGER NOT NULL DEFAULT 1
);

-- OAuth tokens table (for OAuthVault TokenStoreBackend)
CREATE TABLE IF NOT EXISTS oauth_tokens (
    platform TEXT NOT NULL,
    subject TEXT NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    expires_at TEXT,
    created_at TEXT NOT NULL,
    PRIMARY KEY (platform, subject)
);
