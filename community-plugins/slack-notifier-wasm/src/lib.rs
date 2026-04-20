//! Flags requests that look "expensive" so an external system can notify Slack (or PagerDuty, etc.).
//!
//! Wasm cannot perform HTTP; this module only sets headers. Pair with `relay/slack_stdin_relay.py`
//! or your log pipeline.

use panda_pdk::{PANDA_WASM_ABI_VERSION, RC_ALLOW, set_header};

/// Bodies larger than this trigger a flag (tunable constant for guest builds).
const LARGE_BODY_BYTES: usize = 96 * 1024;
/// If JSON contains max_tokens above this, flag (heuristic scan, not a full parser).
const HIGH_MAX_TOKENS: u64 = 16_384;

#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn panda_on_request() -> i32 {
    set_header(b"x-panda-plugin", b"community-slack-notify");
    RC_ALLOW
}

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return RC_ALLOW;
    }
    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let mut reason: Option<&'static [u8]> = None;

    if input.len() >= LARGE_BODY_BYTES {
        reason = Some(b"large_body");
    } else if let Some(v) = scan_max_tokens(input) {
        if v >= HIGH_MAX_TOKENS {
            reason = Some(b"high_max_tokens");
        }
    }

    if let Some(r) = reason {
        set_header(b"x-panda-slack-notify", b"high-cost");
        set_header(b"x-panda-slack-reason", r);
        // Short numeric hint for log parsers (ASCII size only).
        let mut buf = [0u8; 24];
        let n = input.len().min(999_999_999);
        let s = format_compact_usize(n, &mut buf);
        set_header(b"x-panda-slack-body-bytes", s);
    }

    RC_ALLOW
}

/// Best-effort: find `"max_tokens"` / `max_tokens` and read following integer.
fn scan_max_tokens(input: &[u8]) -> Option<u64> {
    let needle = b"max_tokens";
    let mut i = 0usize;
    while i + needle.len() <= input.len() {
        if input[i..i + needle.len()].eq_ignore_ascii_case(needle) {
            let mut j = i + needle.len();
            while j < input.len() && is_json_ws_or_colon(input[j]) {
                j += 1;
            }
            return parse_u64(&input[j..]);
        }
        i += 1;
    }
    None
}

fn is_json_ws_or_colon(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | b':')
}

fn parse_u64(rest: &[u8]) -> Option<u64> {
    let mut k = 0usize;
    while k < rest.len() && rest[k] == b' ' {
        k += 1;
    }
    if k < rest.len() && rest[k] == b'"' {
        k += 1;
    }
    let start = k;
    while k < rest.len() && rest[k].is_ascii_digit() {
        k += 1;
    }
    if k == start {
        return None;
    }
    let mut v: u64 = 0;
    for &d in &rest[start..k] {
        v = v.saturating_mul(10).saturating_add((d - b'0') as u64);
    }
    Some(v)
}

fn format_compact_usize(n: usize, buf: &mut [u8; 24]) -> &[u8] {
    if n == 0 {
        buf[0] = b'0';
        return &buf[..1];
    }
    let mut x = n;
    let mut tmp = [0u8; 24];
    let mut i = tmp.len();
    while x > 0 {
        i -= 1;
        tmp[i] = b'0' + (x % 10) as u8;
        x /= 10;
    }
    let len = tmp.len() - i;
    buf[..len].copy_from_slice(&tmp[i..]);
    &buf[..len]
}
