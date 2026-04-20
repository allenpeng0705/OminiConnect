-- Per-route JWT policy for dynamic ingress (inherit | required | optional).
ALTER TABLE panda_control_plane_ingress_route ADD COLUMN auth_mode TEXT NOT NULL DEFAULT 'inherit';
