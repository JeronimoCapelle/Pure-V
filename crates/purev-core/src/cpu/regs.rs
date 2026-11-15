//! General-purpose integer register file (`x0..=x31`).

use purev_isa::{RegValue, Register};

/// 32-entry 64-bit RISC-V integer register file enforcing `x0 == 0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RegFile([RegValue; 32]);

impl RegFile {
    /// Creates a zero-initialized [`RegFile`].
    pub const fn new() -> Self {
        Self([0; 32])
    }

    /// Reads register `reg`. Reading `x0` always returns `0`.
    #[inline]
    pub const fn read(&self, reg: Register) -> RegValue {
        if reg.0 == 0 || reg.0 >= 32 {
            0
        } else {
            self.0[reg.0 as usize]
        }
    }

    /// Writes `val` to `reg`. Writes to `x0` are silent no-ops and never trap.
    #[inline]
    pub const fn write(&mut self, reg: Register, val: RegValue) {
        if reg.0 != 0 && reg.0 < 32 {
            self.0[reg.0 as usize] = val;
        }
    }
}
