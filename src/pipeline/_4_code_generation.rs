//! Fourth step of the pipeline, encoding the instruction types into their binary forms

// funct3
pub const ADDI_FUNCT3: u32 = 0b000;
pub const ADD_FUNCT3: u32 = 0b000;
pub const SUB_FUNCT3: u32 = 0b000;
pub const BEQ_FUNCT3: u32 = 0b000;
pub const BNE_FUNCT3: u32 = 0b001;
pub const BLT_FUNCT3: u32 = 0b100;
pub const BGE_FUNCT3: u32 = 0b101;
pub const JALR_FUNCT3: u32 = 0b000;
pub const XORI_FUNCT3: u32 = 0b100;
pub const XOR_FUNCT3: u32 = 0b100;
pub const ORI_FUNCT3: u32 = 0b110;
pub const OR_FUNCT3: u32 = 0b110;
pub const ANDI_FUNCT3: u32 = 0b111;
pub const AND_FUNCT3: u32 = 0b111;
pub const LB_FUNCT3: u32 = 0b000;
pub const SB_FUNCT3: u32 = 0b000;
pub const SW_FUNCT3: u32 = 0b010;
pub const LW_FUNCT3: u32 = 0b010;
pub const SLLI_FUNCT3: u32 = 0b001;
pub const SRLI_FUNCT3: u32 = 0b101;

// funct7
pub const ADDI_FUNCT7: u32 = 0b000_0000;
pub const ADD_FUNCT7: u32 = 0b000_0000;
pub const SUB_FUNCT7: u32 = 0b100_000;
pub const OR_FUNCT7: u32 = 0b000_000;
pub const XOR_FUNCT7: u32 = 0b000_000;
pub const AND_FUNCT7: u32 = 0b000_000;
pub const SLLI_FUNCT7: u32 = 0b000_000;
pub const SRLI_FUNCT7: u32 = 0b000_000;

// opcode
pub const RTYPE_OPCODE: u32 = 51;
pub const BTYPE_OPCODE: u32 = 99;
pub const JTYPE_OPCODE: u32 = 111;
pub const ITYPE_OPCODE: u32 = 19;
pub const ITYPE_SHIFTS_OPCODE: u32 = 19;
pub const ITYPE_MEMORY_OPCODE: u32 = 3;
pub const ITYPE_JUMP_OPCODE: u32 = 103;
pub const STYPE_MEMORY_OPCODE: u32 = 35;
pub const LUI_OPCODE: u32 = 0b011_0111;

use crate::utils::instruction::Instruction;

/// Encodes the provided ``instructions`` vector into their binary encoded form according to RV32I
pub(super) fn assemble(instructions: Vec<Instruction>) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();

    for i in instructions {
        buffer.push(encode_instruction(i));
    }

    buffer
}

