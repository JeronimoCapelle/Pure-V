//! Physical guest DRAM (`0x8000_0000 .. 0x8800_0000`, 128 MiB).

use alloc::{vec, vec::Vec};

use purev_isa::{Addr, DRAM_BASE, DRAM_SIZE, Trap};

use super::Memory;

/// Flat little-endian physical DRAM backing `[DRAM_BASE, DRAM_BASE +
/// DRAM_SIZE)`.
#[derive(Debug, Clone)]
pub struct FlatRam {
    base: Addr,
    data: Vec<u8>,
}

impl Default for FlatRam {
    fn default() -> Self {
        Self::new()
    }
}

impl FlatRam {
    /// Allocates zero-initialized guest DRAM (`128 MiB`) at [`DRAM_BASE`].
    pub fn new() -> Self {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "DRAM_SIZE (128 MiB) fits comfortably in host usize on 32/64-bit targets"
        )]
        let size = DRAM_SIZE as usize;
        Self {
            base: DRAM_BASE,
            data: vec![0; size],
        }
    }

    #[inline]
    fn offset(&self, addr: Addr, size: usize) -> Option<usize> {
        let rel = addr.checked_sub(self.base)?;
        let off = usize::try_from(rel).ok()?;
        let end = off.checked_add(size)?;
        if end <= self.data.len() {
            Some(off)
        } else {
            None
        }
    }

    #[inline]
    fn read_slice(&self, addr: Addr, size: usize) -> Result<&[u8], Trap> {
        let off = self.offset(addr, size).ok_or(Trap::LoadAccessFault(addr))?;
        self.data
            .get(off..off + size)
            .ok_or(Trap::LoadAccessFault(addr))
    }

    #[inline]
    fn write_slice(&mut self, addr: Addr, bytes: &[u8]) -> Result<(), Trap> {
        let off = self
            .offset(addr, bytes.len())
            .ok_or(Trap::StoreAccessFault(addr))?;
        let dst = self
            .data
            .get_mut(off..off + bytes.len())
            .ok_or(Trap::StoreAccessFault(addr))?;
        dst.copy_from_slice(bytes);
        Ok(())
    }
}

impl Memory for FlatRam {
    fn read8(&self, addr: Addr) -> Result<u8, Trap> {
        let slice = self.read_slice(addr, 1)?;
        Ok(slice[0])
    }

    fn read16(&self, addr: Addr) -> Result<u16, Trap> {
        let slice = self.read_slice(addr, 2)?;
        let bytes: [u8; 2] = slice.try_into().map_err(|_| Trap::LoadAccessFault(addr))?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn read32(&self, addr: Addr) -> Result<u32, Trap> {
        let slice = self.read_slice(addr, 4)?;
        let bytes: [u8; 4] = slice.try_into().map_err(|_| Trap::LoadAccessFault(addr))?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn read64(&self, addr: Addr) -> Result<u64, Trap> {
        let slice = self.read_slice(addr, 8)?;
        let bytes: [u8; 8] = slice.try_into().map_err(|_| Trap::LoadAccessFault(addr))?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn write8(&mut self, addr: Addr, val: u8) -> Result<(), Trap> {
        self.write_slice(addr, &[val])
    }

    fn write16(&mut self, addr: Addr, val: u16) -> Result<(), Trap> {
        self.write_slice(addr, &val.to_le_bytes())
    }

    fn write32(&mut self, addr: Addr, val: u32) -> Result<(), Trap> {
        self.write_slice(addr, &val.to_le_bytes())
    }

    fn write64(&mut self, addr: Addr, val: u64) -> Result<(), Trap> {
        self.write_slice(addr, &val.to_le_bytes())
    }
}
