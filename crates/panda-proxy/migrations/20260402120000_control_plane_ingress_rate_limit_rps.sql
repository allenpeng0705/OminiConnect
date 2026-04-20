-- Optional per-row RPS cap for dynamic ingress (mirrors static `api_gateway.ingress.routes[].rate_limit.rps`).
ALTER TABLE panda_control_plane_ingress_route ADD COLUMN rate_limit_rps INTEGER;
