//! Content Moderation for Chinese Enterprise Compliance
//!
//! Scans outgoing messages for inappropriate content including:
//! - Adult/NSFW content indicators
//! - Hate speech
//! - Harmful content
//! - Harassment
//!
//! Build:
//! `cargo build -p omni-wasm_policies --target wasm32-unknown-unknown --release`

use panda_pdk::{set_header, PANDA_WASM_ABI_VERSION, RC_ALLOW, RC_REJECT_POLICY_DENIED};

/// Content category for moderation decisions
#[repr(i32)]
pub enum ContentCategory {
    Allow = 0,
    Adult = 1,
    Hate = 2,
    Harassment = 3,
    Violence = 4,
    SelfHarm = 5,
    Spam = 6,
}

/// Adult content indicators
static ADULT_KEYWORDS: &[&str] = &[
    "色情",
    "黄色",
    "裸体",
    "成人内容",
    "AV女优",
    "色情网站",
];

/// Hate speech indicators
static HATE_KEYWORDS: &[&str] = &[
    "种族歧视",
    "仇恨",
    "优越论",
];

/// Harassment indicators
static HARASSMENT_KEYWORDS: &[&str] = &[
    "侮辱",
    "人身攻击",
    "网络暴力",
    "霸凌",
];

/// Violence indicators
static VIOLENCE_KEYWORDS: &[&str] = &[
    "杀人",
    "自杀",
    "自残",
    "威胁",
    "武器",
];

#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn panda_on_request() -> i32 {
    set_header(b"x-omni-policy", b"content-moderation");
    RC_ALLOW
}

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return RC_ALLOW;
    }

    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    match check_content(input) {
        ContentCategory::Allow => RC_ALLOW,
        _ => {
            set_header(b"x-omni-blocked", b"content-moderation");
            set_header(b"x-omni-content-category", category_name(check_content(input)));
            RC_REJECT_POLICY_DENIED
        }
    }
}

fn check_content(content: &[u8]) -> ContentCategory {
    let content_str = std::str::from_utf8(content);

    if content_str.is_err() {
        return ContentCategory::Allow;
    }
    let s = content_str.unwrap();

    // Check adult content
    if contains_any(s, ADULT_KEYWORDS) {
        return ContentCategory::Adult;
    }

    // Check hate speech
    if contains_any(s, HATE_KEYWORDS) {
        return ContentCategory::Hate;
    }

    // Check harassment
    if contains_any(s, HARASSMENT_KEYWORDS) {
        return ContentCategory::Harassment;
    }

    // Check violence
    if contains_any(s, VIOLENCE_KEYWORDS) {
        return ContentCategory::Violence;
    }

    ContentCategory::Allow
}

fn contains_any(content: &str, keywords: &[&str]) -> bool {
    for keyword in keywords {
        if content.contains(keyword) {
            return true;
        }
    }
    false
}

fn category_name(category: ContentCategory) -> &'static [u8] {
    match category {
        ContentCategory::Allow => b"allow",
        ContentCategory::Adult => b"adult",
        ContentCategory::Hate => b"hate",
        ContentCategory::Harassment => b"harassment",
        ContentCategory::Violence => b"violence",
        ContentCategory::SelfHarm => b"self-harm",
        ContentCategory::Spam => b"spam",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adult_content_detection() {
        assert_eq!(
            check_content("包含色情内容".as_bytes()),
            ContentCategory::Adult
        );
    }

    #[test]
    fn test_hate_content_detection() {
        assert_eq!(
            check_content("种族歧视言论".as_bytes()),
            ContentCategory::Hate
        );
    }

    #[test]
    fn test_safe_content() {
        assert_eq!(
            check_content("这是一个正常的商务消息".as_bytes()),
            ContentCategory::Allow
        );
    }
}
