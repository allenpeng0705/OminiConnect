# OminiConnect + Nango — prefer native (no Docker). Docker helpers are optional.
.PHONY: dev dev-omini-connect-nango dev-nango-docker dev-nango-docker-up

# Default: Nango from source + Postgres on localhost + omini-connect-portal (see scripts/dev_omini_connect_nango_native.sh).
dev: dev-omini-connect-nango

dev-omini-connect-nango:
	@./scripts/dev_omini_connect_nango_native.sh

# Optional: Nango via official Docker Compose (requires Docker Desktop).
dev-nango-docker:
	@./scripts/dev_with_nango.sh

dev-nango-docker-up:
	@./scripts/dev_with_nango.sh --nango-only
