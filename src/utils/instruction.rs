//! Module for every instruction type and its operands
use crate::utils::operands::{
    BLabel, CharFlag, Constant, Immediate, JLabel, Offset, Register, Shamt,
};

#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    Add(RType),
    Sub(RType),
    Xor(RType),
    Or(RType),
    And(RType),

    // ---
    Sll(RType),
    Srl(RType),
    Sra(RType),
    Slt(RType),
    Sltu(RType),

    //---
    Addi(IType),
    Xori(IType),
    Ori(IType),
    Andi(IType),
    Slti(IType),
    Sltiu(IType),

    // ---
    Slli(ITypeShifts),
    Srli(ITypeShifts),
    Srai(ITypeShifts),

    // ---
    Jalr(ITypeJump),

    // ---
    Lw(ITypeMemory),
    Lh(ITypeMemory),
    Lb(ITypeMemory),
    Lhu(ITypeMemory),
    Lbu(ITypeMemory),

    // ---
    Sb(STypeMemory),
    Sh(STypeMemory),
    Sw(STypeMemory),

    // ---
    Beq(BType),
    Bne(BType),
    Blt(BType),
    Bge(BType),
    Bltu(BType),
    Bgeu(BType),

    // ---
    Lui(UType),
    Auipc(UType),

    // ---
    Jal(JType),

    //---
    Fence(FType),

    //---
    Ecall,
    Ebreak,
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
    pub rs: Register,
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
#[derive(PartialEq, Eq, Debug)]
pub struct UType {
    pub rd: Register,
    pub constant: Constant,
}
#[derive(PartialEq, Eq, Debug)]
pub struct FType {
    pub pred: CharFlag,
    pub succ: CharFlag,
}
