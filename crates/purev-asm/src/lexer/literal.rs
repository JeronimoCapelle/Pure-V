//! Numeric literal parsing (`0b`, `0x`, octal, and decimal with optional sign
//! and `_`).

/// Parses a signed integer literal in binary (`0b`/`0B`), hex (`0x`/`0X`),
/// octal (`0...`), or decimal.
pub(super) fn parse_literal(raw: &str) -> Option<i128> {
    let (neg, rest) = if let Some(stripped) = raw.strip_prefix('-') {
        (true, stripped)
    } else if let Some(stripped) = raw.strip_prefix('+') {
        (false, stripped)
    } else {
        (false, raw)
    };

    if rest.is_empty() {
        return None;
    }

    let (radix, digits) =
        if let Some(bin) = rest.strip_prefix("0b").or_else(|| rest.strip_prefix("0B")) {
            (2, bin)
        } else if let Some(hex) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")) {
            (16, hex)
        } else if let Some(oct) = rest.strip_prefix('0') {
            if oct.is_empty() || oct.bytes().all(|b| b == b'_') {
                return Some(0);
            }
            (8, oct)
        } else {
            (10, rest)
        };

    let unsigned = parse_digits_with_underscores(digits, radix)?;
    if neg {
        unsigned.checked_neg()
    } else {
        Some(unsigned)
    }
}

fn parse_digits_with_underscores(digits: &str, radix: u32) -> Option<i128> {
    if !digits.contains('_') {
        return i128::from_str_radix(digits, radix).ok();
    }

    let mut acc: i128 = 0;
    let mut saw_digit = false;
    for ch in digits.chars() {
        if ch == '_' {
            continue;
        }
        let digit = ch.to_digit(radix)?;
        saw_digit = true;
        acc = acc
            .checked_mul(i128::from(radix))?
            .checked_add(i128::from(digit))?;
    }

    saw_digit.then_some(acc)
}
