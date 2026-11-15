//! System bus, physical memory traits, and MMIO device routing.

pub mod clint;
pub mod plic;
pub mod ram;
pub mod uart;
pub mod virtio_blk;

use purev_isa::{Addr, DRAM_BASE, DRAM_SIZE, RegValue, Trap};

use self::{clint::Clint, plic::Plic, ram::FlatRam, uart::Uart16550, virtio_blk::VirtioBlk};

/// Base physical address of CLINT (`64 KiB`).
pub const CLINT_BASE: Addr = 0x0200_0000;
/// Size of CLINT MMIO region (`64 KiB`).
pub const CLINT_SIZE: u64 = 0x0001_0000;

/// Base physical address of PLIC (`64 MiB`).
pub const PLIC_BASE: Addr = 0x0C00_0000;
/// Size of PLIC MMIO region (`64 MiB`).
pub const PLIC_SIZE: u64 = 0x0400_0000;

/// Base physical address of 16550 UART (`256 B`).
pub const UART_BASE: Addr = 0x1000_0000;
/// Size of 16550 UART MMIO region (`256 B`).
pub const UART_SIZE: u64 = 0x0000_0100;

/// Base physical address of VirtIO-Block MMIO (`4 KiB`).
pub const VIRTIO_BLK_BASE: Addr = 0x1000_1000;
/// Size of VirtIO-Block MMIO region (`4 KiB`).
pub const VIRTIO_BLK_SIZE: u64 = 0x0000_1000;

/// Host-agnostic serial console backend for the 16550 UART.
pub trait SerialBackend {
    /// Writes a single byte transmitted by the guest UART.
    fn write_byte(&mut self, byte: u8);
    /// Polls for an incoming byte from the host console.
    fn read_byte(&mut self) -> Option<u8>;
}

/// Host-agnostic 512-byte sector block device backend for VirtIO-Block.
pub trait BlockBackend {
    /// Reads a 512-byte sector into `buf`.
    #[allow(
        clippy::result_unit_err,
        reason = "matches ARCHITECTURE.md BlockBackend trait contract"
    )]
    fn read_sector(&self, sector: u64, buf: &mut [u8; 512]) -> Result<(), ()>;
    /// Writes a 512-byte sector from `buf`.
    #[allow(
        clippy::result_unit_err,
        reason = "matches ARCHITECTURE.md BlockBackend trait contract"
    )]
    fn write_sector(&mut self, sector: u64, buf: &[u8; 512]) -> Result<(), ()>;
    /// Returns total device capacity in 512-byte sectors.
    fn capacity_sectors(&self) -> u64;
}

/// Access width for high-level RISC-V load/store operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    /// 8-bit byte (`lb`/`lbu`/`sb`).
    Byte,
    /// 16-bit halfword (`lh`/`lhu`/`sh`).
    HalfWord,
    /// 32-bit word (`lw`/`lwu`/`sw`).
    Word,
    /// 64-bit doubleword (`ld`/`sd`).
    DoubleWord,
}

impl Width {
    /// Returns the natural alignment bitmask for this access width.
    #[inline]
    pub const fn alignment_mask(self) -> Addr {
        match self {
            Self::Byte => 0,
            Self::HalfWord => 0b001,
            Self::Word => 0b011,
            Self::DoubleWord => 0b111,
        }
    }
}

/// Physical memory and MMIO bus interface.
pub trait Memory {
    /// Reads an unsigned 8-bit byte at `addr`.
    fn read8(&self, addr: Addr) -> Result<u8, Trap>;
    /// Reads a little-endian 16-bit halfword at `addr`.
    fn read16(&self, addr: Addr) -> Result<u16, Trap>;
    /// Reads a little-endian 32-bit word at `addr`.
    fn read32(&self, addr: Addr) -> Result<u32, Trap>;
    /// Reads a little-endian 64-bit doubleword at `addr`.
    fn read64(&self, addr: Addr) -> Result<u64, Trap>;

    /// Writes an 8-bit byte at `addr`.
    fn write8(&mut self, addr: Addr, val: u8) -> Result<(), Trap>;
    /// Writes a little-endian 16-bit halfword at `addr`.
    fn write16(&mut self, addr: Addr, val: u16) -> Result<(), Trap>;
    /// Writes a little-endian 32-bit word at `addr`.
    fn write32(&mut self, addr: Addr, val: u32) -> Result<(), Trap>;
    /// Writes a little-endian 64-bit doubleword at `addr`.
    fn write64(&mut self, addr: Addr, val: u64) -> Result<(), Trap>;

