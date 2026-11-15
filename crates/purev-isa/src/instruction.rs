//! Decoded RISC-V instruction representation.

use crate::Register;

/// Decoded RV64IMAC instruction variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// `lui rd, imm`: Load Upper Immediate (`imm` is sign-extended `imm20 <<
    /// 12`).
    Lui {
        /// Destination register.
        rd: Register,
        /// Sign-extended upper immediate (`bits 31:12 << 12`).
        imm: i64,
    },
    /// `auipc rd, imm`: Add Upper Immediate to PC (`imm` is sign-extended
    /// `imm20 << 12`).
    Auipc {
        /// Destination register.
        rd: Register,
        /// Sign-extended upper immediate (`bits 31:12 << 12`).
        imm: i64,
    },
    /// `jal rd, offset`: Jump and Link.
    Jal {
        /// Link destination register.
        rd: Register,
        /// Sign-extended PC-relative jump offset in bytes.
        imm: i64,
    },
    /// `jalr rd, offset(rs1)`: Jump and Link Register.
    Jalr {
        /// Link destination register.
        rd: Register,
        /// Base address register.
        rs1: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `beq rs1, rs2, offset`: Branch if Equal.
    Beq {
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
        /// Sign-extended PC-relative branch offset in bytes.
        imm: i64,
    },
    /// `bne rs1, rs2, offset`: Branch if Not Equal.
    Bne {
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
        /// Sign-extended PC-relative branch offset in bytes.
        imm: i64,
    },
    /// `blt rs1, rs2, offset`: Branch if Less Than (signed).
    Blt {
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
        /// Sign-extended PC-relative branch offset in bytes.
        imm: i64,
    },
    /// `bge rs1, rs2, offset`: Branch if Greater Than or Equal (signed).
    Bge {
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
        /// Sign-extended PC-relative branch offset in bytes.
        imm: i64,
    },
    /// `bltu rs1, rs2, offset`: Branch if Less Than (unsigned).
    Bltu {
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
        /// Sign-extended PC-relative branch offset in bytes.
        imm: i64,
    },
    /// `bgeu rs1, rs2, offset`: Branch if Greater Than or Equal (unsigned).
    Bgeu {
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
        /// Sign-extended PC-relative branch offset in bytes.
        imm: i64,
    },
    /// `lb rd, offset(rs1)`: Load 8-bit Byte (sign-extended).
    Lb {
        /// Destination register.
        rd: Register,
        /// Base address register.
        rs1: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `lh rd, offset(rs1)`: Load 16-bit Halfword (sign-extended).
    Lh {
        /// Destination register.
        rd: Register,
        /// Base address register.
        rs1: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `lw rd, offset(rs1)`: Load 32-bit Word (sign-extended).
    Lw {
        /// Destination register.
        rd: Register,
        /// Base address register.
        rs1: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `ld rd, offset(rs1)`: Load 64-bit Doubleword.
    Ld {
        /// Destination register.
        rd: Register,
        /// Base address register.
        rs1: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `lbu rd, offset(rs1)`: Load 8-bit Byte (zero-extended).
    Lbu {
        /// Destination register.
        rd: Register,
        /// Base address register.
        rs1: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `lhu rd, offset(rs1)`: Load 16-bit Halfword (zero-extended).
    Lhu {
        /// Destination register.
        rd: Register,
        /// Base address register.
        rs1: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `lwu rd, offset(rs1)`: Load 32-bit Word (zero-extended).
    Lwu {
        /// Destination register.
        rd: Register,
        /// Base address register.
        rs1: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `sb rs2, offset(rs1)`: Store 8-bit Byte.
    Sb {
        /// Base address register.
        rs1: Register,
        /// Source value register.
        rs2: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `sh rs2, offset(rs1)`: Store 16-bit Halfword.
    Sh {
        /// Base address register.
        rs1: Register,
        /// Source value register.
        rs2: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `sw rs2, offset(rs1)`: Store 32-bit Word.
    Sw {
        /// Base address register.
        rs1: Register,
        /// Source value register.
        rs2: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `sd rs2, offset(rs1)`: Store 64-bit Doubleword.
    Sd {
        /// Base address register.
        rs1: Register,
        /// Source value register.
        rs2: Register,
        /// Sign-extended 12-bit byte offset.
        imm: i64,
    },
    /// `addi rd, rs1, imm`: Add Immediate (64-bit).
    Addi {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Sign-extended 12-bit immediate.
        imm: i64,
    },
    /// `slti rd, rs1, imm`: Set Less Than Immediate (signed).
    Slti {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Sign-extended 12-bit immediate.
        imm: i64,
    },
    /// `sltiu rd, rs1, imm`: Set Less Than Immediate (unsigned).
    Sltiu {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Sign-extended 12-bit immediate.
        imm: i64,
    },
    /// `xori rd, rs1, imm`: Bitwise XOR Immediate.
    Xori {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Sign-extended 12-bit immediate.
        imm: i64,
    },
    /// `ori rd, rs1, imm`: Bitwise OR Immediate.
    Ori {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Sign-extended 12-bit immediate.
        imm: i64,
    },
    /// `andi rd, rs1, imm`: Bitwise AND Immediate.
    Andi {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Sign-extended 12-bit immediate.
        imm: i64,
    },
    /// `slli rd, rs1, shamt`: Shift Left Logical Immediate (6-bit `shamt`).
    Slli {
        /// Destination register.
        rd: Register,
        /// Source register.
        rs1: Register,
        /// 6-bit shift amount (`0..64`).
        shamt: u8,
    },
    /// `srli rd, rs1, shamt`: Shift Right Logical Immediate (6-bit `shamt`).
    Srli {
        /// Destination register.
        rd: Register,
        /// Source register.
        rs1: Register,
        /// 6-bit shift amount (`0..64`).
        shamt: u8,
    },
    /// `srai rd, rs1, shamt`: Shift Right Arithmetic Immediate (6-bit `shamt`).
    Srai {
        /// Destination register.
        rd: Register,
        /// Source register.
        rs1: Register,
        /// 6-bit shift amount (`0..64`).
        shamt: u8,
    },
    /// `addiw rd, rs1, imm`: Add Immediate Word (sign-extended to 64-bit).
    Addiw {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Sign-extended 12-bit immediate.
        imm: i64,
    },
    /// `slliw rd, rs1, shamt`: Shift Left Logical Immediate Word (5-bit
    /// `shamt`).
    Slliw {
        /// Destination register.
        rd: Register,
        /// Source register.
        rs1: Register,
        /// 5-bit shift amount (`0..32`).
        shamt: u8,
    },
    /// `srliw rd, rs1, shamt`: Shift Right Logical Immediate Word (5-bit
    /// `shamt`).
    Srliw {
        /// Destination register.
        rd: Register,
        /// Source register.
        rs1: Register,
        /// 5-bit shift amount (`0..32`).
        shamt: u8,
    },
    /// `sraiw rd, rs1, shamt`: Shift Right Arithmetic Immediate Word (5-bit
    /// `shamt`).
    Sraiw {
        /// Destination register.
        rd: Register,
        /// Source register.
        rs1: Register,
        /// 5-bit shift amount (`0..32`).
        shamt: u8,
    },
    /// `add rd, rs1, rs2`: Add (64-bit).
    Add {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
    },
    /// `sub rd, rs1, rs2`: Subtract (64-bit).
    Sub {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
    },
    /// `sll rd, rs1, rs2`: Shift Left Logical (64-bit).
    Sll {
        /// Destination register.
        rd: Register,
        /// Value register.
        rs1: Register,
        /// Shift amount register (low 6 bits).
        rs2: Register,
    },
    /// `slt rd, rs1, rs2`: Set Less Than (signed).
    Slt {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
    },
    /// `sltu rd, rs1, rs2`: Set Less Than (unsigned).
    Sltu {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
    },
    /// `xor rd, rs1, rs2`: Bitwise XOR (64-bit).
    Xor {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
    },
    /// `srl rd, rs1, rs2`: Shift Right Logical (64-bit).
    Srl {
        /// Destination register.
        rd: Register,
        /// Value register.
        rs1: Register,
        /// Shift amount register (low 6 bits).
        rs2: Register,
    },
    /// `sra rd, rs1, rs2`: Shift Right Arithmetic (64-bit).
    Sra {
        /// Destination register.
        rd: Register,
        /// Value register.
        rs1: Register,
        /// Shift amount register (low 6 bits).
        rs2: Register,
    },
    /// `or rd, rs1, rs2`: Bitwise OR (64-bit).
    Or {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
    },
    /// `and rd, rs1, rs2`: Bitwise AND (64-bit).
    And {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
    },
    /// `addw rd, rs1, rs2`: Add Word (sign-extended to 64-bit).
    Addw {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
    },
    /// `subw rd, rs1, rs2`: Subtract Word (sign-extended to 64-bit).
    Subw {
        /// Destination register.
        rd: Register,
        /// First source register.
        rs1: Register,
        /// Second source register.
        rs2: Register,
    },
    /// `sllw rd, rs1, rs2`: Shift Left Logical Word (sign-extended to 64-bit).
    Sllw {
        /// Destination register.
        rd: Register,
        /// Value register.
        rs1: Register,
        /// Shift amount register (low 5 bits).
        rs2: Register,
    },
    /// `srlw rd, rs1, rs2`: Shift Right Logical Word (sign-extended to 64-bit).
    Srlw {
        /// Destination register.
        rd: Register,
        /// Value register.
        rs1: Register,
        /// Shift amount register (low 5 bits).
        rs2: Register,
    },
    /// `sraw rd, rs1, rs2`: Shift Right Arithmetic Word (sign-extended to
    /// 64-bit).
    Sraw {
        /// Destination register.
        rd: Register,
        /// Value register.
        rs1: Register,
        /// Shift amount register (low 5 bits).
        rs2: Register,
    },
    /// `fence pred, succ`: Memory ordering fence.
    Fence {
        /// Predecessor set (`bits 3:0` = `iorw`).
        pred: u8,
        /// Successor set (`bits 3:0` = `iorw`).
        succ: u8,
    },
    /// `ecall`: Environment Call.
    Ecall,
    /// `ebreak`: Environment Breakpoint.
    Ebreak,
}