/// Encode individual ``instruction`` into its 4 byte form
fn encode_instruction(instruction: Instruction) -> u32 {
    match instruction {
        Instruction::Add(rtype) => encode_rtype(
            RTYPE_OPCODE,
            rtype.rd.encode(),
            ADD_FUNCT3,
            rtype.rs1.encode(),
            rtype.rs2.encode(),
            ADD_FUNCT7,
        ),

        Instruction::Sub(rtype) => encode_rtype(
            RTYPE_OPCODE,
            rtype.rd.encode(),
            SUB_FUNCT3,
            rtype.rs1.encode(),
            rtype.rs2.encode(),
            SUB_FUNCT7,
        ),

        Instruction::And(rtype) => encode_rtype(
            RTYPE_OPCODE,
            rtype.rd.encode(),
            AND_FUNCT3,
            rtype.rs1.encode(),
            rtype.rs2.encode(),
            AND_FUNCT7,
        ),

        Instruction::Or(rtype) => encode_rtype(
            RTYPE_OPCODE,
            rtype.rd.encode(),
            OR_FUNCT3,
            rtype.rs1.encode(),
            rtype.rs2.encode(),
            OR_FUNCT7,
        ),

        Instruction::Xor(rtype) => encode_rtype(
            RTYPE_OPCODE,
            rtype.rd.encode(),
            XOR_FUNCT3,
            rtype.rs1.encode(),
            rtype.rs2.encode(),
            XOR_FUNCT7,
        ),

        // ---
        Instruction::Bne(btype) => encode_btype(
            BTYPE_OPCODE,
            btype.blabel.encode(),
            BNE_FUNCT3,
            btype.rs1.encode(),
            btype.rs2.encode(),
        ),

        Instruction::Beq(btype) => encode_btype(
            BTYPE_OPCODE,
            btype.blabel.encode(),
            BEQ_FUNCT3,
            btype.rs1.encode(),
            btype.rs2.encode(),
        ),

        Instruction::Blt(btype) => encode_btype(
            BTYPE_OPCODE,
            btype.blabel.encode(),
            BLT_FUNCT3,
            btype.rs1.encode(),
            btype.rs2.encode(),
        ),

        Instruction::Bge(btype) => encode_btype(
            BTYPE_OPCODE,
            btype.blabel.encode(),
            BGE_FUNCT3,
            btype.rs1.encode(),
            btype.rs2.encode(),
        ),

        // ---
        Instruction::Jal(jtype) => {
            encode_jtype(JTYPE_OPCODE, jtype.rd.encode(), jtype.jlabel.encode())
        }

        // ---
        Instruction::Jalr(itype_jump) => encode_itype(
            ITYPE_JUMP_OPCODE,
            itype_jump.rd.encode(),
            JALR_FUNCT3,
            itype_jump.rs1.encode(),
            itype_jump.offset.encode(),
        ),

        // ---
        Instruction::Lw(itype_memory) => encode_itype(
            ITYPE_MEMORY_OPCODE,
            itype_memory.rd.encode(),
            LW_FUNCT3,
            itype_memory.rs1.encode(),
            itype_memory.offset.encode(),
        ),

        Instruction::Lb(itype_memory) => encode_itype(
            ITYPE_MEMORY_OPCODE,
            itype_memory.rd.encode(),
            LB_FUNCT3,
            itype_memory.rs1.encode(),
            itype_memory.offset.encode(),
        ),

        // ---
        Instruction::Sw(stype_memory) => encode_stype(
            STYPE_MEMORY_OPCODE,
            stype_memory.offset.encode(),
            SW_FUNCT3,
            stype_memory.rbase.encode(),
            stype_memory.rs.encode(),
        ),

        Instruction::Sb(stype_memory) => encode_stype(
            STYPE_MEMORY_OPCODE,
            stype_memory.offset.encode(),
            SB_FUNCT3,
            stype_memory.rbase.encode(),
            stype_memory.rs.encode(),
        ),

        // ---
        Instruction::Slli(itype_shifts) => encode_itype(
            ITYPE_SHIFTS_OPCODE,
            itype_shifts.rd.encode(),
            SLLI_FUNCT3,
            itype_shifts.rs1.encode(),
            itype_shifts.shamt.encode() | SLLI_FUNCT7 << 5,
        ),

        Instruction::Srli(itype_shifts) => encode_itype(
            ITYPE_SHIFTS_OPCODE,
            itype_shifts.rd.encode(),
            SRLI_FUNCT3,
            itype_shifts.rs1.encode(),
            itype_shifts.shamt.encode() | SRLI_FUNCT7 << 5,
        ),

        // ---
        Instruction::Addi(itype) => encode_itype(
            ITYPE_OPCODE,
            itype.rd.encode(),
            ADDI_FUNCT3,
            itype.rs1.encode(),
            itype.imm.encode(),
        ),

        Instruction::Andi(itype) => encode_itype(
            ITYPE_OPCODE,
            itype.rd.encode(),
            ANDI_FUNCT3,
            itype.rs1.encode(),
            itype.imm.encode(),
        ),

        Instruction::Ori(itype) => encode_itype(
            ITYPE_OPCODE,
            itype.rd.encode(),
            ORI_FUNCT3,
            itype.rs1.encode(),
            itype.imm.encode(),
        ),

        Instruction::Xori(itype) => encode_itype(
            ITYPE_OPCODE,
            itype.rd.encode(),
            XORI_FUNCT3,
            itype.rs1.encode(),
            itype.imm.encode(),
        ),

        // ---
        Instruction::Lui(utype) => {
            encode_utype(LUI_OPCODE, utype.rd.encode(), utype.constant.encode())
        }
    }
}
pub const fn encode_rtype(
    opcode: u32,
    rd: u32,
    funct3: u32,
    rs1: u32,
    rs2: u32,
    funct7: u32,
) -> u32 {
    let rd = rd << 7;
    let funct3 = funct3 << 12;
    let rs1 = rs1 << 15;
    let rs2 = rs2 << 20;
    let funct7 = funct7 << 25;

    funct7 | rs2 | rs1 | funct3 | rd | opcode
}

pub const fn encode_itype(opcode: u32, rd: u32, funct3: u32, rs1: u32, imm: u32) -> u32 {
    let rd = rd << 7;
    let funct3 = funct3 << 12;
    let rs1 = rs1 << 15;
    let imm = imm << 20;

    imm | rs1 | funct3 | rd | opcode
}

pub const fn encode_stype(opcode: u32, imm: u32, funct3: u32, rs1: u32, rs2: u32) -> u32 {
    let imm_1 = (imm & 0b11111) << 7;
    let funct3 = funct3 << 12;
    let rs1 = rs1 << 15;
    let rs2 = rs2 << 20;
    let imm_2 = ((imm >> 5) & 0b_1111111) << 25;

    imm_2 | rs2 | rs1 | funct3 | imm_1 | opcode
}

pub const fn encode_btype(opcode: u32, imm: u32, funct3: u32, rs1: u32, rs2: u32) -> u32 {
    let imm_1 = ((imm >> 10) & 0b1) << 7;
    let imm_2 = (imm & 0b1111) << 8;
    let funct3 = funct3 << 12;
    let rs1 = rs1 << 15;
    let rs2 = rs2 << 20;
    let imm_3 = ((imm >> 4) & 0b11_1111) << 25;
    let imm_4 = ((imm >> 11) & 0b1) << 31;

    imm_4 | imm_3 | rs2 | rs1 | funct3 | imm_2 | imm_1 | opcode
}

pub const fn encode_jtype(opcode: u32, rd: u32, imm: u32) -> u32 {
    let rd = rd << 7;
    let label_1 = ((imm >> 11) & 0b1111_1111) << 12;
    let label_2 = ((imm >> 10) & 0b1) << 20;
    let label_3 = ((imm) & 0b11_1111_1111) << 21;
    let label_4 = ((imm >> 19) & 0b1) << 31;

    label_4 | label_3 | label_2 | label_1 | rd | opcode
}

pub const fn encode_utype(opcode: u32, rd: u32, imm: u32) -> u32 {
    let rd = rd << 7;
    let imm = imm << 12;

    imm | rd | opcode
}
