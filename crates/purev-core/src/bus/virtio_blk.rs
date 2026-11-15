//! VirtIO-Block MMIO virtual disk (`0x1000_1000 .. 0x1000_2000`, Milestone 6).

/// VirtIO-Block MMIO device stub (implemented in Milestone 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VirtioBlk;

impl VirtioBlk {
    /// Creates a new [`VirtioBlk`] stub.
    pub const fn new() -> Self {
        Self
    }
}
