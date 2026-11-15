//! Instruction decoder (`raw_bits -> Result<(Instruction, inst_len), Trap>`).

use purev_isa::{Instruction, IsaConfig, Register, Trap};

/// Extracts a 5-bit [`Register`] field starting at bit `shift`.
#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "value is masked to 5 bits (0..32)"
)]
const fn reg_field(raw_bits: u32, shift: u32) -> Register {
    Register(((raw_bits >> shift) & 0x1F) as u8)
}

/// Extracts a masked bitfield as `u8`.
#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "caller masks value to <= 6 bits"
)]
const fn bits_u8(raw_bits: u32, shift: u32, mask: u32) -> u8 {
    ((raw_bits >> shift) & mask) as u8
}

#[inline]
fn imm_i(raw_bits: u32) -> i64 {
    i64::from(raw_bits.cast_signed() >> 20)
}

#[inline]
fn imm_s(raw_bits: u32) -> i64 {
    let imm = ((raw_bits >> 20) & 0x0FE0) | ((raw_bits >> 7) & 0x001F);
    i64::from((imm.cast_signed() << 20) >> 20)
}

#[inline]
fn imm_b(raw_bits: u32) -> i64 {
    let imm = ((raw_bits >> 19) & 0x1000)
        | ((raw_bits << 4) & 0x0800)
        | ((raw_bits >> 20) & 0x07E0)
        | ((raw_bits >> 7) & 0x001E);
    i64::from((imm.cast_signed() << 19) >> 19)
}

#[inline]
fn imm_u(raw_bits: u32) -> i64 {
    i64::from((raw_bits & 0xFFFF_F000).cast_signed())
}

#[inline]
fn imm_j(raw_bits: u32) -> i64 {
    let imm = ((raw_bits >> 11) & 0x0010_0000)
        | (raw_bits & 0x000F_F000)
        | ((raw_bits >> 9) & 0x0800)
        | ((raw_bits >> 20) & 0x07FE);
    i64::from((imm.cast_signed() << 11) >> 11)
}