    /// Loads a RISC-V value with natural alignment checks and sign/zero
    /// extension.
    fn load(&self, addr: Addr, width: Width, signed: bool) -> Result<RegValue, Trap> {
        if addr & width.alignment_mask() != 0 {
            return Err(Trap::LoadAddressMisaligned(addr));
        }

        match (width, signed) {
            (Width::Byte, true) => Ok(i64::from(self.read8(addr)?.cast_signed()).cast_unsigned()),
            (Width::Byte, false) => Ok(u64::from(self.read8(addr)?)),
            (Width::HalfWord, true) => {
                Ok(i64::from(self.read16(addr)?.cast_signed()).cast_unsigned())
            },
            (Width::HalfWord, false) => Ok(u64::from(self.read16(addr)?)),
            (Width::Word, true) => Ok(i64::from(self.read32(addr)?.cast_signed()).cast_unsigned()),
            (Width::Word, false) => Ok(u64::from(self.read32(addr)?)),
            (Width::DoubleWord, _) => self.read64(addr),
        }
    }

    /// Stores a RISC-V value with natural alignment checks and width
    /// truncation.
    #[allow(
        clippy::cast_possible_truncation,
        reason = "RISC-V sub-doubleword stores explicitly mask to the lower 8/16/32 bits"
    )]
    fn store(&mut self, addr: Addr, width: Width, val: RegValue) -> Result<(), Trap> {
        if addr & width.alignment_mask() != 0 {
            return Err(Trap::StoreAddressMisaligned(addr));
        }

        match width {
            Width::Byte => self.write8(addr, val as u8),
            Width::HalfWord => self.write16(addr, val as u16),
            Width::Word => self.write32(addr, val as u32),
            Width::DoubleWord => self.write64(addr, val),
        }
    }
}

/// Top-level physical address decoder and device bus.
#[derive(Debug, Clone)]
pub struct SystemBus {
    /// Backing 128 MiB DRAM.
    pub ram: FlatRam,
    /// Stubbed 16550 UART device (Milestone 3).
    pub uart: Uart16550,
    /// Stubbed CLINT device (Milestone 5).
    pub clint: Clint,
    /// Stubbed PLIC device (Milestone 5).
    pub plic: Plic,
    /// Stubbed VirtIO-Block device (Milestone 6).
    pub virtio_blk: VirtioBlk,
    /// Load-reserved / store-conditional reservation address (Milestone 4).
    pub reservation: Option<Addr>,
}

impl Default for SystemBus {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemBus {
    /// Creates a new [`SystemBus`] with zeroed 128 MiB DRAM and reset MMIO
    /// stubs.
    pub fn new() -> Self {
        Self {
            ram: FlatRam::new(),
            uart: Uart16550::new(),
            clint: Clint::new(),
            plic: Plic::new(),
            virtio_blk: VirtioBlk::new(),
            reservation: None,
        }
    }

    #[inline]
    const fn in_dram(addr: Addr) -> bool {
        addr >= DRAM_BASE && addr < DRAM_BASE + DRAM_SIZE
    }
}

impl Memory for SystemBus {
    fn read8(&self, addr: Addr) -> Result<u8, Trap> {
        if Self::in_dram(addr) {
            self.ram.read8(addr)
        } else {
            Err(Trap::LoadAccessFault(addr))
        }
    }

    fn read16(&self, addr: Addr) -> Result<u16, Trap> {
        if Self::in_dram(addr) {
            self.ram.read16(addr)
        } else {
            Err(Trap::LoadAccessFault(addr))
        }
    }

    fn read32(&self, addr: Addr) -> Result<u32, Trap> {
        if Self::in_dram(addr) {
            self.ram.read32(addr)
        } else {
            Err(Trap::LoadAccessFault(addr))
        }
    }

    fn read64(&self, addr: Addr) -> Result<u64, Trap> {
        if Self::in_dram(addr) {
            self.ram.read64(addr)
        } else {
            Err(Trap::LoadAccessFault(addr))
        }
    }

    fn write8(&mut self, addr: Addr, val: u8) -> Result<(), Trap> {
        if Self::in_dram(addr) {
            self.ram.write8(addr, val)
        } else {
            Err(Trap::StoreAccessFault(addr))
        }
    }

    fn write16(&mut self, addr: Addr, val: u16) -> Result<(), Trap> {
        if Self::in_dram(addr) {
            self.ram.write16(addr, val)
        } else {
            Err(Trap::StoreAccessFault(addr))
        }
    }

    fn write32(&mut self, addr: Addr, val: u32) -> Result<(), Trap> {
        if Self::in_dram(addr) {
            self.ram.write32(addr, val)
        } else {
            Err(Trap::StoreAccessFault(addr))
        }
    }

    fn write64(&mut self, addr: Addr, val: u64) -> Result<(), Trap> {
        if Self::in_dram(addr) {
            self.ram.write64(addr, val)
        } else {
            Err(Trap::StoreAccessFault(addr))
        }
    }
}
