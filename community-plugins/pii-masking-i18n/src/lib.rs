//! Heuristic PII-style masking for mixed-language UTF-8 bodies (emails, CN mobiles, ES DNI/NIE).
//!
//! Not a certified data-protection tool: tune patterns for your jurisdiction and red-team before production.

use panda_pdk::{PANDA_WASM_ABI_VERSION, RC_ALLOW, set_body, set_header};

const REDACT_EMAIL: &[u8] = b"[REDACTED_EMAIL]";
const REDACT_CN_MOBILE: &[u8] = b"[REDACTED_CN_MOBILE]";
const REDACT_ES_ID: &[u8] = b"[REDACTED_ES_ID]";

#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn panda_on_request() -> i32 {
    set_header(b"x-panda-plugin", b"community-pii-i18n");
    RC_ALLOW
}

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return RC_ALLOW;
    }
    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let mut out = Vec::with_capacity(input.len());
    let mut changed = false;
    let mut i = 0usize;

    while i < input.len() {
        if let Some(end) = match_email(input, i) {
            out.extend_from_slice(REDACT_EMAIL);
            changed = true;
            i = end;
            continue;
        }
        if let Some(end) = match_china_mobile_ascii(input, i) {
            out.extend_from_slice(REDACT_CN_MOBILE);
            changed = true;
            i = end;
            continue;
        }
        if let Some(end) = match_es_dni_nie(input, i) {
            out.extend_from_slice(REDACT_ES_ID);
            changed = true;
            i = end;
            continue;
        }
        out.push(input[i]);
        i += 1;
    }

    if changed {
        set_header(b"x-panda-pii-masked", b"i18n");
        set_body(&out);
    }
    RC_ALLOW
}

/// `user@domain.tld` (ASCII labels, pragmatic).
fn match_email(input: &[u8], i: usize) -> Option<usize> {
    let at = find_byte(input, i, b'@')?;
    if at == i || at + 1 >= input.len() {
        return None;
    }
    if !input[i..at].iter().all(|b| is_email_local(*b)) {
        return None;
    }
    let mut k = at + 1;
    let domain_start = k;
    while k < input.len() && is_domain_char(input[k]) {
        k += 1;
    }
    if k <= domain_start || k - domain_start < 3 {
        return None;
    }
    if !input[domain_start..k].contains(&b'.') {
        return None;
    }
    Some(k)
}

fn find_byte(input: &[u8], from: usize, needle: u8) -> Option<usize> {
    input[from..].iter().position(|&b| b == needle).map(|p| from + p)
}

fn is_email_local(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'+')
}

fn is_domain_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-')
}

/// Mainland-style 11 digits starting with 1 (ASCII digits in JSON/text).
fn match_china_mobile_ascii(input: &[u8], i: usize) -> Option<usize> {
    if i + 11 > input.len() {
        return None;
    }
    if !is_word_boundary_left(input, i) {
        return None;
    }
    if input[i] != b'1' || !(b'3'..=b'9').contains(&input[i + 1]) {
        return None;
    }
    if !input[i + 2..i + 11].iter().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if !is_word_boundary_right(input, i + 11) {
        return None;
    }
    Some(i + 11)
}

/// Spanish DNI `12345678A` or NIE `X1234567L` (letters checked loosely).
fn match_es_dni_nie(input: &[u8], i: usize) -> Option<usize> {
    if !is_word_boundary_left(input, i) {
        return None;
    }
    if i + 9 <= input.len() {
        let b0 = input[i];
        if matches!(b0, b'X' | b'Y' | b'Z' | b'x' | b'y' | b'z') {
            if input[i + 1..i + 8].iter().all(|b| b.is_ascii_digit()) {
                let lc = input[i + 8];
                if lc.is_ascii_alphabetic() && is_word_boundary_right(input, i + 9) {
                    return Some(i + 9);
                }
            }
        }
    }
    if i + 9 <= input.len() {
        if input[i..i + 8].iter().all(|b| b.is_ascii_digit()) {
            let lc = input[i + 8];
            if lc.is_ascii_alphabetic() && is_word_boundary_right(input, i + 9) {
                return Some(i + 9);
            }
        }
    }
    None
}

fn is_word_boundary_left(input: &[u8], i: usize) -> bool {
    if i == 0 {
        return true;
    }
    !input[i - 1].is_ascii_alphanumeric()
}

fn is_word_boundary_right(input: &[u8], j: usize) -> bool {
    j >= input.len() || !input[j].is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_span() {
        let s = b"user.name@mail.example.com";
        assert_eq!(match_email(s, 0), Some(s.len()));
    }

    #[test]
    fn cn_mobile() {
        let s = b" 13812345678 ";
        assert_eq!(match_china_mobile_ascii(s, 1), Some(12));
    }
}
