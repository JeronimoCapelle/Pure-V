//! Module containing functions for parsing strings to numerics
use core::num::ParseIntError;

/// Utility function for parsing string to numeric from different bases, decimal, octal, hex, and binary
pub fn interpret_literal(value: &str) -> Result<i128, ParseIntError> {
    let _ = match value.strip_prefix("0b") {
        Some(a) => {
            let a = i128::from_str_radix(a, 2)?;
            return Ok(a);
        }
        None => 0,
    };

    let _ = match value.strip_prefix("0x") {
        Some(a) => {
            let a = i128::from_str_radix(a, 16)?;
            return Ok(a);
        }
        _ => 0,
    };

    let _ = match value.strip_prefix("0") {
        Some(a) => {
            if a.is_empty() {
                0
            } else {
                {
                    let a = i128::from_str_radix(a, 8)?;
                    return Ok(a);
                }
            }
        }
        _ => 0,
    };

    value.parse()
}
