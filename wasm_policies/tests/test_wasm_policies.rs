//! Unit tests for Wasm compliance policies

use panda_pdk::{PANDA_WASM_ABI_VERSION, RC_ALLOW, RC_REJECT_POLICY_DENIED};

// Note: These tests run on host side and just verify constants

#[test]
fn test_abi_version() {
    assert_eq!(PANDA_WASM_ABI_VERSION, 1);
}

#[test]
fn test_return_codes() {
    assert_eq!(RC_ALLOW, 0);
    assert_eq!(RC_REJECT_POLICY_DENIED, 1);
}

// Keyword filter tests
#[test]
fn test_keyword_filter_blocked_words() {
    // Test content contains "机密" (classified)
    let test_content = "这是一份机密文件";
    let content_bytes = test_content.as_bytes();

    // Check that "机密" is present
    let keyword = "机密";
    let keyword_bytes = keyword.as_bytes();
    assert!(
        content_bytes
            .windows(keyword_bytes.len())
            .any(|w| w == keyword_bytes),
        "Keyword '{}' not found in content",
        keyword
    );
}

// PII detection test (simplified - actual regex would be in wasm)
#[test]
fn test_pii_patterns() {
    // Test Chinese ID pattern (simplified check)
    let chinese_id = b"11010119900307451X";
    assert_eq!(chinese_id.len(), 18);

    // Test phone pattern
    let phone = b"13812345678";
    assert_eq!(phone.len(), 11);
    assert!(phone[0] == b'1');
    assert!(
        phone[1] == b'3'
            || phone[1] == b'5'
            || phone[1] == b'7'
            || phone[1] == b'8'
            || phone[1] == b'9'
    );
}
