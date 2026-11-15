//! Top-level iterative machine execution loop, datapath execution, and trap
//! handling.

use purev_isa::{Addr, Instruction, IsaConfig, RegValue, Register, Trap};

use crate::{
    bus::{Memory, SystemBus, Width},
    cpu::{CpuCore, alu, branch, decode::decode},
    elf::{ElfError, load_elf},
    journal::{Journal, UndoAction},
};

/// Top-level RV64IMAC virtual machine owning CPU state, undo journal, and
/// system bus.
#[derive(Debug, Clone)]
pub struct Machine {
    /// Architectural CPU core state.
    pub cpu: CpuCore,
    /// System bus and physical memory.
    pub bus: SystemBus,
    /// 8-slot undo journal for atomic step rollback.
    pub journal: Journal,
}

impl Default for Machine {
    fn default() -> Self {
        Self::new(IsaConfig::RV64IMAC)
    }
}

impl Machine {
    /// Creates a new [`Machine`] configured with `isa`.
    pub fn new(isa: IsaConfig) -> Self {
        Self {
            cpu: CpuCore::new(purev_isa::DRAM_BASE, isa),
            bus: SystemBus::new(),
            journal: Journal::new(),
        }
    }

    /// Loads an ELF64 RISC-V binary into physical memory and sets `pc` to its
    /// entry point.
    pub fn load_elf(&mut self, bytes: &[u8]) -> Result<Addr, ElfError> {
        let entry = load_elf(bytes, &mut self.bus)?;
        self.cpu.pc = entry;
        Ok(entry)
    }

    /// Executes a single `Fetch -> Decode -> Execute` cycle atomically.
    pub fn step(&mut self) -> Result<(), Trap> {
        let pc = self.cpu.pc;
        let align_mask = if self.cpu.isa.c { 0b01 } else { 0b11 };
        if pc & align_mask != 0 {
            return Err(Trap::InstructionAddressMisaligned(pc));
        }

        let raw_bits = self
            .bus
            .read32(pc)
            .map_err(|_| Trap::InstructionAccessFault(pc))?;
        let (inst, inst_len) = decode(raw_bits, self.cpu.isa)?;
        let mut next_pc = pc.wrapping_add(inst_len);

        match execute(
            inst,
            &mut self.cpu,
            &mut self.bus,
            &mut self.journal,
            &mut next_pc,
        ) {
            Ok(()) => {
                self.cpu.pc = next_pc;
                self.journal.commit();
                Ok(())
            },
            Err(trap) => {
                self.journal.rollback(&mut self.cpu.regs, &mut self.bus);
                Err(trap)
            },
        }
    }

    /// Runs the flat iterative execution loop until a trap stops execution.
    pub fn run(&mut self) -> Result<(), Trap> {
        loop {
            if let Err(trap) = self.step() {
                TrapHandler::handle(trap, &self.cpu)?;
            }
        }
    }
}

/// Trap handler invoked after [`Journal::rollback`] on `Err(Trap)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TrapHandler;

impl TrapHandler {
    /// Handles a synchronous or asynchronous trap (v0.1: propagates the trap to
    /// halt `run`).
    pub const fn handle(trap: Trap, cpu: &CpuCore) -> Result<(), Trap> {
        let _ = cpu;
        Err(trap)
    }
}

#[inline]
fn write_reg_journaled(cpu: &mut CpuCore, journal: &mut Journal, rd: Register, val: RegValue) {
    if rd != Register::ZERO {
        let old_val = cpu.regs.read(rd);
        let _ = journal.record(UndoAction::Register { reg: rd, old_val });
        cpu.regs.write(rd, val);
    }
}

