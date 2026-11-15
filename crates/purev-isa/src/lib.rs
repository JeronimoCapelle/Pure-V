//! Shared RV64IMAC architectural types, constants, and trap definitions.

#![no_std]

pub mod instruction;

pub use instruction::Instruction;

/// Physical or virtual memory address width (RV64).
pub type Addr = u64;

/// General-purpose register width (RV64).
pub type RegValue = u64;

/// Base physical address of guest DRAM.
pub const DRAM_BASE: Addr = 0x8000_0000;

/// Total guest DRAM size (128 MiB).
pub const DRAM_SIZE: u64 = 128 * 1024 * 1024;

/// Initial guest stack pointer at the top of DRAM.
pub const INITIAL_SP: Addr = DRAM_BASE + DRAM_SIZE;

/// Interrupt bit (`XLEN - 1`) in `mcause` / `scause`.
pub const INTERRUPT_BIT: RegValue = 1 << 63;

/// General-purpose RISC-V integer register index (`0..32`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register(pub u8);

impl Register {
    /// First argument / return value register (`x10`).
    pub const A0: Self = Self(10);
    /// System call / ecall number register (`x17`).
    pub const A7: Self = Self(17);
    /// Global pointer register (`x3`).
    pub const GP: Self = Self(3);
    /// Return address register (`x1`).
    pub const RA: Self = Self(1);
    /// Stack pointer register (`x2`).
    pub const SP: Self = Self(2);
    /// Thread pointer register (`x4`).
    pub const TP: Self = Self(4);
    /// Hardwired zero register (`x0`).
    pub const ZERO: Self = Self(0);

    /// Constructs a [`Register`] from a 5-bit register field (`0..32`).
    #[inline]
    pub const fn new(index: u8) -> Option<Self> {
        if index < 32 { Some(Self(index)) } else { None }
    }

    /// Parses a register name (`x0`..`x31` or RISC-V ABI alias such as `sp`,
    /// `ra`, `a0`, `fp`).
    pub fn try_from_str(name: &str) -> Option<Self> {
        if let Some(suffix) = name.strip_prefix('x')
            && let Ok(num) = suffix.parse::<u8>()
        {
            return Self::new(num);
        }

        let num = match name {
            "zero" => 0,
            "ra" => 1,
            "sp" => 2,
            "gp" => 3,
            "tp" => 4,
            "t0" => 5,
            "t1" => 6,
            "t2" => 7,
            "s0" | "fp" => 8,
            "s1" => 9,
            "a0" => 10,
            "a1" => 11,
            "a2" => 12,
            "a3" => 13,
            "a4" => 14,
            "a5" => 15,
            "a6" => 16,
            "a7" => 17,
            "s2" => 18,
            "s3" => 19,
            "s4" => 20,
            "s5" => 21,
            "s6" => 22,
            "s7" => 23,
            "s8" => 24,
            "s9" => 25,
            "s10" => 26,
            "s11" => 27,
            "t3" => 28,
            "t4" => 29,
            "t5" => 30,
            "t6" => 31,
            _ => return None,
        };

        Some(Self(num))
    }

    /// Returns the raw register index masked to 5 bits (`0..32`).
    #[inline]
    pub const fn index(self) -> usize {
        (self.0 & 0x1F) as usize
    }
}

/// Runtime toggle configuration for optional RISC-V ISA extensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsaConfig {
    /// Multiply/Divide extension (`M`, including `*W` variants).
    pub m: bool,
    /// Atomic memory operations (`A`: `lr.w`/`sc.w`, `lr.d`/`sc.d`, `amo*`).
    pub a: bool,
    /// Compressed 16-bit instructions (`C`).
    pub c: bool,
    /// Custom-0 thesis extensions.
    pub custom: bool,
}

impl IsaConfig {
    /// Base 64-bit integer ISA (`RV64I`) with no extensions enabled.
    pub const RV64I: Self = Self {
        m: false,
        a: false,
        c: false,
        custom: false,
    };
    /// Full `RV64IMAC` target configuration (Milestone 4+ / `xv6-riscv`).
    pub const RV64IMAC: Self = Self {
        m: true,
        a: true,
        c: true,
        custom: false,
    };
}

