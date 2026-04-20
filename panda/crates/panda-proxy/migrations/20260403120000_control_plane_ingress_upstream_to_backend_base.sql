-- Rename ingress route URL column for clarity (was `upstream`).
-- SQLite 3.25+ and PostgreSQL 9.1+.
ALTER TABLE panda_control_plane_ingress_route RENAME COLUMN upstream TO backend_base;