#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "RISC-V sub-doubleword stores explicitly mask to the lower 32 bits"
)]
fn store_journaled(
    bus: &mut SystemBus,
    journal: &mut Journal,
    addr: Addr,
    width: Width,
    val: RegValue,
) -> Result<(), Trap> {
    if addr & width.alignment_mask() != 0 {
        return Err(Trap::StoreAddressMisaligned(addr));
    }

    match width {
        Width::Byte => {
            let old_val = bus.read8(addr)?;
            let _ = journal.record(UndoAction::Memory8 { addr, old_val });
            bus.write8(addr, val as u8)
        },
        Width::HalfWord => {
            let old_val = bus.read16(addr)?;
            let _ = journal.record(UndoAction::Memory16 { addr, old_val });
            bus.write16(addr, val as u16)
        },
        Width::Word => {
            let old_val = bus.read32(addr)?;
            let _ = journal.record(UndoAction::Memory32 { addr, old_val });
            bus.write32(addr, val as u32)
        },
        Width::DoubleWord => {
            let old_val = bus.read64(addr)?;
            let _ = journal.record(UndoAction::Memory64 { addr, old_val });
            bus.write64(addr, val)
        },
    }
}

fn execute(
    inst: Instruction,
    cpu: &mut CpuCore,
    bus: &mut SystemBus,
    journal: &mut Journal,
    next_pc: &mut Addr,
) -> Result<(), Trap> {
    match inst {
        Instruction::Lui { rd, imm } => {
            write_reg_journaled(cpu, journal, rd, imm.cast_unsigned());
        },
        Instruction::Auipc { rd, imm } => {
            let res = alu::addi64(cpu.pc, imm);
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Jal { rd, imm } => {
            let link = *next_pc;
            *next_pc = alu::addi64(cpu.pc, imm);
            write_reg_journaled(cpu, journal, rd, link);
        },
        Instruction::Jalr { rd, rs1, imm } => {
            let link = *next_pc;
            let target = alu::addi64(cpu.regs.read(rs1), imm) & !1;
            *next_pc = target;
            write_reg_journaled(cpu, journal, rd, link);
        },
        Instruction::Beq { rs1, rs2, imm } => {
            if branch::eq(cpu.regs.read(rs1), cpu.regs.read(rs2)) {
                *next_pc = alu::addi64(cpu.pc, imm);
            }
        },
        Instruction::Bne { rs1, rs2, imm } => {
            if branch::ne(cpu.regs.read(rs1), cpu.regs.read(rs2)) {
                *next_pc = alu::addi64(cpu.pc, imm);
            }
        },
        Instruction::Blt { rs1, rs2, imm } => {
            if branch::lt(cpu.regs.read(rs1), cpu.regs.read(rs2)) {
                *next_pc = alu::addi64(cpu.pc, imm);
            }
        },
        Instruction::Bge { rs1, rs2, imm } => {
            if branch::ge(cpu.regs.read(rs1), cpu.regs.read(rs2)) {
                *next_pc = alu::addi64(cpu.pc, imm);
            }
        },
        Instruction::Bltu { rs1, rs2, imm } => {
            if branch::ltu(cpu.regs.read(rs1), cpu.regs.read(rs2)) {
                *next_pc = alu::addi64(cpu.pc, imm);
            }
        },
        Instruction::Bgeu { rs1, rs2, imm } => {
            if branch::geu(cpu.regs.read(rs1), cpu.regs.read(rs2)) {
                *next_pc = alu::addi64(cpu.pc, imm);
            }
        },
        Instruction::Lb { rd, rs1, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            let val = bus.load(addr, Width::Byte, true)?;
            write_reg_journaled(cpu, journal, rd, val);
        },
        Instruction::Lh { rd, rs1, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            let val = bus.load(addr, Width::HalfWord, true)?;
            write_reg_journaled(cpu, journal, rd, val);
        },
        Instruction::Lw { rd, rs1, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            let val = bus.load(addr, Width::Word, true)?;
            write_reg_journaled(cpu, journal, rd, val);
        },
        Instruction::Ld { rd, rs1, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            let val = bus.load(addr, Width::DoubleWord, false)?;
            write_reg_journaled(cpu, journal, rd, val);
        },
        Instruction::Lbu { rd, rs1, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            let val = bus.load(addr, Width::Byte, false)?;
            write_reg_journaled(cpu, journal, rd, val);
        },
        Instruction::Lhu { rd, rs1, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            let val = bus.load(addr, Width::HalfWord, false)?;
            write_reg_journaled(cpu, journal, rd, val);
        },
        Instruction::Lwu { rd, rs1, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            let val = bus.load(addr, Width::Word, false)?;
            write_reg_journaled(cpu, journal, rd, val);
        },
        Instruction::Sb { rs1, rs2, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            store_journaled(bus, journal, addr, Width::Byte, cpu.regs.read(rs2))?;
        },
        Instruction::Sh { rs1, rs2, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            store_journaled(bus, journal, addr, Width::HalfWord, cpu.regs.read(rs2))?;
        },
        Instruction::Sw { rs1, rs2, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            store_journaled(bus, journal, addr, Width::Word, cpu.regs.read(rs2))?;
        },
        Instruction::Sd { rs1, rs2, imm } => {
            let addr = alu::addi64(cpu.regs.read(rs1), imm);
            store_journaled(bus, journal, addr, Width::DoubleWord, cpu.regs.read(rs2))?;
        },
        Instruction::Addi { rd, rs1, imm } => {
            let res = alu::addi64(cpu.regs.read(rs1), imm);
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Slti { rd, rs1, imm } => {
            let res = alu::slt64(cpu.regs.read(rs1), imm.cast_unsigned());
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Sltiu { rd, rs1, imm } => {
            let res = alu::sltu64(cpu.regs.read(rs1), imm.cast_unsigned());
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Xori { rd, rs1, imm } => {
            let res = cpu.regs.read(rs1) ^ imm.cast_unsigned();
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Ori { rd, rs1, imm } => {
            let res = cpu.regs.read(rs1) | imm.cast_unsigned();
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Andi { rd, rs1, imm } => {
            let res = cpu.regs.read(rs1) & imm.cast_unsigned();
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Slli { rd, rs1, shamt } => {
            let res = alu::sll64(cpu.regs.read(rs1), u64::from(shamt));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Srli { rd, rs1, shamt } => {
            let res = alu::srl64(cpu.regs.read(rs1), u64::from(shamt));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Srai { rd, rs1, shamt } => {
            let res = alu::sra64(cpu.regs.read(rs1), u64::from(shamt));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Addiw { rd, rs1, imm } => {
            let res = alu::addw(cpu.regs.read(rs1), imm.cast_unsigned());
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Slliw { rd, rs1, shamt } => {
            let res = alu::sllw(cpu.regs.read(rs1), u64::from(shamt));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Srliw { rd, rs1, shamt } => {
            let res = alu::srlw(cpu.regs.read(rs1), u64::from(shamt));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Sraiw { rd, rs1, shamt } => {
            let res = alu::sraw(cpu.regs.read(rs1), u64::from(shamt));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Add { rd, rs1, rs2 } => {
            let res = alu::add64(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Sub { rd, rs1, rs2 } => {
            let res = alu::sub64(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Sll { rd, rs1, rs2 } => {
            let res = alu::sll64(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Slt { rd, rs1, rs2 } => {
            let res = alu::slt64(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Sltu { rd, rs1, rs2 } => {
            let res = alu::sltu64(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Xor { rd, rs1, rs2 } => {
            let res = cpu.regs.read(rs1) ^ cpu.regs.read(rs2);
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Srl { rd, rs1, rs2 } => {
            let res = alu::srl64(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Sra { rd, rs1, rs2 } => {
            let res = alu::sra64(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Or { rd, rs1, rs2 } => {
            let res = cpu.regs.read(rs1) | cpu.regs.read(rs2);
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::And { rd, rs1, rs2 } => {
            let res = cpu.regs.read(rs1) & cpu.regs.read(rs2);
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Addw { rd, rs1, rs2 } => {
            let res = alu::addw(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Subw { rd, rs1, rs2 } => {
            let res = alu::subw(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Sllw { rd, rs1, rs2 } => {
            let res = alu::sllw(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Srlw { rd, rs1, rs2 } => {
            let res = alu::srlw(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Sraw { rd, rs1, rs2 } => {
            let res = alu::sraw(cpu.regs.read(rs1), cpu.regs.read(rs2));
            write_reg_journaled(cpu, journal, rd, res);
        },
        Instruction::Fence { .. } => {},
        Instruction::Ecall => return Err(Trap::EnvironmentCallFromMMode),
        Instruction::Ebreak => return Err(Trap::Breakpoint),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use purev_isa::{DRAM_BASE, IsaConfig, Register, Trap};

    use super::*;

    #[test]
    fn x0_writes_are_no_ops() {
        let mut machine = Machine::new(IsaConfig::RV64I);
        machine.cpu.regs.write(Register::ZERO, 0xDEAD_BEEF);
        assert_eq!(machine.cpu.regs.read(Register::ZERO), 0);
    }

    #[test]
    fn memory_enforces_alignment_traps() {
        let mut bus = SystemBus::new();
        assert_eq!(
            bus.load(DRAM_BASE + 1, Width::HalfWord, false),
            Err(Trap::LoadAddressMisaligned(DRAM_BASE + 1))
        );
        assert_eq!(
            bus.load(DRAM_BASE + 2, Width::Word, true),
            Err(Trap::LoadAddressMisaligned(DRAM_BASE + 2))
        );
        assert_eq!(
            bus.store(DRAM_BASE + 4, Width::DoubleWord, 42),
            Err(Trap::StoreAddressMisaligned(DRAM_BASE + 4))
        );
    }

    #[test]
    fn journal_rollback_restores_state_in_lifo_order() {
        let mut machine = Machine::new(IsaConfig::RV64I);
        let addr = DRAM_BASE + 0x100;
        machine.cpu.regs.write(Register::A0, 10);
        machine.bus.write64(addr, 0x1122_3344_5566_7788).unwrap();

        write_reg_journaled(&mut machine.cpu, &mut machine.journal, Register::A0, 99);
        store_journaled(
            &mut machine.bus,
            &mut machine.journal,
            addr,
            Width::DoubleWord,
            0,
        )
        .unwrap();
        assert_eq!(machine.cpu.regs.read(Register::A0), 99);
        assert_eq!(machine.bus.read64(addr).unwrap(), 0);

        machine
            .journal
            .rollback(&mut machine.cpu.regs, &mut machine.bus);
        assert!(machine.journal.is_empty());
        assert_eq!(machine.cpu.regs.read(Register::A0), 10);
        assert_eq!(machine.bus.read64(addr).unwrap(), 0x1122_3344_5566_7788);
    }

    #[test]
    fn executes_rv64i_loop_and_sign_extends_word_ops() {
        let mut machine = Machine::new(IsaConfig::RV64I);
        // addi x10, x0, -1     -> 0xfff00513
        // addiw x11, x10, 0    -> 0x0005059b (0xffff_ffff_ffff_ffff)
        // addiw x12, x0, 3     -> 0x0030061b
        // addw x10, x11, x12   -> 0x00c5853b (-1 + 3 = 2)
        // ecall                -> 0x00000073
        let program: [u32; 5] = [
            0xFFF0_0513,
            0x0005_059B,
            0x0030_061B,
            0x00C5_853B,
            0x0000_0073,
        ];
        for (idx, word) in program.into_iter().enumerate() {
            let offset = u64::try_from(idx * 4).unwrap();
            machine.bus.write32(DRAM_BASE + offset, word).unwrap();
        }

        let trap = machine.run().unwrap_err();
        assert_eq!(trap, Trap::EnvironmentCallFromMMode);
        assert_eq!(machine.cpu.regs.read(Register(11)), u64::MAX);
        assert_eq!(machine.cpu.regs.read(Register::A0), 2);
    }
}
