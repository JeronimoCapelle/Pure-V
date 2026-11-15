//! `#![no_std]` ELF64 loader using `goblin`.

use goblin::elf::{
    Elf,
    header::{EI_CLASS, EI_DATA, ELFCLASS64, ELFDATA2LSB, EM_RISCV},
    program_header::PT_LOAD,
};
use purev_isa::{Addr, Trap};

use crate::bus::Memory;

/// Errors returned while validating or loading an ELF64 guest binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfError {
    /// Input byte slice is truncated or malformed.
    InvalidHeader,
    /// File is not a 64-bit little-endian RISC-V ELF.
    UnsupportedArchitecture,
    /// Segment file offset/size exceeds input slice bounds.
    TruncatedSegment,
    /// Bus fault while copying a `PT_LOAD` segment into guest memory.
    MemoryFault(Trap),
}

impl From<Trap> for ElfError {
    fn from(trap: Trap) -> Self {
        Self::MemoryFault(trap)
    }
}

/// Loads `PT_LOAD` segments from a 64-bit little-endian RISC-V ELF into `mem`
/// and returns the ELF entry point address (`e_entry`).
pub fn load_elf(bytes: &[u8], mem: &mut impl Memory) -> Result<Addr, ElfError> {
    let elf = Elf::parse(bytes).map_err(|_| ElfError::InvalidHeader)?;

    if !elf.is_64
        || !elf.little_endian
        || elf.header.e_ident[EI_CLASS] != ELFCLASS64
        || elf.header.e_ident[EI_DATA] != ELFDATA2LSB
        || elf.header.e_machine != EM_RISCV
    {
        return Err(ElfError::UnsupportedArchitecture);
    }

    for ph in &elf.program_headers {
        if ph.p_type != PT_LOAD {
            continue;
        }

        let file_off = usize::try_from(ph.p_offset).map_err(|_| ElfError::TruncatedSegment)?;
        let file_sz = usize::try_from(ph.p_filesz).map_err(|_| ElfError::TruncatedSegment)?;
        let file_end = file_off
            .checked_add(file_sz)
            .ok_or(ElfError::TruncatedSegment)?;
        let segment = bytes
            .get(file_off..file_end)
            .ok_or(ElfError::TruncatedSegment)?;

        let paddr = ph.p_paddr;
        for (offset, &byte) in segment.iter().enumerate() {
            let byte_offset = u64::try_from(offset).map_err(|_| ElfError::TruncatedSegment)?;
            mem.write8(paddr.wrapping_add(byte_offset), byte)?;
        }

        // Zero-fill `.bss` tail (`p_memsz > p_filesz`).
        for bss_offset in ph.p_filesz..ph.p_memsz {
            mem.write8(paddr.wrapping_add(bss_offset), 0)?;
        }
    }

    Ok(elf.entry)
}
