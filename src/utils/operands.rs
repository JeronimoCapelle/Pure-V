//! module containing all the operands for the instructions

use std::collections::HashMap;

use crate::utils::{
    error::SyntaxError::{
        self, BiggerValue, Internal, InvalidToken, NonExistentRegister, OddValue, SmallerValue,
        TexttoNumeric, Translation,
    },
    parsing::interpret_literal,
    token::Token,
};

/// 12-bit signed integer (range: -2048 to 2047). Used by ``IType``
#[derive(PartialEq, Eq, Debug)]
pub struct Immediate(i16);

impl Immediate {
    /// Creates an Immediate from a Literal Token, is bound checked.
    pub(crate) fn new(token: &Token) -> Result<Self, SyntaxError> {
        let max = 2047;
        let min = -2048;

        let Token::Literal(text) = token else {
            return Err(InvalidToken(token.clone()));
        };

        let Ok(value) = interpret_literal(text) else {
            return Err(TexttoNumeric(text.to_owned()));
        };

        if value > max {
            return Err(BiggerValue(max, value));
        }

        if value < min {
            return Err(SmallerValue(min, value));
        }

        let value = i16::try_from(value).map_err(|_| SyntaxError::Internal)?;

        Ok(Self(value))
    }
    /// Encodes the Immediate value into a 12-bit signed integer
    pub(crate) fn encode(&self) -> u32 {
        (u32::from(self.0.cast_unsigned())) & 0b1111_1111_1111
    }
}

/// 5-bit unsigned integer (range: 0 to 31 for 32-bit registers). Used by ``ITypeShifts``
#[derive(PartialEq, Eq, Debug)]
pub struct Shamt(u8);

impl Shamt {
    /// Creates an Shamt from a Literal Token, is bound checked.
    pub(crate) fn new(token: &Token) -> Result<Self, SyntaxError> {
        let max = 31;
        let min = 0;

        let Token::Literal(text) = token else {
            return Err(InvalidToken(token.clone()));
        };

        let Ok(value) = interpret_literal(text) else {
            return Err(TexttoNumeric(text.to_owned()));
        };

        if value > max {
            return Err(BiggerValue(max, value));
        }

        if value < min {
            return Err(SmallerValue(min, value));
        }

        let value = u8::try_from(value).map_err(|_| SyntaxError::Internal)?;

        Ok(Self(value))
    }
    /// Encodes the Immediate value into a 5-bit unsigned integer
    pub(crate) fn encode(&self) -> u32 {
        u32::from(self.0)
    }
}
///12-bit signed immediate offset (range: -2048 to 2047 bytes). Used by ``ITypeMemory``, ``STypeMemory`` and ``ITypeJump``
#[derive(PartialEq, Eq, Debug)]
pub struct Offset(i16);

impl Offset {
    /// Creates an Offset from a Literal Token, is bound checked.
    pub(crate) fn new(token: &Token) -> Result<Self, SyntaxError> {
        let max = 2047;
        let min = -2048;

        let Token::Literal(text) = token else {
            return Err(InvalidToken(token.clone()));
        };

        let Ok(value) = interpret_literal(text) else {
            return Err(TexttoNumeric(text.to_owned()));
        };

        if value > max {
            return Err(BiggerValue(max, value));
        }

        if value < min {
            return Err(SmallerValue(min, value));
        }

        let value = i16::try_from(value).map_err(|_| SyntaxError::Internal)?;

        Ok(Self(value))
    }
    /// Encodes the Offset value into a 12-bit signed integer
    pub(crate) fn encode(&self) -> u32 {
        (u32::from(self.0.cast_unsigned())) & 0b1111_1111_1111
    }
}
///12-bit signed PC-relative offset, multiple of 2 bytes. Used by ``BType``
#[derive(PartialEq, Eq, Debug)]
pub struct BLabel(i16);

impl BLabel {
    /// Creates a ``BLabel`` from an Identifier Token and a ``symbol_table``, is bound checked.
    pub(crate) fn new(
        token: &Token,
        symbol_table: &HashMap<String, usize>,
        current_pc: usize,
    ) -> Result<Self, SyntaxError> {
        let min = -4096;
        let max = 4094;

        let Token::Identifier(text) = token else {
            return Err(InvalidToken(token.clone()));
        };

        let symbol_pc = match symbol_table.get(text) {
            Some(a) => *a,
            None => return Err(Translation(text.to_owned())),
        };

        let Ok(symbol_pc) = i128::try_from(symbol_pc) else {
            return Err(Internal);
        };

        let Ok(current_pc) = i128::try_from(current_pc) else {
            return Err(Internal);
        };

        let Some(offset) = symbol_pc.checked_sub(current_pc) else {
            return Err(Internal);
        };

        if offset % 2 != 0 {
            return Err(OddValue(offset));
        }

        if offset < min {
            return Err(SmallerValue(min, offset));
        }

        if offset > max {
            return Err(BiggerValue(max, offset));
        }

        let offset = i16::try_from(offset).map_err(|_| SyntaxError::Internal)?;

        Ok(Self(offset))
    }
    /// Encodes the ``BLabel`` value into a 12-bit signed integer, implicit 0 bit is truncated
    pub(crate) fn encode(&self) -> u32 {
        (u32::from(self.0.cast_unsigned() >> 1)) & 0b1111_1111_1111
    }
}
///20-bit signed PC-relative offset, multiple of 2 bytes. Used by ``JType``
#[derive(PartialEq, Eq, Debug)]
pub struct JLabel(i32);

