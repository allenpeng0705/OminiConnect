//! Panda guest-side PDK (Plugin Development Kit).
//!
//! Build guests with `cargo build --target wasm32-unknown-unknown` (crate-type `cdylib`).
//!
//! # ABI v1 (summary)
//! - Export `panda_abi_version() -> i32` → [`PANDA_WASM_ABI_VERSION`].
//! - Optional: `panda_on_request() -> i32`, `panda_on_request_body(ptr, len) -> i32`,
//!   `panda_on_response_chunk(ptr, len) -> i32`.
//! - Imports: `panda_set_header`, `panda_set_body`, `panda_set_response_chunk` (see [`set_header`], etc.).
//!
//! Return codes: [`RC_ALLOW`], [`RC_REJECT_POLICY_DENIED`], [`RC_REJECT_MALFORMED_REQUEST`].
//!
//! # Minimal “PII masker” (request body)
//! ```ignore
//! use panda_pdk::{guest_bytes, replace_all_copy, set_body, PANDA_WASM_ABI_VERSION, RC_ALLOW};
//!
//! #[no_mangle]
//! pub extern "C" fn panda_abi_version() -> i32 { PANDA_WASM_ABI_VERSION }
//!
//! #[no_mangle]
//! pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
//!     let Some(buf) = (unsafe { guest_bytes(ptr, len) }) else { return RC_ALLOW };
//!     let mut out = buf.to_vec();
//!     if let Some(patched) = replace_all_copy(&out, b"sk_live_", b"[REDACTED]") { out = patched; }
//!     if out != buf { set_body(&out); }
//!     RC_ALLOW
//! }
//! ```
//!
//! - **Rust example guest:** `rust/pii_mini/` (same repo workspace package `wasm-plugin-pii-mini`).
//! - **Go (TinyGo):** `go/panda/` + `go/examples/pii_mask/`. See `go/README.md`.

pub const PANDA_WASM_ABI_VERSION: i32 = 1;
pub const RC_ALLOW: i32 = 0;
pub const RC_REJECT_POLICY_DENIED: i32 = 1;
pub const RC_REJECT_MALFORMED_REQUEST: i32 = 2;

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn panda_set_header(name_ptr: i32, name_len: i32, value_ptr: i32, value_len: i32);
    fn panda_set_body(ptr: i32, len: i32);
    fn panda_set_response_chunk(ptr: i32, len: i32);
}

/// Host callback: append request header.
#[cfg(target_arch = "wasm32")]
pub fn set_header(name: &[u8], value: &[u8]) {
    unsafe {
        panda_set_header(
            name.as_ptr() as i32,
            name.len() as i32,
            value.as_ptr() as i32,
            value.len() as i32,
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_header(_name: &[u8], _value: &[u8]) {}

/// Host callback: replace request body.
#[cfg(target_arch = "wasm32")]
pub fn set_body(body: &[u8]) {
    unsafe {
        panda_set_body(body.as_ptr() as i32, body.len() as i32);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_body(_body: &[u8]) {}

/// Host callback: replace current response chunk.
#[cfg(target_arch = "wasm32")]
pub fn set_response_chunk(chunk: &[u8]) {
    unsafe {
        panda_set_response_chunk(chunk.as_ptr() as i32, chunk.len() as i32);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_response_chunk(_chunk: &[u8]) {}

/// Interpret `(ptr, len)` from `panda_on_request_body` / `panda_on_response_chunk` as a guest slice.
///
/// # Safety
/// `ptr` must point into the Wasm linear memory buffer the host read from, valid for `len` bytes
/// for the duration of the hook call.
#[inline]
pub unsafe fn guest_bytes<'a>(ptr: i32, len: i32) -> Option<&'a [u8]> {
    if ptr < 0 || len <= 0 {
        return None;
    }
    let len = len as usize;
    Some(std::slice::from_raw_parts(ptr as *const u8, len))
}

/// Non-overlapping left-to-right replacement of every `needle` with `replacement`.
/// Returns `None` if `needle` is empty or there is no match.
pub fn replace_all_copy(data: &[u8], needle: &[u8], replacement: &[u8]) -> Option<Vec<u8>> {
    if needle.is_empty() {
        return None;
    }
    if !data.windows(needle.len()).any(|w| w == needle) {
        return None;
    }
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0usize;
    while i < data.len() {
        if i + needle.len() <= data.len() && data[i..i + needle.len()] == *needle {
            out.extend_from_slice(replacement);
            i += needle.len();
        } else {
            out.push(data[i]);
            i += 1;
        }
    }
    Some(out)
}

/// Apply several redactions in order; returns the final buffer and whether anything changed.
pub fn redact_sequential(data: &[u8], rules: &[(&[u8], &[u8])]) -> (Vec<u8>, bool) {
    let mut cur = data.to_vec();
    let orig = cur.clone();
    for (needle, replacement) in rules {
        if let Some(next) = replace_all_copy(&cur, needle, replacement) {
            cur = next;
        }
    }
    let changed = cur != orig;
    (cur, changed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_all_copy_basic() {
        let s = b"hello sk_live_abc and sk_live_def";
        let out = replace_all_copy(s, b"sk_live_", b"X").unwrap();
        assert_eq!(&out[..], b"hello Xabc and Xdef");
    }

    #[test]
    fn redact_sequential_stacks() {
        let (v, ch) = redact_sequential(b"a password b", &[(b"password", b"[P]")]);
        assert!(ch);
        assert_eq!(v.as_slice(), b"a [P] b");
    }
}
