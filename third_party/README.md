# Third-party upstream engines

This folder is reserved for upstream projects OminiConnect integrates with.

## Nango (recommended layout)

Preferred path:

- `third_party/nango/` (git submodule)

Bootstrap:

```bash
chmod +x scripts/bootstrap_nango_submodule.sh
./scripts/bootstrap_nango_submodule.sh
```

If GitHub HTTPS is flaky on your network, try:

```bash
NANGO_URL=git@github.com:NangoHQ/nango.git ./scripts/bootstrap_nango_submodule.sh
```

If you prefer not to vendor source at all, deploy Nango separately and only configure OminiConnect with `NANGO_BASE_URL` / `NANGO_SECRET_KEY` (see repo `.env.example`).