impl JLabel {
    /// Creates a ``JLabel`` from an Identifier Token and a ``symbol_table``, is bound checked.
    pub(crate) fn new(
        token: &Token,
        symbol_table: &HashMap<String, usize>,
        current_pc: usize,
    ) -> Result<Self, SyntaxError> {
        let min = -1_048_576;
        let max = 1_048_574;

        let Token::Identifier(text) = token else {
            return Err(InvalidToken(token.clone()));
        };

        let symbol_pc = match symbol_table.get(text) {
            Some(a) => *a,
            None => return Err(Translation(text.to_owned())),
        };

        let Ok(symbol_pc) = i128::try_from(symbol_pc) else {
            return Err(Internal);
        };

        let Ok(current_pc) = i128::try_from(current_pc) else {
            return Err(Internal);
        };

        let Some(offset) = symbol_pc.checked_sub(current_pc) else {
            return Err(Internal);
        };

        if offset % 2 != 0 {
            return Err(OddValue(offset));
        }

        if offset < min {
            return Err(SmallerValue(min, offset));
        }

        if offset > max {
            return Err(BiggerValue(max, offset));
        }

        let offset = i32::try_from(offset).map_err(|_| SyntaxError::Internal)?;

        Ok(Self(offset))
    }
    /// Encodes the ``JLabel`` value into a 12-bit signed integer, implicit 0 bit is truncated
    pub(crate) const fn encode(&self) -> u32 {
        (self.0.cast_unsigned() >> 1) & 0b1111_1111_1111_1111_1111
    }
}
//--------------------------------------------------
#[derive(PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

impl Register {
    /// Creates a new integer enum from the given Indentifier Token, accepts aliases.
    pub(crate) fn new(token: &Token) -> Result<Self, SyntaxError> {
        let Token::Identifier(name) = token else {
            return Err(InvalidToken(token.clone()));
        };
        Ok(match name.as_str() {
            "x0" | "zero" => Self::X0,

            "x1" | "ra" => Self::X1,
            "x2" | "sp" => Self::X2,
            "x3" | "gp" => Self::X3,
            "x4" | "tp" => Self::X4,
            //---
            "x5" | "t0" => Self::X5,
            "x6" | "t1" => Self::X6,
            "x7" | "t2" => Self::X7,
            //---
            "x8" | "fp" | "s0" => Self::X8,
            "x9" | "s1" => Self::X9,
            "x10" | "a0" => Self::X10,
            //---
            "x11" | "a1" => Self::X11,
            "x12" | "a2" => Self::X12,
            "x13" | "a3" => Self::X13,
            "x14" | "a4" => Self::X14,
            "x15" | "a5" => Self::X15,
            "x16" | "a6" => Self::X16,
            "x17" | "a7" => Self::X17,
            //---
            "x18" | "s2" => Self::X18,
            "x19" | "s3" => Self::X19,
            "x20" | "s4" => Self::X20,
            "x21" | "s5" => Self::X21,
            "x22" | "s6" => Self::X22,
            "x23" | "s7" => Self::X23,
            "x24" | "s8" => Self::X24,
            "x25" | "s9" => Self::X25,
            "x26" | "s10" => Self::X26,
            "x27" | "s11" => Self::X27,
            //---
            "x28" | "t3" => Self::X28,
            "x29" | "t4" => Self::X29,
            "x30" | "t5" => Self::X30,
            "x31" | "t6" => Self::X31,
            _ => {
                return Err(NonExistentRegister(name.to_owned()));
            }
        })
    }
    /// Returns the u32 representation of the Register
    pub(crate) const fn encode(self) -> u32 {
        match self {
            Self::X0 => 0,
            Self::X1 => 1,
            Self::X2 => 2,
            Self::X3 => 3,
            Self::X4 => 4,
            Self::X5 => 5,
            Self::X6 => 6,
            Self::X7 => 7,
            Self::X8 => 8,
            Self::X9 => 9,
            Self::X10 => 10,
            Self::X11 => 11,
            Self::X12 => 12,
            Self::X13 => 13,
            Self::X14 => 14,
            Self::X15 => 15,
            Self::X16 => 16,
            Self::X17 => 17,
            Self::X18 => 18,
            Self::X19 => 19,
            Self::X20 => 20,
            Self::X21 => 21,
            Self::X22 => 22,
            Self::X23 => 23,
            Self::X24 => 24,
            Self::X25 => 25,
            Self::X26 => 26,
            Self::X27 => 27,
            Self::X28 => 28,
            Self::X29 => 29,
            Self::X30 => 30,
            Self::X31 => 31,
        }
    }
}
