//! Platform-Level Interrupt Controller (`0x0C00_0000 .. 0x1000_0000`, Milestone
//! 5).

/// Minimal single-hart PLIC stub (implemented in Milestone 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Plic;

impl Plic {
    /// Creates a new [`Plic`] stub.
    pub const fn new() -> Self {
        Self
    }
}
