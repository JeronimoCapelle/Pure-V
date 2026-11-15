//! Machine-code encoding stage (`&[Instruction] -> Vec<u32>`).

use alloc::vec::Vec;

use purev_isa::{Instruction, Register};

use crate::error::EncodeError;

/// Encodes a slice of [`Instruction`]s into 32-bit RISC-V instruction words.
pub fn encode(instructions: &[Instruction]) -> Result<Vec<u32>, EncodeError> {
    let mut words = Vec::with_capacity(instructions.len());
    for &inst in instructions {
        words.push(encode_instruction(inst)?);
    }
    Ok(words)
}

/// Encodes a slice of [`Instruction`]s into a little-endian byte buffer.
pub fn encode_bytes(instructions: &[Instruction]) -> Result<Vec<u8>, EncodeError> {
    let words = encode(instructions)?;
    let mut bytes = Vec::with_capacity(words.len() * 4);
    for word in words {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    Ok(bytes)
}

/// Encodes a single [`Instruction`] into its 32-bit little-endian machine word.
pub fn encode_instruction(inst: Instruction) -> Result<u32, EncodeError> {
    let word = match inst {
        Instruction::Lui { rd, imm } => encode_utype(0x37, reg(rd), upper_imm20(imm)),
        Instruction::Auipc { rd, imm } => encode_utype(0x17, reg(rd), upper_imm20(imm)),
        Instruction::Jal { rd, imm } => {
            let imm21 = check_j_imm(imm)?;
            encode_jtype(0x6F, reg(rd), imm21)
        },
        Instruction::Jalr { rd, rs1, imm } => {
            encode_itype(0x67, reg(rd), 0x0, reg(rs1), check_i12(imm)?)
        },
        Instruction::Beq { rs1, rs2, imm } => {
            encode_btype(0x63, check_b_imm(imm)?, 0x0, reg(rs1), reg(rs2))
        },
        Instruction::Bne { rs1, rs2, imm } => {
            encode_btype(0x63, check_b_imm(imm)?, 0x1, reg(rs1), reg(rs2))
        },
        Instruction::Blt { rs1, rs2, imm } => {
            encode_btype(0x63, check_b_imm(imm)?, 0x4, reg(rs1), reg(rs2))
        },
        Instruction::Bge { rs1, rs2, imm } => {
            encode_btype(0x63, check_b_imm(imm)?, 0x5, reg(rs1), reg(rs2))
        },
        Instruction::Bltu { rs1, rs2, imm } => {
            encode_btype(0x63, check_b_imm(imm)?, 0x6, reg(rs1), reg(rs2))
        },
        Instruction::Bgeu { rs1, rs2, imm } => {
            encode_btype(0x63, check_b_imm(imm)?, 0x7, reg(rs1), reg(rs2))
        },
        Instruction::Lb { rd, rs1, imm } => {
            encode_itype(0x03, reg(rd), 0x0, reg(rs1), check_i12(imm)?)
        },
        Instruction::Lh { rd, rs1, imm } => {
            encode_itype(0x03, reg(rd), 0x1, reg(rs1), check_i12(imm)?)
        },
        Instruction::Lw { rd, rs1, imm } => {
            encode_itype(0x03, reg(rd), 0x2, reg(rs1), check_i12(imm)?)
        },
        Instruction::Ld { rd, rs1, imm } => {
            encode_itype(0x03, reg(rd), 0x3, reg(rs1), check_i12(imm)?)
        },
        Instruction::Lbu { rd, rs1, imm } => {
            encode_itype(0x03, reg(rd), 0x4, reg(rs1), check_i12(imm)?)
        },
        Instruction::Lhu { rd, rs1, imm } => {
            encode_itype(0x03, reg(rd), 0x5, reg(rs1), check_i12(imm)?)
        },
        Instruction::Lwu { rd, rs1, imm } => {
            encode_itype(0x03, reg(rd), 0x6, reg(rs1), check_i12(imm)?)
        },
        Instruction::Sb { rs1, rs2, imm } => {
            encode_stype(0x23, check_i12(imm)?, 0x0, reg(rs1), reg(rs2))
        },
        Instruction::Sh { rs1, rs2, imm } => {
            encode_stype(0x23, check_i12(imm)?, 0x1, reg(rs1), reg(rs2))
        },
        Instruction::Sw { rs1, rs2, imm } => {
            encode_stype(0x23, check_i12(imm)?, 0x2, reg(rs1), reg(rs2))
        },
        Instruction::Sd { rs1, rs2, imm } => {
            encode_stype(0x23, check_i12(imm)?, 0x3, reg(rs1), reg(rs2))
        },
        Instruction::Addi { rd, rs1, imm } => {
            encode_itype(0x13, reg(rd), 0x0, reg(rs1), check_i12(imm)?)
        },
        Instruction::Slti { rd, rs1, imm } => {
            encode_itype(0x13, reg(rd), 0x2, reg(rs1), check_i12(imm)?)
        },
        Instruction::Sltiu { rd, rs1, imm } => {
            encode_itype(0x13, reg(rd), 0x3, reg(rs1), check_i12(imm)?)
        },
        Instruction::Xori { rd, rs1, imm } => {
            encode_itype(0x13, reg(rd), 0x4, reg(rs1), check_i12(imm)?)
        },
        Instruction::Ori { rd, rs1, imm } => {
            encode_itype(0x13, reg(rd), 0x6, reg(rs1), check_i12(imm)?)
        },
        Instruction::Andi { rd, rs1, imm } => {
            encode_itype(0x13, reg(rd), 0x7, reg(rs1), check_i12(imm)?)
        },
        Instruction::Slli { rd, rs1, shamt } => {
            encode_itype(0x13, reg(rd), 0x1, reg(rs1), check_shamt6(shamt)?)
        },
        Instruction::Srli { rd, rs1, shamt } => {
            encode_itype(0x13, reg(rd), 0x5, reg(rs1), check_shamt6(shamt)?)
        },
        Instruction::Srai { rd, rs1, shamt } => encode_itype(
            0x13,
            reg(rd),
            0x5,
            reg(rs1),
            check_shamt6(shamt)? | (0b01_0000 << 6),
        ),
        Instruction::Addiw { rd, rs1, imm } => {
            encode_itype(0x1B, reg(rd), 0x0, reg(rs1), check_i12(imm)?)
        },
        Instruction::Slliw { rd, rs1, shamt } => {
            encode_itype(0x1B, reg(rd), 0x1, reg(rs1), check_shamt5(shamt)?)
        },
        Instruction::Srliw { rd, rs1, shamt } => {
            encode_itype(0x1B, reg(rd), 0x5, reg(rs1), check_shamt5(shamt)?)
        },
        Instruction::Sraiw { rd, rs1, shamt } => encode_itype(
            0x1B,
            reg(rd),
            0x5,
            reg(rs1),
            check_shamt5(shamt)? | (0b010_0000 << 5),
        ),
        Instruction::Add { rd, rs1, rs2 } => {
            encode_rtype(0x33, reg(rd), 0x0, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::Sub { rd, rs1, rs2 } => {
            encode_rtype(0x33, reg(rd), 0x0, reg(rs1), reg(rs2), 0x20)
        },
        Instruction::Sll { rd, rs1, rs2 } => {
            encode_rtype(0x33, reg(rd), 0x1, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::Slt { rd, rs1, rs2 } => {
            encode_rtype(0x33, reg(rd), 0x2, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::Sltu { rd, rs1, rs2 } => {
            encode_rtype(0x33, reg(rd), 0x3, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::Xor { rd, rs1, rs2 } => {
            encode_rtype(0x33, reg(rd), 0x4, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::Srl { rd, rs1, rs2 } => {
            encode_rtype(0x33, reg(rd), 0x5, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::Sra { rd, rs1, rs2 } => {
            encode_rtype(0x33, reg(rd), 0x5, reg(rs1), reg(rs2), 0x20)
        },
        Instruction::Or { rd, rs1, rs2 } => {
            encode_rtype(0x33, reg(rd), 0x6, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::And { rd, rs1, rs2 } => {
            encode_rtype(0x33, reg(rd), 0x7, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::Addw { rd, rs1, rs2 } => {
            encode_rtype(0x3B, reg(rd), 0x0, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::Subw { rd, rs1, rs2 } => {
            encode_rtype(0x3B, reg(rd), 0x0, reg(rs1), reg(rs2), 0x20)
        },
        Instruction::Sllw { rd, rs1, rs2 } => {
            encode_rtype(0x3B, reg(rd), 0x1, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::Srlw { rd, rs1, rs2 } => {
            encode_rtype(0x3B, reg(rd), 0x5, reg(rs1), reg(rs2), 0x00)
        },
        Instruction::Sraw { rd, rs1, rs2 } => {
            encode_rtype(0x3B, reg(rd), 0x5, reg(rs1), reg(rs2), 0x20)
        },
        Instruction::Fence { pred, succ } => {
            encode_ftype(0x0F, u32::from(succ & 0xF), u32::from(pred & 0xF))
        },
        Instruction::Ecall => encode_etype(0x73, 0x000),
        Instruction::Ebreak => encode_etype(0x73, 0x001),
    };

    Ok(word)
}

#[inline]
const fn reg(r: Register) -> u32 {
    (r.0 & 0x1F) as u32
}

#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "upper 20 bits are extracted via shift and mask"
)]
const fn upper_imm20(imm: i64) -> u32 {
    ((imm.cast_unsigned() as u32) >> 12) & 0x000F_FFFF
}

#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "range-checked to signed 12-bit integer"
)]
const fn check_i12(imm: i64) -> Result<u32, EncodeError> {
    if imm < -2048 || imm > 2047 {
        Err(EncodeError::ImmediateOutOfRange)
    } else {
        Ok((imm as i32).cast_unsigned() & 0xFFF)
    }
}

#[inline]
const fn check_shamt5(shamt: u8) -> Result<u32, EncodeError> {
    if shamt >= 32 {
        Err(EncodeError::ImmediateOutOfRange)
    } else {
        Ok(shamt as u32)
    }
}

#[inline]
const fn check_shamt6(shamt: u8) -> Result<u32, EncodeError> {
    if shamt >= 64 {
        Err(EncodeError::ImmediateOutOfRange)
    } else {
        Ok(shamt as u32)
    }
}

#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "range-checked to signed 13-bit even offset"
)]
const fn check_b_imm(imm: i64) -> Result<u32, EncodeError> {
    if imm % 2 != 0 {
        return Err(EncodeError::UnalignedBranchTarget);
    }
    if imm < -4096 || imm > 4094 {
        return Err(EncodeError::ImmediateOutOfRange);
    }
    Ok((imm as i32).cast_unsigned())
}

#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "range-checked to signed 21-bit even offset"
)]
const fn check_j_imm(imm: i64) -> Result<u32, EncodeError> {
    if imm % 2 != 0 {
        return Err(EncodeError::UnalignedBranchTarget);
    }
    if imm < -1_048_576 || imm > 1_048_574 {
        return Err(EncodeError::ImmediateOutOfRange);
    }
    Ok((imm as i32).cast_unsigned())
}

const fn encode_rtype(opcode: u32, rd: u32, funct3: u32, rs1: u32, rs2: u32, funct7: u32) -> u32 {
    (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}

const fn encode_itype(opcode: u32, rd: u32, funct3: u32, rs1: u32, imm: u32) -> u32 {
    (imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}

const fn encode_stype(opcode: u32, imm: u32, funct3: u32, rs1: u32, rs2: u32) -> u32 {
    let imm_lo = (imm & 0x1F) << 7;
    let imm_hi = ((imm >> 5) & 0x7F) << 25;
    imm_hi | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | imm_lo | opcode
}

const fn encode_btype(opcode: u32, imm: u32, funct3: u32, rs1: u32, rs2: u32) -> u32 {
    let bit_11 = ((imm >> 11) & 0x1) << 7;
    let bits_4_1 = ((imm >> 1) & 0xF) << 8;
    let bits_10_5 = ((imm >> 5) & 0x3F) << 25;
    let bit_12 = ((imm >> 12) & 0x1) << 31;
    bit_12 | bits_10_5 | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | bits_4_1 | bit_11 | opcode
}

const fn encode_jtype(opcode: u32, rd: u32, imm: u32) -> u32 {
    let bits_19_12 = ((imm >> 12) & 0xFF) << 12;
    let bit_11 = ((imm >> 11) & 0x1) << 20;
    let bits_10_1 = ((imm >> 1) & 0x3FF) << 21;
    let bit_20 = ((imm >> 20) & 0x1) << 31;
    bit_20 | bits_10_1 | bit_11 | bits_19_12 | (rd << 7) | opcode
}

const fn encode_utype(opcode: u32, rd: u32, imm20: u32) -> u32 {
    (imm20 << 12) | (rd << 7) | opcode
}

const fn encode_ftype(opcode: u32, succ: u32, pred: u32) -> u32 {
    (pred << 24) | (succ << 20) | opcode
}

const fn encode_etype(opcode: u32, funct12: u32) -> u32 {
    (funct12 << 20) | opcode
}