/// Decodes a raw instruction word according to the active [`IsaConfig`].
///
/// Returns `(Instruction, inst_len)` where `inst_len` is `4` for base 32-bit
/// instructions and `2` for compressed `RV64C` instructions.
pub fn decode(raw_bits: u32, isa: IsaConfig) -> Result<(Instruction, u64), Trap> {
    if (raw_bits & 0b11) != 0b11 {
        let _ = isa.c;
        return Err(Trap::IllegalInstruction(raw_bits));
    }

    let opcode = raw_bits & 0x7F;
    let rd = reg_field(raw_bits, 7);
    let funct3 = (raw_bits >> 12) & 0x7;
    let rs1 = reg_field(raw_bits, 15);
    let rs2 = reg_field(raw_bits, 20);
    let funct7 = (raw_bits >> 25) & 0x7F;
    let funct6 = (raw_bits >> 26) & 0x3F;

    let inst = match opcode {
        // LUI
        0x37 => Instruction::Lui {
            rd,
            imm: imm_u(raw_bits),
        },
        // AUIPC
        0x17 => Instruction::Auipc {
            rd,
            imm: imm_u(raw_bits),
        },
        // JAL
        0x6F => Instruction::Jal {
            rd,
            imm: imm_j(raw_bits),
        },
        // JALR
        0x67 if funct3 == 0 => Instruction::Jalr {
            rd,
            rs1,
            imm: imm_i(raw_bits),
        },
        // BRANCH
        0x63 => match funct3 {
            0x0 => Instruction::Beq {
                rs1,
                rs2,
                imm: imm_b(raw_bits),
            },
            0x1 => Instruction::Bne {
                rs1,
                rs2,
                imm: imm_b(raw_bits),
            },
            0x4 => Instruction::Blt {
                rs1,
                rs2,
                imm: imm_b(raw_bits),
            },
            0x5 => Instruction::Bge {
                rs1,
                rs2,
                imm: imm_b(raw_bits),
            },
            0x6 => Instruction::Bltu {
                rs1,
                rs2,
                imm: imm_b(raw_bits),
            },
            0x7 => Instruction::Bgeu {
                rs1,
                rs2,
                imm: imm_b(raw_bits),
            },
            _ => return Err(Trap::IllegalInstruction(raw_bits)),
        },
        // LOAD
        0x03 => match funct3 {
            0x0 => Instruction::Lb {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x1 => Instruction::Lh {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x2 => Instruction::Lw {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x3 => Instruction::Ld {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x4 => Instruction::Lbu {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x5 => Instruction::Lhu {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x6 => Instruction::Lwu {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            _ => return Err(Trap::IllegalInstruction(raw_bits)),
        },
        // STORE
        0x23 => match funct3 {
            0x0 => Instruction::Sb {
                rs1,
                rs2,
                imm: imm_s(raw_bits),
            },
            0x1 => Instruction::Sh {
                rs1,
                rs2,
                imm: imm_s(raw_bits),
            },
            0x2 => Instruction::Sw {
                rs1,
                rs2,
                imm: imm_s(raw_bits),
            },
            0x3 => Instruction::Sd {
                rs1,
                rs2,
                imm: imm_s(raw_bits),
            },
            _ => return Err(Trap::IllegalInstruction(raw_bits)),
        },
        // OP-IMM
        0x13 => match funct3 {
            0x0 => Instruction::Addi {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x1 if funct6 == 0x00 => Instruction::Slli {
                rd,
                rs1,
                shamt: bits_u8(raw_bits, 20, 0x3F),
            },
            0x2 => Instruction::Slti {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x3 => Instruction::Sltiu {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x4 => Instruction::Xori {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x5 if funct6 == 0x00 => Instruction::Srli {
                rd,
                rs1,
                shamt: bits_u8(raw_bits, 20, 0x3F),
            },
            0x5 if funct6 == 0x10 => Instruction::Srai {
                rd,
                rs1,
                shamt: bits_u8(raw_bits, 20, 0x3F),
            },
            0x6 => Instruction::Ori {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x7 => Instruction::Andi {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            _ => return Err(Trap::IllegalInstruction(raw_bits)),
        },
        // OP-IMM-32 (bit 25 must be 0 for *iw shifts)
        0x1B => match funct3 {
            0x0 => Instruction::Addiw {
                rd,
                rs1,
                imm: imm_i(raw_bits),
            },
            0x1 if funct7 == 0x00 => Instruction::Slliw {
                rd,
                rs1,
                shamt: bits_u8(raw_bits, 20, 0x1F),
            },
            0x5 if funct7 == 0x00 => Instruction::Srliw {
                rd,
                rs1,
                shamt: bits_u8(raw_bits, 20, 0x1F),
            },
            0x5 if funct7 == 0x20 => Instruction::Sraiw {
                rd,
                rs1,
                shamt: bits_u8(raw_bits, 20, 0x1F),
            },
            _ => return Err(Trap::IllegalInstruction(raw_bits)),
        },
        // OP
        0x33 => match (funct3, funct7) {
            (0x0, 0x00) => Instruction::Add { rd, rs1, rs2 },
            (0x0, 0x20) => Instruction::Sub { rd, rs1, rs2 },
            (0x1, 0x00) => Instruction::Sll { rd, rs1, rs2 },
            (0x2, 0x00) => Instruction::Slt { rd, rs1, rs2 },
            (0x3, 0x00) => Instruction::Sltu { rd, rs1, rs2 },
            (0x4, 0x00) => Instruction::Xor { rd, rs1, rs2 },
            (0x5, 0x00) => Instruction::Srl { rd, rs1, rs2 },
            (0x5, 0x20) => Instruction::Sra { rd, rs1, rs2 },
            (0x6, 0x00) => Instruction::Or { rd, rs1, rs2 },
            (0x7, 0x00) => Instruction::And { rd, rs1, rs2 },
            _ => return Err(Trap::IllegalInstruction(raw_bits)),
        },
        // OP-32
        0x3B => match (funct3, funct7) {
            (0x0, 0x00) => Instruction::Addw { rd, rs1, rs2 },
            (0x0, 0x20) => Instruction::Subw { rd, rs1, rs2 },
            (0x1, 0x00) => Instruction::Sllw { rd, rs1, rs2 },
            (0x5, 0x00) => Instruction::Srlw { rd, rs1, rs2 },
            (0x5, 0x20) => Instruction::Sraw { rd, rs1, rs2 },
            _ => return Err(Trap::IllegalInstruction(raw_bits)),
        },
        // MISC-MEM (FENCE)
        0x0F if funct3 == 0x0 => Instruction::Fence {
            pred: bits_u8(raw_bits, 24, 0xF),
            succ: bits_u8(raw_bits, 20, 0xF),
        },
        // SYSTEM (ECALL / EBREAK)
        0x73 if raw_bits == 0x0000_0073 => Instruction::Ecall,
        0x73 if raw_bits == 0x0010_0073 => Instruction::Ebreak,
        _ => return Err(Trap::IllegalInstruction(raw_bits)),
    };

    Ok((inst, 4))
}
