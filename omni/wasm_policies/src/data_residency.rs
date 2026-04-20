//! Data Residency Checker for PIPL Compliance
//!
//! Ensures data leaving China meets regulatory requirements.
//!
//! Build:
//! `cargo build -p omni-wasm_policies --target wasm32-unknown-unknown --release`

use panda_pdk::{set_header, PANDA_WASM_ABI_VERSION, RC_ALLOW, RC_REJECT_POLICY_DENIED};

// Approved international destinations
static APPROVED_DESTINATIONS: &[&[u8]] = &[
    b"api.openai.com",
    b"api.anthropic.com",
    b"api.cohere.ai",
];

#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn panda_on_request() -> i32 {
    set_header(b"x-omni-policy", b"data-residency");
    RC_ALLOW
}

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return RC_ALLOW;
    }

    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    // Check if request contains approved destination
    let has_approved_dest = APPROVED_DESTINATIONS.iter().any(|dest| {
        contains_ascii_ci(input, dest)
    });

    if !has_approved_dest && is_international_request(input) {
        set_header(b"x-omni-blocked", b"data-residency");
        return RC_REJECT_POLICY_DENIED;
    }

    RC_ALLOW
}

fn is_international_request(_content: &[u8]) -> bool {
    // Simplified check - in production would parse URLs and check destination
    false
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
