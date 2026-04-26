//! Per-user connector identity helpers (catalog entry key + OAuth vault storage key).

use crate::oauth::models::ConnectorConfig;

/// Logical catalog / built-in entry key: one active connector per owner per key.
pub fn catalog_entry_key(engine: &str, platform: &str, provider_key: &str) -> String {
    let e = engine.trim();
    let pk = provider_key.trim();
    if e == "nango" {
        pk.split("__")
            .next()
            .unwrap_or(pk)
            .trim()
            .to_lowercase()
    } else {
        platform.trim().to_lowercase()
    }
}

/// Stable key for [`omini_connect_oauth_vault::OAuthVault`] token rows for this portal user + connector.
pub fn oauth_vault_platform_key(owner_username: &str, connector_platform: &str) -> String {
    let o = sanitize_key_segment(owner_username);
    let p = sanitize_key_segment(connector_platform);
    format!("{o}__{p}")
}

fn sanitize_key_segment(s: &str) -> String {
    if s.is_empty() {
        return "_".to_string();
    }
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Returns a conflicting connector `platform` id for the same owner + entry key, if any.
pub fn find_duplicate_platform_for_entry<'a>(
    owner: &str,
    exclude_platform: &str,
    engine: &str,
    platform: &str,
    provider_key: &str,
    existing: &'a [ConnectorConfig],
) -> Option<&'a str> {
    let want = catalog_entry_key(engine, platform, provider_key);
    for c in existing {
        if c.owner_username != owner {
            continue;
        }
        if c.platform == exclude_platform {
            continue;
        }
        let pk = if c.provider_key.trim().is_empty() {
            c.platform.as_str()
        } else {
            c.provider_key.as_str()
        };
        if catalog_entry_key(&c.engine, &c.platform, pk) == want {
            return Some(c.platform.as_str());
        }
    }
    None
}
