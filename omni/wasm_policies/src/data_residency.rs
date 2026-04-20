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

    if !has_approved_dest && contains_chinese_pii(input) {
        set_header(b"x-omni-blocked", b"data-residency");
        set_header(b"x-omni-pii-type", b"chinese-national-id");
        return RC_REJECT_POLICY_DENIED;
    }

    RC_ALLOW
}

/// Checks if content contains Chinese PII indicating domestic data
/// that should not be transferred internationally without approval.
fn contains_chinese_pii(content: &[u8]) -> bool {
    // Chinese mobile phone number pattern (11 digits starting with 1)
    if contains_chinese_phone(content) {
        return true;
    }

    // Chinese ID card number pattern (18 digits)
    if contains_chinese_id(content) {
        return true;
    }

    // Chinese name patterns in UTF-8 (common surname characters)
    // This is a heuristic check
    if contains_chinese_name_indicator(content) {
        return true;
    }

    false
}

/// Detect Chinese mobile phone numbers (11 digits starting with 1)
fn contains_chinese_phone(content: &[u8]) -> bool {
    let content_str = std::str::from_utf8(content).ok();

    if let Some(s) = content_str {
        // Check for 11-digit sequences that look like phone numbers
        for i in 0..s.len().saturating_sub(10) {
            let slice = &s[i..];

            // Try to match 11 consecutive digits starting with 1
            let mut digits = 0;
            let mut has_11_digits = false;

            for (j, c) in slice.char_indices() {
                if j >= 11 {
                    break;
                }
                if c.is_ascii_digit() {
                    digits += 1;
                    if digits == 11 {
                        has_11_digits = true;
                        break;
                    }
                } else {
                    digits = 0;
                }
            }

            if has_11_digits && digits == 11 {
                // Verify it starts with valid prefix (1)
                let first_digit = slice.chars().find(|c| c.is_ascii_digit());
                if let Some('1') = first_digit {
                    return true;
                }
            }
        }
    }

    // Check raw bytes for 11+ consecutive digit patterns starting with 1[3-9]
    let mut consecutive_start = None;

    for (i, &byte) in content.iter().enumerate() {
        if byte.is_ascii_digit() {
            if consecutive_start.is_none() {
                consecutive_start = Some(i);
            }
        } else {
            if let Some(start) = consecutive_start {
                let len = i - start;
                if len >= 11 {
                    // Check if starts with 1 and followed by valid prefix
                    if content[start] == b'1' && i < content.len() {
                        let second = content[start + 1];
                        if second >= b'3' && second <= b'9' {
                            return true;
                        }
                    }
                }
            }
            consecutive_start = None;
        }
    }

    false
}

/// Detect Chinese ID card numbers (18 digits)
fn contains_chinese_id(content: &[u8]) -> bool {
    // Chinese ID: 18 digits, first 6 are region code, digits 7-14 are birthdate
    // Pattern: YYYYMMDD in digits 7-14
    let content_str = std::str::from_utf8(content).ok();

    if let Some(s) = content_str {
        let mut consecutive_digits = 0u8;
        let mut digit_start = 0usize;

        for (i, c) in s.char_indices() {
            if c.is_ascii_digit() {
                if consecutive_digits == 0 {
                    digit_start = i;
                }
                consecutive_digits += 1;

                if consecutive_digits == 18 {
                    // Found 18 consecutive digits - check if it could be an ID
                    let slice = &s[digit_start..i+1];
                    if looks_like_chinese_id(slice) {
                        return true;
                    }
                    // Continue searching, reset
                    consecutive_digits = 0;
                }
            } else {
                consecutive_digits = 0;
            }
        }
    }

    false
}

