# Ingress TLS and ACME (catalog)

Panda’s process listens on a single HTTP(S) address (`listen` / `server`); **TLS termination** and **certificate lifecycle** are usually owned by infrastructure in front of the proxy. This document catalogs common patterns without prescribing in-process ACME (not required for core gateway behavior).

## Termination modes

1. **TLS at Panda** — `tls.cert_pem` / `tls.key_pem` in config; clients connect HTTPS directly to Panda. Suitable for dev/small deployments; rotate certs out-of-band or via sidecar/sync.
2. **TLS at a load balancer / ingress controller** — HTTP or HTTPS from the edge to Panda (often HTTP on a private network). Public TLS and ACME are handled by the LB or Kubernetes Ingress (e.g. **cert-manager** HTTP-01 or DNS-01). Panda sees plain HTTP or re-encrypted mTLS depending on chain design.
3. **Split listeners** — public 443 for clients, separate bind for admin/metrics (not implemented as a first-class feature in Panda; achieve with **two deployments**, **two Services**, or **port mapping** at the edge).

## ACME / Let’s Encrypt

- **Kubernetes**: cert-manager `Certificate` + `ClusterIssuer` / `Issuer`; Ingress or Gateway API routes terminate TLS; backend Service targets Panda’s `listen` port.
- **VM / bare metal**: **Caddy** or **nginx** with ACME plugins, **or** standalone **certbot** + reload; reverse-proxy to `127.0.0.1:<panda-port>`.
- **Cloud LBs**: AWS ALB/NLB + ACM, GCP HTTPS LB + managed certs, Azure App Gateway + Key Vault—same idea: **certs stay off the Panda host**.

## Observability

- **JWT policy per ingress row**: `api_gateway.ingress.routes[].auth` (`inherit` | `required` | `optional`) and dynamic routes via control plane / SQL (`auth_mode`).
- **RPS**: aggregate `panda_gateway_rps_*_{allowed,denied}_total{layer="ingress"}` plus per-row `panda_gateway_ingress_rps_total{tenant_id,path_prefix,result}` when a row-level limit applies.

## References

- `docs/deployment.md` — deployment layout.
- `docs/grpc_ingress_roadmap.md` — gRPC/TLS ingress direction (if present).
