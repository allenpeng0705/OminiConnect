//! Minimal guest module for Panda ABI v0.
//! Build: `cargo build -p wasm-plugin-sample --target wasm32-unknown-unknown --release`

use panda_pdk::{set_body, set_header, set_response_chunk, PANDA_WASM_ABI_VERSION, RC_ALLOW};

/// Must match `panda_wasm::PANDA_WASM_ABI_VERSION` (duplicated to avoid a host dep in the guest).
#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

/// Optional smoke export called by host startup.
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a.saturating_add(b)
}

/// Request hook: inject a marker header via host import.
#[no_mangle]
pub extern "C" fn panda_on_request() -> i32 {
    set_header(b"x-panda-plugin", b"sample-rust");
    RC_ALLOW
}

/// Body hook: replace ASCII `password` with `********` when present.
#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return 0;
    }
    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let needle = b"password";
    if !input.windows(needle.len()).any(|w| w == needle) {
        return 0;
    }

    let mut out = Vec::with_capacity(input.len());
    let mut i = 0usize;
    while i < input.len() {
        if i + needle.len() <= input.len() && &input[i..i + needle.len()] == needle {
            out.extend_from_slice(b"********");
            i += needle.len();
        } else {
            out.push(input[i]);
            i += 1;
        }
    }
    set_header(b"x-panda-redacted", b"true");
    set_body(&out);
    RC_ALLOW
}

/// Streaming response chunk hook (ABI v1): mask `secret` in each chunk.
#[no_mangle]
pub extern "C" fn panda_on_response_chunk(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return RC_ALLOW;
    }
    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let needle = b"secret";
    if !input.windows(needle.len()).any(|w| w == needle) {
        return RC_ALLOW;
    }
    let mut out = Vec::with_capacity(input.len());
    let mut i = 0usize;
    while i < input.len() {
        if i + needle.len() <= input.len() && &input[i..i + needle.len()] == needle {
            out.extend_from_slice(b"******");
            i += needle.len();
        } else {
            out.push(input[i]);
            i += 1;
        }
    }
    set_response_chunk(&out);
    RC_ALLOW
}
