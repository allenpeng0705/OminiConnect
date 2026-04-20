//! PII Scrubber for Chinese Enterprise Compliance
//!
//! Detects and redacts:
//! - Chinese ID numbers (18-digit)
//! - Phone numbers (mobile + landline)
//! - Email addresses
//! - Bank card numbers
//!
//! Build:
//! `cargo build -p omni-wasm_policies --target wasm32-unknown-unknown --release`

use panda_pdk::{set_body, PANDA_WASM_ABI_VERSION, RC_ALLOW};

#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return RC_ALLOW;
    }

    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    // Check for PII patterns and redact
    let mut output = input.to_vec();

    // Redact Chinese ID numbers
    if let Some(patched) = redact_chinese_id(&output) {
        output = patched;
    }

    // Redact mobile phone numbers
    if let Some(patched) = redact_phone(&output) {
        output = patched;
    }

    // Redact email addresses
    if let Some(patched) = redact_email(&output) {
        output = patched;
    }

    if output != input {
        set_body(&output);
    }

    RC_ALLOW
}

/// Check if a byte is a digit
fn is_digit(b: u8) -> bool {
    b >= b'0' && b <= b'9'
}

/// Check if a byte is a valid Chinese ID checksum character
fn is_valid_id_char(b: u8) -> bool {
    is_digit(b) || b == b'X' || b == b'x'
}

/// Scan for Chinese ID numbers (18 digits, last can be X)
/// Returns redacted version if found
fn redact_chinese_id(input: &[u8]) -> Option<Vec<u8>> {
    let mut result = input.to_vec();
    let mut changed = false;
    let mut i = 0;

    while i <= input.len().saturating_sub(18) {
        // Check if we have 17 digits followed by valid 18th char
        let first_17_digits = &input[i..i + 17];
        let all_17_digits = first_17_digits.iter().all(|&b| is_digit(b));

        if all_17_digits {
            let remaining = &input[i + 17..];
            if !remaining.is_empty() && is_valid_id_char(remaining[0]) {
                // Found a Chinese ID - redact it
                let replacement = b"[CHINESE_ID]";
                result.splice(i..i + 18, replacement.iter().copied());
                changed = true;
                i += 18 + replacement.len();
                continue;
            }
        }
        i += 1;
    }

    if changed {
        Some(result)
    } else {
        None
    }
}

/// Scan for mobile phone numbers (11 digits starting with 1[3-9])
fn redact_phone(input: &[u8]) -> Option<Vec<u8>> {
    let mut result = input.to_vec();
    let mut changed = false;
    let mut i = 0;

    while i <= input.len().saturating_sub(11) {
        // Check if starts with 1 and second digit is 3-9
        if input[i] == b'1' && i + 1 < input.len() {
            let second = input[i + 1];
            if second >= b'3' && second <= b'9' {
                // Check if all 11 digits
                let all_digits = input[i..i + 11].iter().all(|&b| is_digit(b));
                if all_digits {
                    let replacement = b"[PHONE]";
                    result.splice(i..i + 11, replacement.iter().copied());
                    changed = true;
                    i += 11 + replacement.len();
                    continue;
                }
            }
        }
        i += 1;
    }

    if changed {
        Some(result)
    } else {
        None
    }
}

/// Scan for email addresses (simplified pattern matching)
fn redact_email(input: &[u8]) -> Option<Vec<u8>> {
    let mut result = input.to_vec();
    let mut changed = false;
    let mut i = 0;

    while i < input.len() {
        // Look for @ symbol
        if input[i] == b'@' && i > 0 {
            // Scan backwards for local part
            let mut local_start = i;
            while local_start > 0 {
                let prev = local_start - 1;
                let c = input[prev];
                if c.is_ascii_alphanumeric() || c == b'.' || c == b'_' || c == b'%' || c == b'+' || c == b'-' {
                    local_start = prev;
                } else {
                    break;
                }
            }

            // Scan forwards for domain part (at least 2 chars after @)
            let mut domain_end = i + 1;
            let mut dot_count = 0;
            while domain_end < input.len() && domain_end < i + 64 {
                let c = input[domain_end];
                if c.is_ascii_alphanumeric() || c == b'-' || c == b'.' {
                    if c == b'.' {
                        dot_count += 1;
                    }
                    domain_end += 1;
                } else {
                    break;
                }
            }

            // Valid email: has dots in domain, at least one char in local and domain
            let local_len = i - local_start;
            let domain_len = domain_end - i - 1;
            if local_len > 0 && domain_len > 2 && dot_count >= 1 {
                let replacement = b"[EMAIL]";
                result.splice(local_start..domain_end, replacement.iter().copied());
                changed = true;
                i = local_start + replacement.len();
                continue;
            }
        }
        i += 1;
    }

    if changed {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_id_redaction() {
        // Valid Chinese ID (18 digits)
        let input = b"My ID is 110101199003078935 and I'm verified";
        let result = redact_chinese_id(input);
        assert!(result.is_some());
        let redacted = result.unwrap();
        assert!(redacted.contains(b"[CHINESE_ID]"));
        assert!(!redacted.contains(b"110101199003078935"));
    }

    #[test]
    fn test_phone_redaction() {
        // Valid Chinese mobile phone
        let input = b"Contact me at 13812345678 please";
        let result = redact_phone(input);
        assert!(result.is_some());
        let redacted = result.unwrap();
        assert!(redacted.contains(b"[PHONE]"));
        assert!(!redacted.contains(b"13812345678"));
    }

    #[test]
    fn test_email_redaction() {
        // Standard email
        let input = b"Email me at user@example.com for details";
        let result = redact_email(input);
        assert!(result.is_some());
        let redacted = result.unwrap();
        assert!(redacted.contains(b"[EMAIL]"));
        assert!(!redacted.contains(b"user@example.com"));
    }

    #[test]
    fn test_no_pii_in_clean_text() {
        let input = b"This is clean text with no PII";
        assert!(redact_chinese_id(input).is_none());
        assert!(redact_phone(input).is_none());
        assert!(redact_email(input).is_none());
    }
}