impl Default for IsaConfig {
    fn default() -> Self {
        Self::RV64IMAC
    }
}

/// Synchronous exceptions and asynchronous interrupts defined by RISC-V.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trap {
    /// Cause 0: Fetch target address is misaligned.
    InstructionAddressMisaligned(Addr),
    /// Cause 1: Bus or PMP/PMA fault during instruction fetch.
    InstructionAccessFault(Addr),
    /// Cause 2: Illegal or unsupported instruction encoding (32-bit raw bits).
    IllegalInstruction(u32),
    /// Cause 3: `ebreak` / `c.ebreak` breakpoint exception.
    Breakpoint,
    /// Cause 4: Load address is not naturally aligned.
    LoadAddressMisaligned(Addr),
    /// Cause 5: Bus or PMP/PMA fault on load.
    LoadAccessFault(Addr),
    /// Cause 6: Store or AMO address is not naturally aligned.
    StoreAddressMisaligned(Addr),
    /// Cause 7: Bus or PMP/PMA fault on store or AMO.
    StoreAccessFault(Addr),
    /// Cause 8: `ecall` executed in User mode.
    EnvironmentCallFromUMode,
    /// Cause 9: `ecall` executed in Supervisor mode.
    EnvironmentCallFromSMode,
    /// Cause 11: `ecall` executed in Machine mode.
    EnvironmentCallFromMMode,
    /// Cause 12: Virtual-memory page fault on instruction fetch.
    InstructionPageFault(Addr),
    /// Cause 13: Virtual-memory page fault on load.
    LoadPageFault(Addr),
    /// Cause 15: Virtual-memory page fault on store or AMO (cause 14 is
    /// reserved).
    StorePageFault(Addr),
    /// Interrupt + 1: Supervisor software interrupt.
    SupervisorSoftwareInterrupt,
    /// Interrupt + 3: Machine software interrupt.
    MachineSoftwareInterrupt,
    /// Interrupt + 5: Supervisor timer interrupt.
    SupervisorTimerInterrupt,
    /// Interrupt + 7: Machine timer interrupt.
    MachineTimerInterrupt,
    /// Interrupt + 9: Supervisor external interrupt.
    SupervisorExternalInterrupt,
    /// Interrupt + 11: Machine external interrupt.
    MachineExternalInterrupt,
}

impl Trap {
    /// Returns the architectural `mcause` / `scause` value for this trap.
    pub const fn cause(self) -> RegValue {
        match self {
            Self::InstructionAddressMisaligned(_) => 0,
            Self::InstructionAccessFault(_) => 1,
            Self::IllegalInstruction(_) => 2,
            Self::Breakpoint => 3,
            Self::LoadAddressMisaligned(_) => 4,
            Self::LoadAccessFault(_) => 5,
            Self::StoreAddressMisaligned(_) => 6,
            Self::StoreAccessFault(_) => 7,
            Self::EnvironmentCallFromUMode => 8,
            Self::EnvironmentCallFromSMode => 9,
            Self::EnvironmentCallFromMMode => 11,
            Self::InstructionPageFault(_) => 12,
            Self::LoadPageFault(_) => 13,
            Self::StorePageFault(_) => 15,
            Self::SupervisorSoftwareInterrupt => INTERRUPT_BIT | 0x01,
            Self::MachineSoftwareInterrupt => INTERRUPT_BIT | 0x03,
            Self::SupervisorTimerInterrupt => INTERRUPT_BIT | 0x05,
            Self::MachineTimerInterrupt => INTERRUPT_BIT | 0x07,
            Self::SupervisorExternalInterrupt => INTERRUPT_BIT | 0x09,
            Self::MachineExternalInterrupt => INTERRUPT_BIT | 0x0B,
        }
    }

    /// Returns `true` if this trap represents an asynchronous interrupt.
    pub const fn is_interrupt(self) -> bool {
        matches!(
            self,
            Self::SupervisorSoftwareInterrupt
                | Self::MachineSoftwareInterrupt
                | Self::SupervisorTimerInterrupt
                | Self::MachineTimerInterrupt
                | Self::SupervisorExternalInterrupt
                | Self::MachineExternalInterrupt
        )
    }
}
