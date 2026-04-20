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
    let blocked_keywords = [
        "机密",
        "秘密",
        "绝密",
    ];

    // These keywords should be blocked
    let test_content = b"这是一份机密文件";
    let content_lower: Vec<u8> = test_content.iter().map(|c| c.to_ascii_lowercase()).collect();

    for keyword in blocked_keywords.iter() {
        let keyword_lower: Vec<u8> = keyword.as_bytes().iter().map(|c| c.to_ascii_lowercase()).collect();
        assert!(content_lower.windows(keyword_lower.len()).any(|w| w == &keyword_lower[..]));
    }
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
    assert!(phone[1] == b'3' || phone[1] == b'5' || phone[1] == b'7' || phone[1] == b'8' || phone[1] == b'9');
}
