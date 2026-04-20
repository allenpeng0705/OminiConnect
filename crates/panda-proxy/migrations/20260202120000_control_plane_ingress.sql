-- Dynamic ingress routes (control plane). Works on SQLite and PostgreSQL (incl. Cloud SQL for PostgreSQL).
CREATE TABLE IF NOT EXISTS panda_control_plane_ingress_route (
    path_prefix TEXT NOT NULL PRIMARY KEY,
    backend TEXT NOT NULL,
    methods_json TEXT NOT NULL,
    upstream TEXT,
    updated_at_ms BIGINT NOT NULL
);
