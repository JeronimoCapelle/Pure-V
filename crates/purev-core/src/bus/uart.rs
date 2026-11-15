//! PC16550 UART MMIO controller (`0x1000_0000 .. 0x1000_0100`, Milestone 3).

/// 16550 UART device stub (implemented in Milestone 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Uart16550;

impl Uart16550 {
    /// Creates a new [`Uart16550`] stub.
    pub const fn new() -> Self {
        Self
    }
}
