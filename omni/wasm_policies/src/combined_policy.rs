//! Combined Compliance Policy for OmniConnect
//!
//! This module combines PII scrubbing, keyword filtering, and data residency checks
//! into a single Wasm plugin for Panda's plugin system.

use panda_pdk::{set_body, set_header, PANDA_WASM_ABI_VERSION, RC_ALLOW, RC_REJECT_POLICY_DENIED};

// ============================================================================
// Keyword Filter
// ============================================================================

// Common restricted keywords (configurable in production)
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

fn contains_blocked_keyword(content: &[u8]) -> bool {
    let content_str = std::str::from_utf8(content).ok();

    for keyword in BLOCKED_KEYWORDS {
        if content_str.map_or(false, |s| s.contains(keyword)) {
            return true;
        }
    }
    false
}

// ============================================================================
// PII Scrubber
// ============================================================================

fn is_digit(b: u8) -> bool {
    b >= b'0' && b <= b'9'
}

fn is_valid_id_char(b: u8) -> bool {
    is_digit(b) || b == b'X' || b == b'x'
}

/// Scan for Chinese ID numbers (18 digits, last can be X)
/// Returns redacted version if found
fn redact_chinese_id(input: &[u8]) -> Option<Vec<u8>> {
    let mut result = input.to_vec();
    let mut changed = false;
    let mut i = 0;

    while i <= result.len().saturating_sub(18) {
        // Check if we have 17 digits followed by valid 18th char
        if i + 17 < result.len() {
            let first_17_digits = &result[i..i + 17];
            let all_17_digits = first_17_digits.iter().all(|&b| is_digit(b));

            if all_17_digits {
                let remaining_char = result[i + 17];
                if is_valid_id_char(remaining_char) {
                    // Found a Chinese ID - redact it
                    let replacement = b"[CHINESE_ID]";
                    result.splice(i..i + 18, replacement.iter().copied());
                    changed = true;
                    // Skip past the replacement we just inserted
                    i += replacement.len();
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

/// Apply all PII redactions
fn redact_pii(input: &[u8]) -> Vec<u8> {
    let mut output = input.to_vec();

    if let Some(patched) = redact_chinese_id(&output) {
        output = patched;
    }

    if let Some(patched) = redact_phone(&output) {
        output = patched;
    }

    if let Some(patched) = redact_email(&output) {
        output = patched;
    }

    output
}

// ============================================================================
// Data Residency
// ============================================================================

/// Check if request appears to be international (non-Chinese)
fn is_international_request(_content: &[u8]) -> bool {
    // In production, this would check:
    // - Request headers for geographic indicators
    // - API endpoint targets
    // - User/org region settings
    // For now, always return false (assume domestic)
    false
}

// ============================================================================
// Panda Plugin Exports
// ============================================================================

#[no_mangle]
pub extern "C" fn panda_abi_version() -> i32 {
    PANDA_WASM_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn panda_on_request() -> i32 {
    set_header(b"x-omni-policy", b"combined-compliance");
    RC_ALLOW
}

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return RC_ALLOW;
    }

    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    // Step 1: Check for blocked keywords (reject if found)
    if contains_blocked_keyword(input) {
        set_header(b"x-omni-blocked", b"keyword");
        return RC_REJECT_POLICY_DENIED;
    }

    // Step 2: Check for data residency issues (reject if international)
    if is_international_request(input) {
        set_header(b"x-omni-blocked", b"data-residency");
        return RC_REJECT_POLICY_DENIED;
    }

    // Step 3: Redact PII (continue even if redaction happens)
    let output = redact_pii(input);
    if output != input {
        set_body(&output);
    }

    RC_ALLOW
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_blocking() {
        assert!(contains_blocked_keyword("这是机密信息".as_bytes()));
        assert!(contains_blocked_keyword("包含秘密内容".as_bytes()));
        assert!(!contains_blocked_keyword("这是正常内容".as_bytes()));
    }

    #[test]
    fn test_chinese_id_redaction() {
        let input = b"My ID is 110101199003078935 and I'm verified";
        let result = redact_chinese_id(input);
        assert!(result.is_some());
        let redacted = result.unwrap();
        // Use windows to check for substring - [CHINESE_ID] is 12 characters
        assert!(redacted.windows(12).any(|w| w == b"[CHINESE_ID]"), "Did not find [CHINESE_ID] in redacted");
        assert!(!redacted.windows(18).any(|w| w == b"110101199003078935"), "Original ID still present");
    }

    #[test]
    fn test_phone_redaction() {
        let input = b"Contact me at 13812345678 please";
        let result = redact_phone(input);
        assert!(result.is_some());
        let redacted = result.unwrap();
        assert!(redacted.windows(7).any(|w| w == b"[PHONE]"));
        assert!(!redacted.windows(11).any(|w| w == b"13812345678"));
    }

    #[test]
    fn test_email_redaction() {
        let input = b"Email me at user@example.com for details";
        let result = redact_email(input);
        assert!(result.is_some());
        let redacted = result.unwrap();
        assert!(redacted.windows(7).any(|w| w == b"[EMAIL]"));
        assert!(!redacted.windows(16).any(|w| w == b"user@example.com"));
    }

    #[test]
    fn test_combined_policy_all_pass() {
        let input = b"Hello, this is normal content with no issues.";
        assert_eq!(contains_blocked_keyword(input), false);
        assert_eq!(is_international_request(input), false);
        let redacted = redact_pii(input);
        assert_eq!(&redacted[..], &input[..]);
    }
}
