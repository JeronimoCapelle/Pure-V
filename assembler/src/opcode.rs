use crate::{
    opcode::ParsingError::{
        BiggerValueError, NonExistentRegisterError, SmallerValueError, SymbolError,
    },
    tokens::Token,
};

#[derive(Debug)]
pub enum ParsingError {
    BiggerValueError,
    SmallerValueError,
    TexttoNumericError,
    NonExistentRegisterError,
    WrongArgumentCountError,
    NonExistentOpcodeError,
    TokenizerError,
    SymbolError,
}

pub enum Mnemonic {
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

pub struct RType {
    pub destination: Register,
    pub first_source: Register,
    pub second_source: Register,
}

pub struct IType {
    pub destination: Register,
    pub source: Register,
    pub immediate: Immediate,
}

pub struct ITypeShifts {
    pub destination: Register,
    pub source: Register,
    pub shamt: Shamt,
}

pub struct ITypeMemory {
    pub destination: Register,
    pub offset: Offset,
    pub base_address: Register,
}

pub struct STypeMemory {
    pub source: Register,
    pub offset: Offset,
    pub base_address: Register,
}

pub struct BType {
    pub first_source: Register,
    pub second_source: Register,
    pub label: Label,
}

pub struct JType {
    pub destination: Register,
    pub big_label: BigLabel,
}

pub struct ITypeJump {
    pub destination: Register,
    pub offset: Offset,
    pub target_address: Register,
}

//--------------------------------------------------

pub struct Immediate(i16); // 12-bit signed integer (range: -2048 to 2047). Limit artificially

impl Immediate {
    pub fn new(value: &str) -> Result<Immediate, ParsingError> {
        let value = value.trim();
        let numeric: i16;

        if value.starts_with("0b") {
            numeric = i16::from_str_radix(value.strip_prefix("0b").unwrap().trim(), 8).unwrap();
        } else if value.starts_with("0x") {
            numeric = i16::from_str_radix(value.strip_prefix("0x").unwrap().trim(), 16).unwrap();
        } else {
            numeric = value.parse().unwrap();
        }

        if numeric < -2048 {
            return Err(SmallerValueError);
        } else if numeric > 2047 {
            return Err(BiggerValueError);
        }
        Ok(Immediate(numeric))
    }
}

pub struct Shamt(u8); //5-bit unsigned integer (range: 0 to 31 for 32-bit registers). Limit artificially

impl Shamt {
    pub fn new(value: u8) -> Result<Shamt, ParsingError> {
        if value > 31 {
            return Err(BiggerValueError);
        }
        Ok(Shamt(value))
    }
}

pub struct Offset(i16); //12-bit signed immediate offset (range: -2048 to 2047 bytes). Limit artificially

impl Offset {
    pub fn new(value: i16) -> Result<Offset, ParsingError> {
        if value < -2048 {
            return Err(SmallerValueError);
        }

        if value > 2047 {
            return Err(BiggerValueError);
        }
        Ok(Offset(value))
    }
}
pub struct Label(i16); //12-bit signed PC-relative offset. limit artificially.

impl Label {
    pub fn new() {}
}
pub struct BigLabel(i32); //20-bit signed PC-relative offset. Limit artificially

impl BigLabel {
    pub fn new() {}
}
//--------------------------------------------------

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
    pub fn new(token: &Token) -> Result<Register, ParsingError> {
        let name = match token {
            Token::Identifier(a) => a,
            _ => {
                return Err(SymbolError);
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

            _ => return Err(NonExistentRegisterError),
        })
    }
}
