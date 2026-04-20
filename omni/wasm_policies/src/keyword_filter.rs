//! Keyword Filter for Chinese Enterprise Compliance
//!
//! Blocks outgoing messages containing restricted keywords.
//!
//! Build:
//! `cargo build -p omni-wasm_policies --target wasm32-unknown-unknown --release`

use panda_pdk::{set_header, PANDA_WASM_ABI_VERSION, RC_ALLOW, RC_REJECT_POLICY_DENIED};

// Common restricted keywords (configurable in production)
static BLOCKED_KEYWORDS: &[&[u8]] = &[
    b"机密",
    b"秘密",
    b"绝密",
    b"反动",
    b"暴力",
    b"赌博",
    b"毒品",
    b"诈骗",
    b"邪教",
    b"分裂",
    b"恐怖",
];

#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn panda_on_request() -> i32 {
    set_header(b"x-omni-policy", b"keyword-filter");
    RC_ALLOW
}

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return RC_ALLOW;
    }

    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    if contains_blocked_keyword(input) {
        set_header(b"x-omni-blocked", b"keyword");
        return RC_REJECT_POLICY_DENIED;
    }

    RC_ALLOW
}

fn contains_blocked_keyword(content: &[u8]) -> bool {
    let content_lower = to_lower(content);

    for keyword in BLOCKED_KEYWORDS {
        if contains_ascii_ci(&content_lower, keyword) {
            return true;
        }
    }
    false
}

fn to_lower(input: &[u8]) -> Vec<u8> {
    input.iter().map(|c| c.to_ascii_lowercase()).collect()
}

fn contains_ascii_ci(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return false;
    }

    'outer: for i in 0..=haystack.len().saturating_sub(needle.len()) {
        for j in 0..needle.len() {
            if haystack[i + j].to_ascii_lowercase() != needle[j].to_ascii_lowercase() {
                continue 'outer;
            }
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_blocking() {
        assert!(contains_blocked_keyword(b"这是机密信息"));
        assert!(contains_blocked_keyword(b"包含秘密内容"));
    }
}
