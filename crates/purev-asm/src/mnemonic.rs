//! RISC-V instruction mnemonic classifications (RV64I).

/// R-type register-register ALU mnemonics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ROp {
    /// `add`
    Add,
    /// `sub`
    Sub,
    /// `sll`
    Sll,
    /// `slt`
    Slt,
    /// `sltu`
    Sltu,
    /// `xor`
    Xor,
    /// `srl`
    Srl,
    /// `sra`
    Sra,
    /// `or`
    Or,
    /// `and`
    And,
    /// `addw`
    Addw,
    /// `subw`
    Subw,
    /// `sllw`
    Sllw,
    /// `srlw`
    Srlw,
    /// `sraw`
    Sraw,
}

/// I-type ALU and shift-immediate mnemonics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOp {
    /// `addi`
    Addi,
    /// `slti`
    Slti,
    /// `sltiu`
    Sltiu,
    /// `xori`
    Xori,
    /// `ori`
    Ori,
    /// `andi`
    Andi,
    /// `slli`
    Slli,
    /// `srli`
    Srli,
    /// `srai`
    Srai,
    /// `addiw`
    Addiw,
    /// `slliw`
    Slliw,
    /// `srliw`
    Srliw,
    /// `sraiw`
    Sraiw,
}

/// Load instruction mnemonics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadOp {
    /// `lb`
    Lb,
    /// `lh`
    Lh,
    /// `lw`
    Lw,
    /// `ld`
    Ld,
    /// `lbu`
    Lbu,
    /// `lhu`
    Lhu,
    /// `lwu`
    Lwu,
}

/// Store instruction mnemonics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreOp {
    /// `sb`
    Sb,
    /// `sh`
    Sh,
    /// `sw`
    Sw,
    /// `sd`
    Sd,
}

/// Conditional branch mnemonics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchOp {
    /// `beq`
    Beq,
    /// `bne`
    Bne,
    /// `blt`
    Blt,
    /// `bge`
    Bge,
    /// `bltu`
    Bltu,
    /// `bgeu`
    Bgeu,
}

/// Upper-immediate mnemonics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UOp {
    /// `lui`
    Lui,
    /// `auipc`
    Auipc,
}

/// System instruction mnemonics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysOp {
    /// `ecall`
    Ecall,
    /// `ebreak`
    Ebreak,
}

/// Parsed RISC-V instruction mnemonic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    /// R-type arithmetic/logic/shift instruction.
    R(ROp),
    /// I-type immediate arithmetic/logic/shift instruction.
    I(IOp),
    /// Load instruction.
    Load(LoadOp),
    /// Store instruction.
    Store(StoreOp),
    /// Branch instruction.
    Branch(BranchOp),
    /// Upper-immediate instruction.
    U(UOp),
    /// `jal` jump-and-link instruction.
    Jal,
    /// `jalr` jump-and-link-register instruction.
    Jalr,
    /// `fence` memory ordering instruction.
    Fence,
    /// System instruction (`ecall` / `ebreak`).
    Sys(SysOp),
}

impl Mnemonic {
    /// Looks up a [`Mnemonic`] from its lowercase assembly keyword.
    pub fn try_from_str(s: &str) -> Option<Self> {
        Some(match s {
            // R-type (64-bit)
            "add" => Self::R(ROp::Add),
            "sub" => Self::R(ROp::Sub),
            "sll" => Self::R(ROp::Sll),
            "slt" => Self::R(ROp::Slt),
            "sltu" => Self::R(ROp::Sltu),
            "xor" => Self::R(ROp::Xor),
            "srl" => Self::R(ROp::Srl),
            "sra" => Self::R(ROp::Sra),
            "or" => Self::R(ROp::Or),
            "and" => Self::R(ROp::And),

            // R-type (*W 32-bit word)
            "addw" => Self::R(ROp::Addw),
            "subw" => Self::R(ROp::Subw),
            "sllw" => Self::R(ROp::Sllw),
            "srlw" => Self::R(ROp::Srlw),
            "sraw" => Self::R(ROp::Sraw),

            // I-type (64-bit)
            "addi" => Self::I(IOp::Addi),
            "slti" => Self::I(IOp::Slti),
            "sltiu" => Self::I(IOp::Sltiu),
            "xori" => Self::I(IOp::Xori),
            "ori" => Self::I(IOp::Ori),
            "andi" => Self::I(IOp::Andi),
            "slli" => Self::I(IOp::Slli),
            "srli" => Self::I(IOp::Srli),
            "srai" => Self::I(IOp::Srai),

            // I-type (*W 32-bit word)
            "addiw" => Self::I(IOp::Addiw),
            "slliw" => Self::I(IOp::Slliw),
            "srliw" => Self::I(IOp::Srliw),
            "sraiw" => Self::I(IOp::Sraiw),

            // Loads
            "lb" => Self::Load(LoadOp::Lb),
            "lh" => Self::Load(LoadOp::Lh),
            "lw" => Self::Load(LoadOp::Lw),
            "ld" => Self::Load(LoadOp::Ld),
            "lbu" => Self::Load(LoadOp::Lbu),
            "lhu" => Self::Load(LoadOp::Lhu),
            "lwu" => Self::Load(LoadOp::Lwu),

            // Stores
            "sb" => Self::Store(StoreOp::Sb),
            "sh" => Self::Store(StoreOp::Sh),
            "sw" => Self::Store(StoreOp::Sw),
            "sd" => Self::Store(StoreOp::Sd),

            // Branches
            "beq" => Self::Branch(BranchOp::Beq),
            "bne" => Self::Branch(BranchOp::Bne),
            "blt" => Self::Branch(BranchOp::Blt),
            "bge" => Self::Branch(BranchOp::Bge),
            "bltu" => Self::Branch(BranchOp::Bltu),
            "bgeu" => Self::Branch(BranchOp::Bgeu),

            // Upper & Jumps
            "lui" => Self::U(UOp::Lui),
            "auipc" => Self::U(UOp::Auipc),
            "jal" => Self::Jal,
            "jalr" => Self::Jalr,

            // Fence & System
            "fence" => Self::Fence,
            "ecall" => Self::Sys(SysOp::Ecall),
            "ebreak" => Self::Sys(SysOp::Ebreak),

            _ => return None,
        })
    }
}
