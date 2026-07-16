use std::collections::HashMap;

use crate::auxiliar::{
    convertion::{i128_to_i16, i128_to_i32, i128_to_u8, interpret_literal},
    error::SyntaxError::{
        self, BiggerValue, Impossible, InvalidWord, NonExistentRegister, OddValue, SmallerValue,
        TexttoNumeric, Translation,
    },
    token::Token,
};

//--------------------------------------------------
#[derive(PartialEq, Eq, Debug)]
pub struct Immediate(i16); // 12-bit signed integer (range: -2048 to 2047). Limit artificially

impl Immediate {
    pub fn new(token: &Token) -> Result<Self, SyntaxError> {
        let max = 2047;
        let min = -2048;

        let Token::Literal(text) = token else {
            return Err(InvalidWord(token.clone()));
        };

        let Ok(value) = interpret_literal(text) else {
            return Err(TexttoNumeric(token.clone()));
        };

        if value > max {
            return Err(BiggerValue(max, value));
        }

        if value < min {
            return Err(SmallerValue(min, value));
        }

        let Ok(value) = i128_to_i16(value) else {
            return Err(SyntaxError::Impossible);
        };

        Ok(Self(value))
    }

    pub const fn encode(&self) -> u32 {
        (self.0.cast_unsigned() as u32) & 0b1111_1111_1111
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Shamt(u8); //5-bit unsigned integer (range: 0 to 31 for 32-bit registers). Limit artificially

impl Shamt {
    pub fn new(token: &Token) -> Result<Self, SyntaxError> {
        let max = 31;
        let min = 0;

        let Token::Literal(text) = token else {
            return Err(InvalidWord(token.clone()));
        };

        let Ok(value) = interpret_literal(text) else {
            return Err(TexttoNumeric(token.clone()));
        };

        if value > max {
            return Err(BiggerValue(max, value));
        }

        if value < min {
            return Err(SmallerValue(min, value));
        }

        let Ok(value) = i128_to_u8(value) else {
            return Err(SyntaxError::Impossible);
        };

        Ok(Self(value))
    }
    pub fn encode(&self) -> u32 {
        u32::from(self.0)
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Offset(i16); //12-bit signed immediate offset (range: -2048 to 2047 bytes). Limit artificially

impl Offset {
    pub fn new(token: &Token) -> Result<Self, SyntaxError> {
        let max = 2047;
        let min = -2048;

        let Token::Literal(text) = token else {
            return Err(InvalidWord(token.clone()));
        };

        let Ok(value) = interpret_literal(text) else {
            return Err(TexttoNumeric(token.clone()));
        };

        if value > max {
            return Err(BiggerValue(max, value));
        }

        if value < min {
            return Err(SmallerValue(min, value));
        }

        let Ok(value) = i128_to_i16(value) else {
            return Err(SyntaxError::Impossible);
        };

        Ok(Self(value))
    }
    pub const fn encode(&self) -> u32 {
        (self.0.cast_unsigned() as u32) & 0b1111_1111_1111
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Label(i16); //12-bit signed PC-relative offset. limit artificially. multiple of 2 bytes

impl Label {
    pub fn new(
        token: &Token,
        symbol_table: &HashMap<String, usize>,
        current_pc: usize,
    ) -> Result<Self, SyntaxError> {
        let min = -4096;
        let max = 4094;

        let Token::Identifier(text) = token else {
            return Err(InvalidWord(token.clone()));
        };

        let symbol_pc = match symbol_table.get(text) {
            Some(a) => *a,
            None => return Err(Translation(token.clone())),
        };

        let Ok(symbol_pc) = i128::try_from(symbol_pc) else {
            return Err(Impossible);
        };

        let Ok(current_pc) = i128::try_from(current_pc) else {
            return Err(Impossible);
        };
        let offset = symbol_pc - current_pc;

        if offset % 2 != 0 {
            return Err(OddValue(token.clone()));
        }

        if offset < min {
            return Err(SmallerValue(min, offset));
        }

        if offset > max {
            return Err(BiggerValue(max, offset));
        }

        let Ok(offset) = i128_to_i16(offset) else {
            return Err(SyntaxError::Impossible);
        };

        Ok(Self(offset))
    }
    pub const fn encode(&self) -> u32 {
        (self.0.cast_unsigned() as u32) & 0b1111_1111_1111
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct BigLabel(i32); //20-bit signed PC-relative offset. Limit artificially. multiple of 2 bytes

impl BigLabel {
    pub fn new(
        token: &Token,
        symbol_table: &HashMap<String, usize>,
        current_pc: usize,
    ) -> Result<Self, SyntaxError> {
        let min = -1_048_576;
        let max = 1_048_574;

        let Token::Identifier(text) = token else {
            return Err(InvalidWord(token.clone()));
        };

        let symbol_pc = match symbol_table.get(text) {
            Some(a) => *a,
            None => return Err(Translation(token.clone())),
        };

        let Ok(symbol_pc) = i128::try_from(symbol_pc) else {
            return Err(Impossible);
        };

        let Ok(current_pc) = i128::try_from(current_pc) else {
            return Err(Impossible);
        };

        let offset = symbol_pc - current_pc;

        if offset % 2 != 0 {
            return Err(OddValue(token.clone()));
        }

        if offset < min {
            return Err(SmallerValue(min, offset));
        }

        if offset > max {
            return Err(BiggerValue(max, offset));
        }

        let Ok(offset) = i128_to_i32(offset) else {
            return Err(SyntaxError::Impossible);
        };
        Ok(Self(offset))
    }

    pub const fn encode(&self) -> u32 {
        (self.0 as u16 as u32) & 0b1111_1111_1111_1111_1111
    }
}
//--------------------------------------------------
#[derive(PartialEq, Eq, Debug)]
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
    pub fn new(token: &Token) -> Result<Self, SyntaxError> {
        let Token::Identifier(name) = token else {
            return Err(InvalidWord(token.clone()));
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
                return Err(NonExistentRegister(token.clone()));
            }
        })
    }
}
