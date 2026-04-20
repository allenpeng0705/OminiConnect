//! Minimal PII-style plugin using [`panda_pdk`] helpers (~25 lines of logic).
//!
//! Build from repo root:
//! `cargo build -p wasm-plugin-pii-mini --target wasm32-unknown-unknown --release`
//!
//! Or from this directory:
//! `cargo build --target wasm32-unknown-unknown --release`

use panda_pdk::{
    guest_bytes, redact_sequential, set_body, set_header, PANDA_WASM_ABI_VERSION, RC_ALLOW,
};

#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn panda_on_request() -> i32 {
    set_header(b"x-panda-plugin", b"pii-mini");
    RC_ALLOW
}

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    let Some(input) = (unsafe { guest_bytes(ptr, len) }) else {
        return RC_ALLOW;
    };
    let rules: &[(&[u8], &[u8])] = &[
        (b"sk_live_", b"[REDACTED_SK]"),
        (b"sk_test_", b"[REDACTED_SK]"),
        (b"password", b"[REDACTED]"),
    ];
    let (out, changed) = redact_sequential(input, rules);
    if changed {
        set_header(b"x-panda-pii", b"masked");
        set_body(&out);
    }
    RC_ALLOW
}
