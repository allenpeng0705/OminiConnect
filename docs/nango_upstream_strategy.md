# Nango Upstream Strategy (Option B)

## Decision

OminiConnect keeps Rust as the product/control plane and treats Nango as an upstream integration engine.

- OminiConnect portal is the only tenant-facing portal.
- Nango portal is internal ops/debug.
- OminiConnect supports two connector engines:
  - `omini_connect_native` for China-first and custom providers
  - `nango` for overlapping global providers

## Recommended Repository Layout

Do not place Nango directly at repo root.

Recommended paths:

- `panda/` (existing upstream engine)
- `third_party/nango/` (optional vendored upstream copy, if you decide to vendor)
- `omini_connect/` (OminiConnect product layer)

If you run Nango as an external deployment, keep only integration docs/config here (no source vendoring required).

## Source Management Options

Pick one and use it consistently:

1. **Submodule** (clean history separation; best for large upstream repos)
2. **Subtree** (single-repo UX; easier for teams that dislike submodules)
3. **Separate repo deployment** (no source copy in OminiConnect; consume over HTTP)

Default recommendation: **separate deployment + HTTP integration** first, then decide if vendoring is needed.

## Update Workflow

## A) If using a git submodule

```bash
# Initial add
git submodule add https://github.com/NangoHQ/nango third_party/nango
git submodule update --init --recursive

# Update later
git submodule update --remote --recursive
```

Repo helper script (preferred):

```bash
chmod +x scripts/bootstrap_nango_submodule.sh
./scripts/bootstrap_nango_submodule.sh
```

If HTTPS to GitHub is flaky, prefer SSH:

```bash
NANGO_URL=git@github.com:NangoHQ/nango.git ./scripts/bootstrap_nango_submodule.sh
```

## B) If using git subtree

```bash
# Initial add
git remote add nango-upstream https://github.com/NangoHQ/nango
git fetch nango-upstream
git subtree add --prefix=third_party/nango nango-upstream master --squash

# Update later
git fetch nango-upstream
git subtree pull --prefix=third_party/nango nango-upstream master --squash
```

## C) If using separate deployment (no vendoring)

- Deploy Nango independently (Docker/K8s).
- Configure OminiConnect with:
  - `NANGO_BASE_URL`
  - `NANGO_SECRET_KEY`
- OminiConnect uses Nango through HTTP adapter only.

## Connector Ownership Rules

- `engine=nango`:
  - LinkedIn
  - Facebook
  - X/Twitter
  - Other global connectors proven in Nango
- `engine=omini_connect_native`:
  - Feishu
  - DingTalk
  - WeChat Work
  - China-specific connectors not available in Nango

## Deprecation Rules for Existing OminiConnect Connectors

Do not remove OminiConnect native implementation immediately.

Remove only after:

1. Nango-backed path passes parity tests (auth, proxy, errors, refresh behavior)
2. Staging burn-in is stable
3. Rollback path exists (`engine` switch back to `omini_connect_native`)

## Minimal Integration Contract (Rust)

OminiConnect Rust adapter should own:

- engine routing (`nango` vs `omini_connect_native`)
- normalized test/status responses
- policy and audit hooks
- stable API contracts for portal/SDK/MCP

Nango should own:

- provider auth lifecycle
- provider proxy execution
- provider catalog maintenance

## Operational Principle

Treat Nango updates exactly like Panda updates:

- sync upstream on schedule
- run compatibility tests
- ship only after parity checks

This keeps OminiConnect stable while gaining connector breadth.
