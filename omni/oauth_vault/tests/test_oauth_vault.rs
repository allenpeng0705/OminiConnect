//! Unit tests for oauth_vault

use omni_oauth_vault::{OAuthToken, TokenStore};
use std::sync::Arc;

#[tokio::test]
async fn test_token_store_in_memory() {
    let store = TokenStore::in_memory();

    let token = OAuthToken {
        platform: "feishu".to_string(),
        subject: "test_user".to_string(),
        access_token: "test_access_token".to_string(),
        refresh_token: Some("test_refresh_token".to_string()),
        token_type: "Bearer".to_string(),
        expires_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 + 3600,
        scopes: vec!["calendar".to_string(), "message".to_string()],
    };

    // Store token
    store.set(&token).await.unwrap();

    // Retrieve token
    let retrieved = store.get("feishu", "test_user").await.unwrap();
    assert!(retrieved.is_some());

    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.access_token, "test_access_token");
    assert_eq!(retrieved.platform, "feishu");
    assert_eq!(retrieved.subject, "test_user");

    // Delete token
    store.delete("feishu", "test_user").await.unwrap();

    // Verify deleted
    let deleted = store.get("feishu", "test_user").await.unwrap();
    assert!(deleted.is_none());
}

#[tokio::test]
async fn test_token_not_found() {
    let store = TokenStore::in_memory();

    let result = store.get("feishu", "nonexistent").await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_token_expiry_check() {
    use omni_oauth_vault::OAuthVault;

    let store = Arc::new(TokenStore::in_memory());
    let vault = OAuthVault::new(store);

    let expired_token = OAuthToken {
        platform: "feishu".to_string(),
        subject: "expired_user".to_string(),
        access_token: "expired_token".to_string(),
        refresh_token: None,
        token_type: "Bearer".to_string(),
        expires_at: 1, // Expired timestamp
        scopes: vec![],
    };

    // Token should be considered expired
    let is_expired = OAuthVault::is_token_expired(&expired_token);
    assert!(is_expired);
}