/// Heuristic check for Chinese name patterns
fn contains_chinese_name_indicator(content: &[u8]) -> bool {
    // Check for common Chinese surname characters appearing
    // This is a simplified heuristic
    let chinese_chars = [
        '张', '王', '李', '刘', '陈', '杨', '黄', '赵', '周', '吴',
        '徐', '孙', '马', '朱', '胡', '郭', '林', '何', '高', '梁',
        '郑', '罗', '宋', '谢', '唐', '韩', '曹', '许', '邓', '萧',
    ];

    let content_str = std::str::from_utf8(content).ok()?;

    for c in content_str.chars() {
        if chinese_chars.contains(&c) {
            // Found a Chinese character - check if it might be a name
            // by looking for adjacent Chinese characters
            let mut chinese_count = 1;

            // Look ahead
            let remaining: String = content_str[content_str.char_indices()
                .nth(0).map(|(i, _)| i).unwrap_or(0)..]
                .chars()
                .take(100) // Check up to 100 chars ahead
                .collect();

            for next_c in remaining.chars().skip(1) {
                if chinese_chars.contains(&next_c) {
                    chinese_count += 1;
                    if chinese_count >= 2 {
                        // Likely a name (2+ Chinese chars together)
                        return true;
                    }
                } else {
                    break;
                }
            }
        }
    }

    false
}

/// Check if an 18-digit string looks like a Chinese ID
fn looks_like_chinese_id(s: &str) -> bool {
    if s.len() != 18 {
        return false;
    }

    // First 6 digits should be region code (mostly numeric)
    // Digits 7-14 should be a valid date (YYYYMMDD)
    // Digit 17 should be odd for male, even for female

    let digits: Vec<u32> = s.chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != 18 {
        return false;
    }

    // Check birthdate (positions 6-13, 0-indexed)
    // Year: digits[6-9], Month: digits[10-11], Day: digits[12-13]
    let year = digits[6] * 1000 + digits[7] * 100 + digits[8] * 10 + digits[9];
    let month = digits[10] * 10 + digits[11];
    let day = digits[12] * 10 + digits[13];

    if year < 1900 || year > 2100 {
        return false;
    }
    if month < 1 || month > 12 {
        return false;
    }
    if day < 1 || day > 31 {
        return false;
    }

    true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_phone_detection() {
        // Valid Chinese phone numbers
        assert!(contains_chinese_phone(b"13812345678"));
        assert!(contains_chinese_phone(b"我的电话是13812345678"));
        assert!(contains_chinese_phone(b"139-1234-5678"));

        // Invalid phone numbers
        assert!(!contains_chinese_phone(b"12345678901")); // Starts with 12, not valid prefix
        assert!(!contains_chinese_phone(b"1234567890"));  // Only 10 digits
    }

    #[test]
    fn test_chinese_id_detection() {
        // Valid Chinese ID (1985-07-18 birthdate, male)
        assert!(contains_chinese_id(b"110101198507180015"));
        assert!(contains_chinese_id(b"身份证号110101198507180015"));

        // Invalid IDs
        assert!(!contains_chinese_id(b"123456789012345678")); // Bad date
        assert!(!contains_chinese_id(b"12345678901234567")); // Only 17 digits
    }

    #[test]
    fn test_chinese_name_detection() {
        // Common Chinese names
        assert!(contains_chinese_name_indicator("张三".as_bytes()));
        assert!(contains_chinese_name_indicator("李四先生".as_bytes()));
        assert!(contains_chinese_name_indicator("王五".as_bytes()));

        // Not Chinese names
        assert!(!contains_chinese_name_indicator(b"hello world"));
    }

    #[test]
    fn test_safe_content() {
        // Content without Chinese PII
        assert!(!contains_chinese_pii(b"Hello, this is a normal message"));
        assert!(!contains_chinese_pii(b"No sensitive data here 12345"));
    }

    #[test]
    fn test_approved_destination_allows() {
        // Request to approved destination should be allowed even with digits
        let content = b"api.openai.com request with some data";
        let has_approved = APPROVED_DESTINATIONS.iter().any(|dest| {
            contains_ascii_ci(content, dest)
        });
        assert!(has_approved);
    }
}
