//! SSRF / private-URL guard for LLM and tool request bodies.
//!
//! Scans UTF-8 bodies case-insensitively for URL patterns that often indicate SSRF or
//! exfiltration attempts (localhost, RFC1918, link-local, `file://`, legacy schemes, GCP metadata host).
//!
//! Build:
//! `cargo build -p wasm-plugin-ssrf-guard --target wasm32-unknown-unknown --release`
//! Then copy `target/wasm32-unknown-unknown/release/wasm_plugin_ssrf_guard.wasm` into `plugins/`.

use panda_pdk::{set_header, PANDA_WASM_ABI_VERSION, RC_ALLOW, RC_REJECT_POLICY_DENIED};

/// Substrings matched with ASCII case-folding (sufficient for `http://` schemes and hosts).
static BLOCKED: &[&[u8]] = &[
    b"http://127.",
    b"http://10.",
    b"http://192.168.",
    b"http://172.16.",
    b"http://172.17.",
    b"http://172.18.",
    b"http://172.19.",
    b"http://172.20.",
    b"http://172.21.",
    b"http://172.22.",
    b"http://172.23.",
    b"http://172.24.",
    b"http://172.25.",
    b"http://172.26.",
    b"http://172.27.",
    b"http://172.28.",
    b"http://172.29.",
    b"http://172.30.",
    b"http://172.31.",
    b"http://169.254.",
    b"http://0.0.0.0",
    b"http://[::1]",
    b"http://localhost",
    b"https://127.",
    b"https://10.",
    b"https://192.168.",
    b"https://localhost",
    b"file://",
    b"ftp://",
    b"gopher://",
    b"dict://",
    b"metadata.google.internal",
];

#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn panda_on_request() -> i32 {
    set_header(b"x-panda-wasm-plugin", b"ssrf-guard");
    RC_ALLOW
}

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return RC_ALLOW;
    }
    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    if body_has_blocked_pattern(input) {
        set_header(b"x-panda-ssrf-block", b"1");
        return RC_REJECT_POLICY_DENIED;
    }
    RC_ALLOW
}

fn body_has_blocked_pattern(haystack: &[u8]) -> bool {
    BLOCKED.iter().any(|pat| contains_ascii_ci(haystack, pat))
}

#[cfg(test)]
mod tests {
    use super::body_has_blocked_pattern;

    #[test]
    fn blocks_localhost_http() {
        assert!(body_has_blocked_pattern(
            b"{\"url\": \"http://localhost:8080/internal\"}"
        ));
    }

    #[test]
    fn blocks_private_ten() {
        assert!(body_has_blocked_pattern(b"fetch https://10.0.0.1/x"));
    }

    #[test]
    fn blocks_file_scheme() {
        assert!(body_has_blocked_pattern(b"path file:///etc/passwd"));
    }

    #[test]
    fn allows_public_https() {
        assert!(!body_has_blocked_pattern(
            b"{\"x\":\"https://api.openai.com/v1\"}"
        ));
    }
}

fn contains_ascii_ci(haystack: &[u8], pat: &[u8]) -> bool {
    if pat.is_empty() || pat.len() > haystack.len() {
        return false;
    }
    'outer: for i in 0..=haystack.len() - pat.len() {
        for j in 0..pat.len() {
            if haystack[i + j].to_ascii_lowercase() != pat[j].to_ascii_lowercase() {
                continue 'outer;
            }
        }
        return true;
    }
    false
}
