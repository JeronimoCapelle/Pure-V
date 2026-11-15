//! CPU core state, instruction decoding, and stateless datapath helpers.

pub mod alu;
pub mod branch;
pub mod csr;
pub mod decode;
pub mod regs;

use purev_isa::{Addr, DRAM_BASE, INITIAL_SP, IsaConfig, Register};

use self::{
    csr::{CsrFile, PrivilegeMode},
    regs::RegFile,
};

/// Architectural CPU core state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuCore {
    /// General-purpose integer registers (`x0..=x31`).
    pub regs: RegFile,
    /// Program counter.
    pub pc: Addr,
    /// Current architectural privilege mode.
    pub mode: PrivilegeMode,
    /// Control and status registers.
    pub csrs: CsrFile,
    /// Runtime-enabled ISA extensions.
    pub isa: IsaConfig,
}

impl Default for CpuCore {
    fn default() -> Self {
        Self::new(DRAM_BASE, IsaConfig::RV64IMAC)
    }
}

impl CpuCore {
    /// Creates a new [`CpuCore`] initialized at `entry_pc` in Machine mode with
    /// `sp = INITIAL_SP`.
    pub const fn new(entry_pc: Addr, isa: IsaConfig) -> Self {
        let mut regs = RegFile::new();
        regs.write(Register::SP, INITIAL_SP);
        Self {
            regs,
            pc: entry_pc,
            mode: PrivilegeMode::Machine,
            csrs: CsrFile::new(),
            isa,
        }
    }
}
