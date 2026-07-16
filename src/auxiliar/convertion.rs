use std::num::ParseIntError;

use crate::auxiliar::error::{AssemblerError, Stage::MathematicalProof};

pub const fn i128_to_u8(x: i128) -> Result<u8, AssemblerError> {
    if x < (u8::MIN as i128) || x > (u8::MAX as i128) {
        return Err(AssemblerError::internal(MathematicalProof));
    }
    Ok(x as u8)
}

pub const fn i128_to_i16(x: i128) -> Result<i16, AssemblerError> {
    if x < (i16::MIN as i128) || x > (i16::MAX as i128) {
        return Err(AssemblerError::internal(MathematicalProof));
    }
    Ok(x as i16)
}

pub const fn i128_to_i32(x: i128) -> Result<i32, AssemblerError> {
    if x < (i32::MIN as i128) || x > (i32::MAX as i128) {
        return Err(AssemblerError::internal(MathematicalProof));
    }
    Ok(x as i32)
}

// ----

pub fn interpret_literal(value: &str) -> Result<i128, ParseIntError> {
    let _ = match value.strip_prefix("0b") {
        Some(a) => match i128::from_str_radix(a, 2) {
            Ok(a) => return Ok(a),
            Err(a) => return Err(a),
        },
        None => 0,
    };

    let _ = match value.strip_prefix("0x") {
        Some(a) => match i128::from_str_radix(a, 16) {
            Ok(a) => return Ok(a),
            Err(a) => return Err(a),
        },
        _ => 0,
    };

    let _ = match value.strip_prefix("0") {
        Some(a) => match i128::from_str_radix(a, 8) {
            Ok(a) => return Ok(a),
            Err(a) => return Err(a),
        },
        _ => 0,
    };

    value.parse()
}
