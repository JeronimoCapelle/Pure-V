//! Integration tests for the `#![no_std]` ELF64 loader (`purev_core::elf`).

use purev_asm::assemble_bytes;
use purev_core::{Machine, bus::Memory, elf::ElfError};
use purev_isa::{DRAM_BASE, IsaConfig, Register, Trap};

#[allow(clippy::unwrap_used, reason = "integration test helper")]
fn build_elf64_riscv(entry: u64, paddr: u64, code: &[u8], bss_extra: u64) -> Vec<u8> {
    let mut elf = Vec::new();
    // ELF64 header (64 bytes)
    elf.extend_from_slice(&[0x7F, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    elf.extend_from_slice(&2u16.to_le_bytes()); // e_type = ET_EXEC
    elf.extend_from_slice(&243u16.to_le_bytes()); // e_machine = EM_RISCV
    elf.extend_from_slice(&1u32.to_le_bytes()); // e_version
    elf.extend_from_slice(&entry.to_le_bytes()); // e_entry
    elf.extend_from_slice(&64u64.to_le_bytes()); // e_phoff
    elf.extend_from_slice(&0u64.to_le_bytes()); // e_shoff
    elf.extend_from_slice(&0u32.to_le_bytes()); // e_flags
    elf.extend_from_slice(&64u16.to_le_bytes()); // e_ehsize
    elf.extend_from_slice(&56u16.to_le_bytes()); // e_phentsize
    elf.extend_from_slice(&1u16.to_le_bytes()); // e_phnum
    elf.extend_from_slice(&64u16.to_le_bytes()); // e_shentsize
    elf.extend_from_slice(&0u16.to_le_bytes()); // e_shnum
    elf.extend_from_slice(&0u16.to_le_bytes()); // e_shstrndx

    let filesz = u64::try_from(code.len()).unwrap();
    let memsz = filesz + bss_extra;

    // ProgramHeader (56 bytes)
    elf.extend_from_slice(&1u32.to_le_bytes()); // p_type = PT_LOAD
    elf.extend_from_slice(&7u32.to_le_bytes()); // p_flags = RWX
    elf.extend_from_slice(&120u64.to_le_bytes()); // p_offset = 64 + 56
    elf.extend_from_slice(&paddr.to_le_bytes()); // p_vaddr
    elf.extend_from_slice(&paddr.to_le_bytes()); // p_paddr
    elf.extend_from_slice(&filesz.to_le_bytes()); // p_filesz
    elf.extend_from_slice(&memsz.to_le_bytes()); // p_memsz
    elf.extend_from_slice(&0x1000u64.to_le_bytes()); // p_align

    elf.extend_from_slice(code);
    elf
}

#[test]
fn load_elf_maps_segments_zero_fills_bss_and_runs_to_ecall() {
    let code = assemble_bytes(
        "
        _start:
            addi a0, zero, 0
            addi t0, zero, 1
            addi t1, zero, 5
        loop:
            beq t0, t1, done
            add a0, a0, t0
            addi t0, t0, 1
            jal zero, loop
        done:
            ecall
        ",
    )
    .unwrap();

    let bss_addr = DRAM_BASE + u64::try_from(code.len()).unwrap();
    let elf = build_elf64_riscv(DRAM_BASE, DRAM_BASE, &code, 16);

    let mut machine = Machine::new(IsaConfig::RV64I);
    // Pre-dirty the .bss region in RAM to verify load_elf zero-fills it.
    machine
        .bus
        .write64(bss_addr, 0xDEAD_BEEF_CAFE_BABE)
        .unwrap();
    machine
        .bus
        .write64(bss_addr + 8, 0x1234_5678_9ABC_DEF0)
        .unwrap();

    let entry = machine.load_elf(&elf).unwrap();
    assert_eq!(entry, DRAM_BASE);
    assert_eq!(machine.bus.read64(bss_addr).unwrap(), 0);
    assert_eq!(machine.bus.read64(bss_addr + 8).unwrap(), 0);

    let trap = machine.run().unwrap_err();
    assert_eq!(trap, Trap::EnvironmentCallFromMMode);
    assert_eq!(machine.cpu.regs.read(Register::A0), 1 + 2 + 3 + 4);
}

#[test]
fn load_elf_rejects_non_riscv_elf() {
    let mut elf = build_elf64_riscv(DRAM_BASE, DRAM_BASE, &[0x73, 0x00, 0x00, 0x00], 0);
    // Overwrite e_machine with x86_64 (62)
    elf[18..20].copy_from_slice(&62u16.to_le_bytes());

    let mut machine = Machine::new(IsaConfig::RV64I);
    assert_eq!(
        machine.load_elf(&elf),
        Err(ElfError::UnsupportedArchitecture)
    );
}
