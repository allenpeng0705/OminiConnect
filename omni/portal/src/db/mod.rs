//! In-memory database — users and API keys.

use std::collections::HashMap;

use chrono::Utc;

/// In-memory user store.
#[derive(Debug)]
pub struct UserStore {
    users: HashMap<String, crate::auth::models::User>,
}

impl UserStore {
    pub fn new() -> Self {
        Self { users: HashMap::new() }
    }

    pub fn get(&self, username: &str) -> Option<&crate::auth::models::User> {
        self.users.get(username)
    }

    pub fn insert(&mut self, user: crate::auth::models::User) {
        self.users.insert(user.username.clone(), user);
    }
}

impl Default for UserStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Seed the default admin user if not exists.
pub async fn seed_admin_user(state: &crate::app::AppState) {
    let mut users = state.users.write().await;
    if users.get("admin").is_none() {
        let hash = bcrypt::hash("admin", bcrypt::DEFAULT_COST).unwrap_or_default();
        users.insert(crate::auth::models::User {
            username: "admin".to_string(),
            password_hash: hash,
            created_at: Utc::now(),
        });
        tracing::info!("Seeded default admin user (username=admin, password=admin)");
    }
}
