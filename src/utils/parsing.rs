//! Module containing functions for parsing strings to numerics
use core::num::ParseIntError;
use core::ops::Neg;

/// Utility function for parsing string to numeric from different bases, decimal, octal, hex, and binary
pub fn interpret_literal(value: &str) -> Result<i128, ParseIntError> {
    let neg: bool;

    let value = if let Some(a) = value.strip_prefix("-") {
        neg = true;
        a
    } else {
        neg = false;
        value
    };

    let _ = match value.strip_prefix("0b") {
        Some(a) => {
            let a = i128::from_str_radix(a, 2)?;
            return Ok(if neg { a.neg() } else { a });
        }
        None => 0,
    };

    let _ = match value.strip_prefix("0x") {
        Some(a) => {
            let a = i128::from_str_radix(a, 16)?;
            return Ok(if neg { a.neg() } else { a });
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
                    return Ok(if neg { a.neg() } else { a });
                }
            }
        }
        _ => 0,
    };

    match value.parse::<i128>() {
        Ok(a) => Ok(if neg { a.neg() } else { a }),
        Err(a) => Err(a),
    }
}
