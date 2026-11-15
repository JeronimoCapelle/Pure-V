//! Fixed-size stack undo journal for atomic instruction rollback on traps.

use purev_isa::{Addr, RegValue, Register};

use crate::{bus::Memory, cpu::regs::RegFile};

/// Maximum number of state mutations recorded per instruction.
pub const JOURNAL_CAPACITY: usize = 8;

/// A reversible state mutation logged prior to in-place execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UndoAction {
    /// Previous value of a general-purpose register.
    Register {
        /// Mutated register.
        reg: Register,
        /// Value before the instruction wrote to `reg`.
        old_val: RegValue,
    },
    /// Previous 8-bit value in memory.
    Memory8 {
        /// Physical address.
        addr: Addr,
        /// Byte value before write.
        old_val: u8,
    },
    /// Previous 16-bit value in memory.
    Memory16 {
        /// Physical address.
        addr: Addr,
        /// Halfword value before write.
        old_val: u16,
    },
    /// Previous 32-bit value in memory.
    Memory32 {
        /// Physical address.
        addr: Addr,
        /// Word value before write.
        old_val: u32,
    },
    /// Previous 64-bit value in memory.
    Memory64 {
        /// Physical address.
        addr: Addr,
        /// Doubleword value before write.
        old_val: u64,
    },
}

/// 8-slot stack-allocated undo buffer (zero heap allocations).
#[derive(Debug, Clone, Copy)]
pub struct Journal {
    entries: [Option<UndoAction>; JOURNAL_CAPACITY],
    len: usize,
}

impl Default for Journal {
    fn default() -> Self {
        Self::new()
    }
}

impl Journal {
    /// Creates an empty [`Journal`].
    pub const fn new() -> Self {
        Self {
            entries: [None; JOURNAL_CAPACITY],
            len: 0,
        }
    }

    /// Returns the number of currently recorded undo entries.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if no undo entries are currently recorded.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Records a reversible state mutation before executing an in-place write.
    #[inline]
    pub fn record(&mut self, action: UndoAction) -> bool {
        if let Some(slot) = self.entries.get_mut(self.len) {
            *slot = Some(action);
            self.len += 1;
            true
        } else {
            false
        }
    }

    /// Commits all mutations for the current step (single integer reset).
    #[inline]
    pub const fn commit(&mut self) {
        self.len = 0;
    }

    /// Rewinds all recorded mutations in reverse (LIFO) order on a trap.
    pub fn rollback<M: Memory>(&mut self, regs: &mut RegFile, mem: &mut M) {
        while self.len > 0 {
            self.len -= 1;
            if let Some(action) = self.entries[self.len].take() {
                match action {
                    UndoAction::Register { reg, old_val } => regs.write(reg, old_val),
                    UndoAction::Memory8 { addr, old_val } => {
                        let _ = mem.write8(addr, old_val);
                    },
                    UndoAction::Memory16 { addr, old_val } => {
                        let _ = mem.write16(addr, old_val);
                    },
                    UndoAction::Memory32 { addr, old_val } => {
                        let _ = mem.write32(addr, old_val);
                    },
                    UndoAction::Memory64 { addr, old_val } => {
                        let _ = mem.write64(addr, old_val);
                    },
                }
            }
        }
    }
}
