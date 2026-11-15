//! Core Local Interruptor (`0x0200_0000 .. 0x0201_0000`, Milestone 5).

/// CLINT timer and software interrupt controller stub (implemented in Milestone
/// 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Clint {
    /// Machine-mode `mtime` counter.
    pub mtime: u64,
    /// Machine-mode `mtimecmp` comparator.
    pub mtimecmp: u64,
    /// Machine software interrupt pending flag (`msip`).
    pub msip: u32,
}

impl Clint {
    /// Creates a reset-state [`Clint`].
    pub const fn new() -> Self {
        Self {
            mtime: 0,
            mtimecmp: 0,
            msip: 0,
        }
    }
}
