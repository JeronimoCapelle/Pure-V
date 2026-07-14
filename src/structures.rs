use std::collections::HashMap;

use crate::structures::ParsingError::{
    BiggerValue, LabelTranslation, NonExistentRegister, NonIdentifier, NonLiteral, OddValue,
    SmallerValue, TexttoNumeric,
};

//--------------------------------

#[derive(Debug)]
pub struct TrackedError {
    pub kind: ParsingError,
    pub line: u32,
    pub file: &'static str, // Tracks the exact file (e.g., "src/structures.rs")
}

#[derive(Debug)]
pub enum ParsingError {
    BiggerValue,
    SmallerValue,
    OddValue,
    TexttoNumeric,
    NonExistentRegister,
    NonExistentMnemonic,
    WrongArgument,
    Tokenizer,
    NonLiteral,
    NonIdentifier,
    LabelTranslation,
    Empty,
}

//-----------------------
#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    ADDI(IType),
    ADD(RType),
    SUB(RType),
    BNE(BType),
    BEQ(BType),
    BLT(BType),
    BGE(BType),
    JAL(JType),
    JALR(ITypeJump),
    LW(ITypeMemory),
    SW(STypeMemory),
    LB(ITypeMemory),
    SB(STypeMemory),
    SLLI(ITypeShifts),
    SRLI(ITypeShifts),
    AND(RType),
    OR(RType),
    XOR(RType),
    ANDI(IType),
    ORI(IType),
    XORI(IType),
}

