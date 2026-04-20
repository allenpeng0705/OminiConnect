-- Multi-tenant dynamic ingress: composite primary key (tenant_id, path_prefix). Empty tenant_id = global row.
CREATE TABLE panda_control_plane_ingress_route_new (
    tenant_id TEXT NOT NULL DEFAULT '',
    path_prefix TEXT NOT NULL,
    backend TEXT NOT NULL,
    methods_json TEXT NOT NULL,
    upstream TEXT,
    updated_at_ms BIGINT NOT NULL,
    PRIMARY KEY (tenant_id, path_prefix)
);

INSERT INTO panda_control_plane_ingress_route_new (tenant_id, path_prefix, backend, methods_json, upstream, updated_at_ms)
SELECT '', path_prefix, backend, methods_json, upstream, updated_at_ms FROM panda_control_plane_ingress_route;

DROP TABLE panda_control_plane_ingress_route;

ALTER TABLE panda_control_plane_ingress_route_new RENAME TO panda_control_plane_ingress_route;
