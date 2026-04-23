//! Keyword Filter for Chinese Enterprise Compliance
//!
//! Blocks outgoing messages containing restricted keywords.
//!
//! Build:
//! `cargo build -p omini-connect-wasm-policies --target wasm32-unknown-unknown --release`

use panda_pdk::{set_header, PANDA_WASM_ABI_VERSION, RC_ALLOW, RC_REJECT_POLICY_DENIED};

// Common restricted keywords (configurable in production)
// Using string literals to support UTF-8 Chinese characters
static BLOCKED_KEYWORDS: &[&str] = &[
    "机密",     // classified
    "秘密",     // secret
    "绝密",     // top secret
    "反动",     // reactionary
    "暴力",     // violence
    "赌博",     // gambling
    "毒品",     // drugs
    "诈骗",     // fraud
    "邪教",     // cult
    "分裂",     // separatism
    "恐怖",     // terrorism
];

#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn panda_on_request() -> i32 {
    set_header(b"x-omini-connect-policy", b"keyword-filter");
    RC_ALLOW
}

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return RC_ALLOW;
    }

    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    if contains_blocked_keyword(input) {
        set_header(b"x-omini-connect-blocked", b"keyword");
        return RC_REJECT_POLICY_DENIED;
    }

    RC_ALLOW
}

fn contains_blocked_keyword(content: &[u8]) -> bool {
    let content_str = std::str::from_utf8(content).ok();

    for keyword in BLOCKED_KEYWORDS {
        if content_str.map_or(false, |s| s.contains(keyword)) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_blocking() {
        assert!(contains_blocked_keyword("这是机密信息".as_bytes()));
        assert!(contains_blocked_keyword("包含秘密内容".as_bytes()));
    }
}