//--------------------------------------------------
#[derive(PartialEq, Eq, Debug)]
pub struct RType {
    pub destination: Register,
    pub first_source: Register,
    pub second_source: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub struct IType {
    pub destination: Register,
    pub source: Register,
    pub immediate: Immediate,
}
#[derive(PartialEq, Eq, Debug)]
pub struct ITypeShifts {
    pub destination: Register,
    pub source: Register,
    pub shamt: Shamt,
}
#[derive(PartialEq, Eq, Debug)]
pub struct ITypeMemory {
    pub destination: Register,
    pub offset: Offset,
    pub base_address: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub struct STypeMemory {
    pub source: Register,
    pub offset: Offset,
    pub base_address: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub struct BType {
    pub first_source: Register,
    pub second_source: Register,
    pub label: Label,
}
#[derive(PartialEq, Eq, Debug)]
pub struct JType {
    pub destination: Register,
    pub big_label: BigLabel,
}
#[derive(PartialEq, Eq, Debug)]
pub struct ITypeJump {
    pub destination: Register,
    pub offset: Offset,
    pub target_address: Register,
}

//--------------------------------------------------
#[derive(PartialEq, Eq, Debug)]
pub struct Immediate(i16); // 12-bit signed integer (range: -2048 to 2047). Limit artificially

impl Immediate {
    pub fn new(token: &Token) -> Result<Immediate, TrackedError> {
        let value = match token {
            Token::Literal(a) => a,
            _ => {
                return Err(TrackedError {
                    kind: NonLiteral,
                    line: line!(),
                    file: file!(),
                });
            }
        };
        let numeric: i16;

        if value.starts_with("0b") {
            numeric = match i16::from_str_radix(value.strip_prefix("0b").unwrap().trim(), 2) {
                Ok(a) => a,
                Err(_) => {
                    return Err(TrackedError {
                        kind: TexttoNumeric,
                        line: line!(),
                        file: file!(),
                    });
                }
            }
        } else if value.starts_with("0x") {
            numeric = match i16::from_str_radix(value.strip_prefix("0x").unwrap().trim(), 16) {
                Ok(a) => a,
                Err(_) => {
                    return Err(TrackedError {
                        kind: TexttoNumeric,
                        line: line!(),
                        file: file!(),
                    });
                }
            }
        } else {
            numeric = match value.parse() {
                Ok(a) => a,
                Err(_) => {
                    return Err(TrackedError {
                        kind: TexttoNumeric,
                        line: line!(),
                        file: file!(),
                    });
                }
            }
        }

        if numeric < -2048 {
            return Err(TrackedError {
                kind: SmallerValue,
                line: line!(),
                file: file!(),
            });
        } else if numeric > 2047 {
            return Err(TrackedError {
                kind: BiggerValue,
                line: line!(),
                file: file!(),
            });
        }
        Ok(Immediate(numeric))
    }
    pub fn encode(&self) -> u32 {
        if self.0 >= 0 {
            self.0 as u32
        } else {
            ((self.0 + 2048) as u32) | 2048
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Shamt(u8); //5-bit unsigned integer (range: 0 to 31 for 32-bit registers). Limit artificially

impl Shamt {
    pub fn new(token: &Token) -> Result<Shamt, TrackedError> {
        let value = match token {
            Token::Literal(a) => a,
            _ => {
                return Err(TrackedError {
                    kind: NonLiteral,
                    line: line!(),
                    file: file!(),
                });
            }
        };

        let numeric: u8;

        if value.starts_with("0b") {
            numeric = match u8::from_str_radix(value.strip_prefix("0b").unwrap().trim(), 2) {
                Ok(a) => a,
                Err(_) => {
                    return Err(TrackedError {
                        kind: TexttoNumeric,
                        line: line!(),
                        file: file!(),
                    });
                }
            }
        } else if value.starts_with("0x") {
            numeric = match u8::from_str_radix(value.strip_prefix("0x").unwrap().trim(), 16) {
                Ok(a) => a,
                Err(_) => {
                    return Err(TrackedError {
                        kind: TexttoNumeric,
                        line: line!(),
                        file: file!(),
                    });
                }
            }
        } else {
            numeric = match value.parse() {
                Ok(a) => a,
                Err(_) => {
                    return Err(TrackedError {
                        kind: TexttoNumeric,
                        line: line!(),
                        file: file!(),
                    });
                }
            }
        }

        if numeric > 31 {
            return Err(TrackedError {
                kind: BiggerValue,
                line: line!(),
                file: file!(),
            });
        }
        Ok(Shamt(numeric))
    }
    pub fn encode(&self) -> u32 {
        self.0 as u32
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Offset(i16); //12-bit signed immediate offset (range: -2048 to 2047 bytes). Limit artificially

impl Offset {
    pub fn new(token: &Token) -> Result<Offset, TrackedError> {
        let value = match token {
            Token::Literal(a) => a,
            _ => {
                return Err(TrackedError {
                    kind: NonLiteral,
                    line: line!(),
                    file: file!(),
                });
            }
        };

        let numeric: i16;

        if value.starts_with("0b") {
            numeric = match i16::from_str_radix(value.strip_prefix("0b").unwrap().trim(), 2) {
                Ok(a) => a,
                Err(_) => {
                    return Err(TrackedError {
                        kind: TexttoNumeric,
                        line: line!(),
                        file: file!(),
                    });
                }
            }
        } else if value.starts_with("0x") {
            numeric = match i16::from_str_radix(value.strip_prefix("0x").unwrap().trim(), 16) {
                Ok(a) => a,
                Err(_) => {
                    return Err(TrackedError {
                        kind: TexttoNumeric,
                        line: line!(),
                        file: file!(),
                    });
                }
            }
        } else {
            numeric = match value.parse() {
                Ok(a) => a,
                Err(_) => {
                    return Err(TrackedError {
                        kind: TexttoNumeric,
                        line: line!(),
                        file: file!(),
                    });
                }
            }
        }

        if numeric < -2048 {
            return Err(TrackedError {
                kind: SmallerValue,
                line: line!(),
                file: file!(),
            });
        }

        if numeric > 2047 {
            return Err(TrackedError {
                kind: BiggerValue,
                line: line!(),
                file: file!(),
            });
        }
        Ok(Offset(numeric))
    }
    pub fn encode(&self) -> u32 {
        if self.0 >= 0 {
            self.0 as u32
        } else {
            ((self.0 + 2048) as u32) | 2048
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Label(i16); //12-bit signed PC-relative offset. limit artificially. multiple of 2 bytes

impl Label {
    pub fn new(
        token: &Token,
        symbol_table: &HashMap<String, usize>,
        current_pc: usize,
    ) -> Result<Label, TrackedError> {
        let value = match token {
            Token::Identifier(a) => a,
            _ => {
                return Err(TrackedError {
                    kind: NonIdentifier,
                    line: line!(),
                    file: file!(),
                });
            }
        };
        if !symbol_table.contains_key(value) {
            return Err(TrackedError {
                kind: LabelTranslation,
                line: line!(),
                file: file!(),
            });
        }
        let offset: i128 = i128::try_from(*symbol_table.get(value).unwrap()).unwrap()
            - i128::try_from(current_pc).unwrap();
        if offset % 2 != 0 {
            return Err(TrackedError {
                kind: OddValue,
                line: line!(),
                file: file!(),
            });
        }
        if offset < -4096 {
            return Err(TrackedError {
                kind: SmallerValue,
                line: line!(),
                file: file!(),
            });
        }

        if offset > 4094 {
            return Err(TrackedError {
                kind: BiggerValue,
                line: line!(),
                file: file!(),
            });
        }

        Ok(Label(offset.try_into().unwrap()))
    }
    pub fn encode(&self) -> u32 {
        if self.0 >= 0 {
            self.0 as u32
        } else {
            ((self.0 + 4096) as u32) | 4096
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct BigLabel(i32); //20-bit signed PC-relative offset. Limit artificially. multiple of 2 bytes

impl BigLabel {
    pub fn new(
        token: &Token,
        symbol_table: &HashMap<String, usize>,
        current_pc: usize,
    ) -> Result<BigLabel, TrackedError> {
        let value = match token {
            Token::Identifier(a) => a,
            _ => {
                return Err(TrackedError {
                    kind: NonIdentifier,
                    line: line!(),
                    file: file!(),
                });
            }
        };
        if !symbol_table.contains_key(value) {
            return Err(TrackedError {
                kind: LabelTranslation,
                line: line!(),
                file: file!(),
            });
        }
        let offset: i128 = i128::try_from(*symbol_table.get(value).unwrap()).unwrap()
            - i128::try_from(current_pc).unwrap();
        if offset % 2 != 0 {
            return Err(TrackedError {
                kind: OddValue,
                line: line!(),
                file: file!(),
            });
        }
        if offset < -1_048_576 {
            return Err(TrackedError {
                kind: SmallerValue,
                line: line!(),
                file: file!(),
            });
        }

        if offset > 1_048_574 {
            return Err(TrackedError {
                kind: BiggerValue,
                line: line!(),
                file: file!(),
            });
        }

        Ok(BigLabel(offset.try_into().unwrap()))
    }
    pub fn encode(&self) -> u32 {
        if self.0 >= 0 {
            self.0 as u32
        } else {
            ((self.0 + 1_048_576) as u32) | 1_048_576
        }
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
    pub fn new(token: &Token) -> Result<Register, TrackedError> {
        let name = match token {
            Token::Identifier(a) => a,
            _ => {
                return Err(TrackedError {
                    kind: NonIdentifier,
                    line: line!(),
                    file: file!(),
                });
            }
        };

        Ok(match name.as_str() {
            "x0" => Register::X0,
            "zero" => Register::X0,

            "x1" => Register::X1,
            "ra" => Register::X1,

            "x2" => Register::X2,
            "sp" => Register::X2,

            "x3" => Register::X3,
            "gp" => Register::X3,

            "x4" => Register::X4,
            "tp" => Register::X4,

            //---
            "x5" => Register::X5,
            "t0" => Register::X5,

            "x6" => Register::X6,
            "t1" => Register::X6,

            "x7" => Register::X7,
            "t2" => Register::X7,

            //---
            "x8" => Register::X8,
            "fp" => Register::X8,
            "s0" => Register::X8,

            "x9" => Register::X9,
            "s1" => Register::X9,

            "x10" => Register::X10,
            "a0" => Register::X10,

            //---
            "x11" => Register::X11,
            "a1" => Register::X11,

            "x12" => Register::X12,
            "a2" => Register::X12,

            "x13" => Register::X13,
            "a3" => Register::X13,

            "x14" => Register::X14,
            "a4" => Register::X14,

            "x15" => Register::X15,
            "a5" => Register::X15,

            "x16" => Register::X16,
            "a6" => Register::X16,

            "x17" => Register::X17,
            "a7" => Register::X17,

            //---
            "x18" => Register::X18,
            "s2" => Register::X18,

            "x19" => Register::X19,
            "s3" => Register::X19,

            "x20" => Register::X20,
            "s4" => Register::X20,

            "x21" => Register::X21,
            "s5" => Register::X21,

            "x22" => Register::X22,
            "s6" => Register::X22,

            "x23" => Register::X23,
            "s7" => Register::X23,

            "x24" => Register::X24,
            "s8" => Register::X24,

            "x25" => Register::X25,
            "s9" => Register::X25,

            "x26" => Register::X26,
            "s10" => Register::X26,

            "x27" => Register::X27,
            "s11" => Register::X27,

            //---
            "x28" => Register::X28,
            "t3" => Register::X28,

            "x29" => Register::X29,
            "t4" => Register::X29,

            "x30" => Register::X30,
            "t5" => Register::X30,

            "x31" => Register::X31,
            "t6" => Register::X31,

            _ => {
                return Err(TrackedError {
                    kind: NonExistentRegister,
                    line: line!(),
                    file: file!(),
                });
            }
        })
    }
}

//--------------------------------------------------

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Literal(String),
    Identifier(String),

    Comma,
    Colon,
    OpeningParenthesis,
    ClosingParenthesis,
    NewLine,
}
