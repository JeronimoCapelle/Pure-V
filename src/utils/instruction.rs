//! Module for every instruction type and its operands
use crate::utils::operands::{BLabel, Immediate, JLabel, Offset, Register, Shamt};

#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    Addi(IType),
    Add(RType),
    Sub(RType),
    Bne(BType),
    Beq(BType),
    Blt(BType),
    Bge(BType),
    Jal(JType),
    Jalr(ITypeJump),
    Lw(ITypeMemory),
    Sw(STypeMemory),
    Lb(ITypeMemory),
    Sb(STypeMemory),
    Slli(ITypeShifts),
    Srli(ITypeShifts),
    And(RType),
    Or(RType),
    Xor(RType),
    Andi(IType),
    Ori(IType),
    Xori(IType),
}

//--------------------------------------------------
#[derive(PartialEq, Eq, Debug)]
pub struct RType {
    pub rd: Register,
    pub rs1: Register,
    pub rs2: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub struct IType {
    pub rd: Register,
    pub rs1: Register,
    pub imm: Immediate,
}
#[derive(PartialEq, Eq, Debug)]
pub struct ITypeShifts {
    pub rd: Register,
    pub rs1: Register,
    pub shamt: Shamt,
}
#[derive(PartialEq, Eq, Debug)]
pub struct ITypeMemory {
    pub rd: Register,
    pub offset: Offset,
    pub rs1: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub struct STypeMemory {
    pub rs1: Register,
    pub offset: Offset,
    pub rbase: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub struct BType {
    pub rs1: Register,
    pub rs2: Register,
    pub blabel: BLabel,
}
#[derive(PartialEq, Eq, Debug)]
pub struct JType {
    pub rd: Register,
    pub jlabel: JLabel,
}
#[derive(PartialEq, Eq, Debug)]
pub struct ITypeJump {
    pub rd: Register,
    pub offset: Offset,
    pub rs1: Register,
}
