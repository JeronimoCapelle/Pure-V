//! Privilege modes and Control and Status Registers (Milestone 5).

/// RISC-V architectural privilege mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrivilegeMode {
    /// Unprivileged user mode (`U`, encoding `0b00`).
    User = 0,
    /// Supervisor mode (`S`, encoding `0b01`).
    Supervisor = 1,
    /// Machine mode (`M`, encoding `0b11`).
    #[default]
    Machine = 3,
}

/// Control and Status Register file (stubbed in Milestone 1, populated in
/// Milestone 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CsrFile {
    /// Machine status register (`mstatus`).
    pub mstatus: u64,
    /// Supervisor address translation and protection (`satp`).
    pub satp: u64,
}

impl CsrFile {
    /// Creates a reset-state [`CsrFile`].
    pub const fn new() -> Self {
        Self {
            mstatus: 0,
            satp: 0,
        }
    }
}
