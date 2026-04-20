# Dependency security audit (cargo-audit)

Panda uses [`cargo-audit`](https://github.com/RustSec/rustsec/tree/main/cargo-audit) to scan the workspace lockfile against the [RustSec advisory database](https://github.com/RustSec/advisory-db).

## How to run

From the repository root:

```bash
cargo install cargo-audit
./scripts/run_cargo_audit.sh
```

Or directly:

```bash
cargo audit
```

Optional: ignore a specific advisory only after explicit review (document the ID and reason in your PR):

```bash
cargo audit --ignore RUSTSEC-0000-0000
```

## What to do when findings appear

1. **Upgrade the affected crate** to a patched version (`cargo update -p crate_name`) and re-run tests.
2. If no fixed release exists yet, document the risk (affected feature, exposure, compensating controls) and consider `--ignore` with a linked tracking issue.
3. Refresh `Cargo.lock` on a branch and re-run `cargo audit` until clean or documented.

## Recording results (maintainers)

After each audit you care to snapshot for releases, append a row to the table below with the date, commit or tag, command output summary, and action taken.

| Date (UTC) | Scope | Result | Notes |
|------------|--------|--------|--------|
| _run `cargo audit` and fill in_ | workspace `Cargo.lock` | | |

The table is intentionally not auto-generated so it stays a human decision log, not a stale copy of CI output.

## Automation

You can add a scheduled GitHub Action that runs `cargo audit` and fails the job on new advisories; keep this doc as the policy reference for how failures are triaged.
