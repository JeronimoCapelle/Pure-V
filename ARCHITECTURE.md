# `purev` — RV64IMAC Emulator Architecture, Contracts & Milestone Reference

## 1. Project Target & Strategic Decisions

### Why `RV64IMAC` (64-Bit) From Day 1
* **Eliminates a 32-to-64-Bit Rewrite:** Locking in `Addr = u64` and `RegValue = u64` on Day 1 avoids refactoring register widths, 64-bit load/store alignment (`ld`/`sd`), 32-bit sign-extending `*W` instructions, unified 64-bit CSRs, and `Sv39` virtual memory page tables later.
* **Why `RV64IMAC` Instead of `RV64GC`:**
  * The `G` extension bundle includes single- (`F`) and double-precision (`D`) hardware floating point, which requires 32 separate FP registers, NaN-boxing of 32-bit floats inside 64-bit registers, dynamic rounding modes, and IEEE-754 sticky exception flags (`fcsr`).
  * Targeting **`RV64IMAC`** avoids FP complexity while still allowing you to compile native `no_std` Rust binaries, pass the official `rv64ui`, `rv64um`, `rv64ua`, and `rv64uc` compliance suites, and boot **MIT `xv6-riscv`**—a complete multi-process Unix-like operating system with `Sv39` virtual memory, pipes, and a filesystem.
* **Boot Target Distinction (`xv6-riscv` vs. Linux):**
  * **`xv6-riscv` (Milestone 6 Target):** Boots directly in Machine mode at `0x8000_0000` (no OpenSBI firmware required) and runs cleanly on `RV64IMAC` (compiled with `-march=rv64imac -mabi=lp64`).
  * **Mainstream Linux (`vmlinux` + BusyBox):** Requires OpenSBI firmware **and** `RV64GC` (`F`/`D` floating-point extensions) for standard user-space rootfs binaries. Keep Linux/`F`/`D` out of `v1.0.0` scope.

### Workspace Topology (`#![no_std]` Core + Swappable Frontends)
Split the repository into a Cargo workspace so the exact same emulator engine compiles unchanged for both desktop testing (`std`) and bare-metal `x86_64` execution (`#![no_std]`):

```text
purev/
├── Cargo.toml                  # Workspace definition (resolver = "2")
└── crates/
    ├── purev-isa/              # #![no_std] Shared types: Addr, RegValue, Register, Instruction, IsaConfig, Trap
    ├── purev-asm/              # #![no_std] + alloc: Lexer -> Vec<Token> -> Parser -> Encoder
    ├── purev-core/             # #![no_std] + alloc: Machine, CpuCore, Journal, Decode, Execute, SystemBus, ELF
    ├── purev-cli/              # std: Desktop runner, riscv-tests harness, lockstep fuzzer, criterion benchmarks
    └── purev-limine/           # #![no_std] + #![no_main]: Bare-metal x86_64 unikernel host (Milestone 7)
```

Configure `goblin` in `purev-core/Cargo.toml` for 64-bit `no_std` ELF parsing from raw byte slices (`&[u8]`):

```toml
[dependencies]
purev-isa = { path = "../purev-isa" }
goblin = { version = "0.10", default-features = false, features = ["elf64", "endian_fd"] }
```

---

## 2. Core Architectural Philosophy & Rust Idioms

### Discrete Dataflow & Visibility Boundaries
* **Batch Pipeline Over Streaming (Assembler):** Pass discrete, immutable buffers between stages (`&str -> Vec<Token> -> Vec<Instruction> -> Vec<u32>`) rather than coupling stages with mutable references (`Parser` holding `&mut Lexer`). Every stage is independently unit-testable.
* **Encapsulation via Visibility, Not Traits:** Use traits **only for genuine polymorphism** (`Memory`, `SerialBackend`, `BlockBackend`). Hide internal state using private struct fields, newtypes (`Register(u8)`), and `pub(crate)` visibility.
* **Concrete `RegFile` & Stateless Math Modules:**
  * `RegFile` is a concrete struct (`struct RegFile([RegValue; 32])`).
  * `alu` and `branch` are **stateless modules of pure free functions**—never stateful structs holding `&mut` references to registers or memory, which causes borrow-checker contention.
* **Single `Memory` Trait with Default Helpers:** Declare low-level physical primitives (`read8/16/32/64`, `write8/16/32/64`) as required methods on `Memory`, and implement high-level RISC-V operations (`load` with sign/zero extension, `store` with width masking) as **default methods** on the same trait.

### Runtime `IsaConfig` vs. Cargo `#[cfg]` Flags
* **Rule:** Never gate instruction variants or decoder arms behind Cargo `#[cfg(feature = "...")]` flags. Cargo features are global and additive across a workspace, breaking runtime `--isa` toggles and mixed-ISA test runners.
* **Implementation:** Keep all `Instruction` variants compiled at all times and validate active extensions at runtime inside `decode()` via `IsaConfig`. Reserve `#[cfg]` strictly for `std` vs. `no_std` and target-architecture boundaries.

### Iterative Execution Loop (No Recursive Trap Calls)
* **Rule:** `ControlUnit` must never call `TrapHandler`, and `TrapHandler` must never recursively call `ControlUnit` (which overflows the host stack under frequent syscalls or interrupts).
* **Implementation:** Run a flat iterative loop inside `Machine::run()`:
  1. `Machine::step()` executes `Fetch -> Decode -> Execute` and returns `Result<(), Trap>`.
  2. On `Ok(())`, commit `pc = next_pc` and reset `Journal`.
  3. On `Err(trap)`, rewind `Journal` in reverse order and invoke `TrapHandler` (which panics in `v0.1` or updates CSRs/`pc = mtvec` in `v0.5`), then let the outer loop continue.

---

## 3. System Components & Responsibility Matrix

```text
 ┌───────────────────────────────────────────────────────────────┐
 │                            Machine                            │
 │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────┐  │
 │  │    CpuCore    │  │    Journal    │  │     SystemBus     │  │
 │  │ - regs (u64)  │  │ - 8 undo slots│  │ - FlatRam         │  │
 │  │ - pc   (u64)  │  └───────────────┘  │ - UART / CLINT    │  │
 │  │ - mode        │                     │ - PLIC / VirtIO   │  │
 │  │ - csrs        │                     │ - Reservation     │  │
 │  └───────┬───────┘                     └─────────▲─────────┘  │
 │          │                                       │            │
 │          ▼                                       │            │
 │     ControlUnit ──(fetch / decode / execute)─────┘            │
 └──────────────────────────────┬────────────────────────────────┘
                                │
                          On Err(Trap)
                                ▼
                       ┌─────────────────┐
                       │   TrapHandler   │
                       │ - rollback()    │
                       │ - panic (v0.1)  │
                       │ - trap vec(v0.5)│
                       └─────────────────┘
```

### Stage & Module Responsibilities

| Component / Stage | Responsibility & Invariants |
| :--- | :--- |
| **`Fetch`** | Reads raw 32-bit instruction bits at `self.pc` via `SystemBus` (translating virtual-to-physical addresses when `Sv39` is active). |
| **`Decode` (The Gatekeeper)** | Converts raw bits into `(Instruction, inst_len)`. Validates opcode, `funct3`, `funct7`, shift bit-25 constraints, and `IsaConfig` immediately. Rejects invalid bit patterns or disabled extensions with `Err(Trap::IllegalInstruction(raw_u32))` before unpacking registers. |
| **`Execute` (Datapath Router)** | Pure value-router and orchestrator. Reads `RegValue` (`u64`) from `RegFile`, passes plain integers to `alu` / `branch` / `SystemBus`, writes results back via journaled helpers, and updates `*next_pc` on branches/jumps. Validates **dynamic runtime state** (privilege level for `mret`/`sret`/`sfence.vma`, memory faults). |
| **`RegFile`** | Encapsulates `[u64; 32]`. Enforces hardwired `x0 == 0` invariant internally. |
| **`SystemBus`** | Implements `Memory`. Decodes physical addresses into `FlatRam` or MMIO devices, enforces little-endian conversion and strict alignment, and tracks `lr.w`/`sc.w` & `lr.d`/`sc.d` reservations. |
| **`Journal`** | Fixed 8-slot stack undo buffer (`[Option<UndoAction>; 8]`, zero heap allocations). Records old values before any state mutation so multi-step instructions roll back atomically on mid-instruction faults. |
| **`TrapHandler`** | Receives `Err(Trap)` after `Journal::rollback()`. Panics with diagnostics in `v0.1`; vectors to `mtvec`/`stvec` and updates `mepc`/`sepc`, `mcause`/`scause`, `mtval`/`stval` in `v0.5`. |

### PC Advancement & Undo Journal Mechanics
* **PC Advancement:** Before calling `execute()`, compute `let mut next_pc = self.pc.wrapping_add(inst_len);` (`inst_len` is `4` for base `RV64I` and `2` for compressed `RV64C`). Pass `&mut next_pc` into `execute()`. Taken branches/jumps overwrite `*next_pc = target`; sequential instructions leave it untouched.
* **Undo Journal (History Buffer):** State mutates in place on the hot path (`>99.9%` of cycles), logging the *previous* value into the 8-slot stack `Journal` before writing. On `Ok(())`, `journal.commit()` sets `len = 0` (single integer reset). On `Err(Trap)`, `journal.rollback()` unwinds entries in reverse (LIFO) order.

---

## 4. Non-Negotiable `RV64` Hardware Contracts

1. **Writes to `x0` Must NEVER Trap:**
   * `RegFile::read(Register(0))` always returns `0`.
   * `RegFile::write(Register(0), val)` is a **silent no-op**.
   * *Why:* Core RISC-V pseudo-instructions (`nop` = `addi x0, x0, 0`, `j label` = `jal x0, label`, `ret` = `jalr x0, x1, 0`) discard outputs into `x0`.
2. **Integer Divide-by-Zero & Overflow (`RV64M`) Must NEVER Trap:**
   * `x / 0 = u64::MAX` (`-1`), `x % 0 = x`.
   * Signed 64-bit overflow (`i64::MIN / -1`) returns `i64::MIN`; remainder (`i64::MIN % -1`) returns `0`.
   * 32-bit word variants (`divw`, `divuw`, `remw`, `remuw`) follow the same non-trapping rules on 32-bit operands and sign-extend the 32-bit result to 64 bits.
3. **The `*W` (32-Bit Word) Sign-Extension Invariant:**
   * `RV64I` (`addiw`, `slliw`, `srliw`, `sraiw`, `addw`, `subw`, `sllw`, `srlw`, `sraw`) and `RV64M` (`mulw`, `divw`, `divuw`, `remw`, `remuw`) operate only on the lower 32 bits of source registers and **sign-extend bit 31 across bits `32..=63`**:
     ```rust
     let res_64 = (res_u32 as i32) as i64 as u64;
     ```
4. **Shift Masking & Bit-25 Immediate Check:**
   * 64-bit shifts (`sll`, `srl`, `sra`, `slli`, `srli`, `srai`) mask shift amounts with `& 0x3F` (6 bits, `0..=63`). In `slli`/`srli`/`srai`, `shamt` occupies bits `20..=25` and `funct7` occupies the top 6 bits (`26..=31`).
   * 32-bit word shifts (`sllw`, `srlw`, `sraw`, `slliw`, `srliw`, `sraiw`) mask shift amounts with `& 0x1F` (5 bits, `0..=31`). **If bit `25` is `1` in `slliw`/`srliw`/`sraiw`, `decode()` must reject it with `Trap::IllegalInstruction`.**
5. **32-Bit Loads (`lw` vs. `lwu`):**
   * `lw` loads 32 bits and **sign-extends** to 64 bits (`(val as i32) as i64 as u64`).
   * `lwu` loads 32 bits and **zero-extends** to 64 bits (`val as u64`).
6. **Strict Alignment & Little-Endian Byte Order:**
   * All memory reads/writes use `from_le_bytes` and `to_le_bytes`.
   * Misaligned accesses (`addr % 2 != 0` for 16-bit, `addr % 4 != 0` for 32-bit, `addr % 8 != 0` for 64-bit) immediately return `Trap::LoadAddressMisaligned(addr)` or `Trap::StoreAddressMisaligned(addr)`.
7. **`IllegalInstruction` Stays `u32`:**
   * Even on `RV64`, instruction encodings are 32 bits wide (`Trap::IllegalInstruction(u32)`), while faulting addresses and `mtval`/`stval` are 64 bits wide (`Addr = u64`).
8. **Guest Compiler Flags for Milestones 1–3:**
   * Compile guest binaries with `riscv64imac-unknown-none-elf` and `-C target-feature=-m,-a,-c` during Milestones 1–3 so `rustc` does not emit compressed 16-bit or multiply instructions before Milestone 4 is implemented.

---

## 5. Platform Memory Map (QEMU `virt` / `xv6-riscv` Layout)

Lay out `SystemBus` using these exact physical addresses from Day 1 so `xv6-riscv` boots without modifying its hardware constants:

| Physical Address Range | Size | Device / Region | Milestone |
| :--- | :--- | :--- | :--- |
| `0x0200_0000 .. 0x0201_0000` | 64 KiB | **CLINT** (`msip`, `mtimecmp`, `mtime`) | Milestone 5 |
| `0x0C00_0000 .. 0x1000_0000` | 64 MiB | **PLIC** (External Interrupt Controller: UART IRQ 10, VirtIO IRQ 1) | Milestone 5 / 6 |
| `0x1000_0000 .. 0x1000_0100` | 256 B | **16550 UART** (Serial Console) | Milestone 3 |
| `0x1000_1000 .. 0x1000_2000` | 4 KiB | **VirtIO-Block** (MMIO Virtual Disk) | Milestone 6 |
| `0x8000_0000 .. 0x8800_0000` | 128 MiB | **`FlatRam`** (Physical DRAM; Reset Vector `0x8000_0000`, Initial `sp = 0x8800_0000`) | Milestone 1 |

*Any unmapped physical address outside these ranges returns `Err(Trap::LoadAccessFault(addr))` or `Err(Trap::StoreAccessFault(addr))`.*

---

## 6. Upfront Skeleton Contracts & Intentional Compromises

### Day-1 Core Signatures (`purev-isa` & `purev-core`)

```rust
// purev-isa/src/lib.rs
#![no_std]

pub type Addr = u64;
pub type RegValue = u64;

pub const DRAM_BASE: Addr = 0x8000_0000;
pub const DRAM_SIZE: u64  = 128 * 1024 * 1024; // 128 MiB
pub const INITIAL_SP: Addr = DRAM_BASE + DRAM_SIZE;
pub const INTERRUPT_BIT: RegValue = 1 << 63;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register(pub u8); // Invariant: 0..32

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsaConfig {
    pub m: bool,      // Multiply/Divide (including *W)
    pub a: bool,      // Atomics (lr.w/sc.w, lr.d/sc.d, amo*)
    pub c: bool,      // Compressed 16-bit instructions
    pub custom: bool, // Custom-0 thesis instructions
}

impl IsaConfig {
    pub const RV64I: Self = Self { m: false, a: false, c: false, custom: false };
    pub const RV64IMAC: Self = Self { m: true, a: true, c: true, custom: false };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trap {
    InstructionAddressMisaligned(Addr), // Cause 0
    InstructionAccessFault(Addr),       // Cause 1
    IllegalInstruction(u32),            // Cause 2 (32-bit instruction word)
    Breakpoint,                         // Cause 3
    LoadAddressMisaligned(Addr),        // Cause 4
    LoadAccessFault(Addr),              // Cause 5
    StoreAddressMisaligned(Addr),       // Cause 6
    StoreAccessFault(Addr),             // Cause 7
    EnvironmentCallFromUMode,           // Cause 8
    EnvironmentCallFromSMode,           // Cause 9
    EnvironmentCallFromMMode,           // Cause 11
    InstructionPageFault(Addr),         // Cause 12
    LoadPageFault(Addr),                // Cause 13
    StorePageFault(Addr),               // Cause 15 (Cause 14 is reserved in spec)
    SupervisorSoftwareInterrupt,        // Interrupt + 1
    MachineSoftwareInterrupt,           // Interrupt + 3
    SupervisorTimerInterrupt,           // Interrupt + 5
    MachineTimerInterrupt,              // Interrupt + 7
    SupervisorExternalInterrupt,        // Interrupt + 9
    MachineExternalInterrupt,           // Interrupt + 11
}

// purev-core/src/bus/mod.rs
pub trait SerialBackend {
    fn write_byte(&mut self, byte: u8);
    fn read_byte(&mut self) -> Option<u8>;
}

pub trait BlockBackend {
    fn read_sector(&self, sector: u64, buf: &mut [u8; 512]) -> Result<(), ()>;
    fn write_sector(&mut self, sector: u64, buf: &[u8; 512]) -> Result<(), ()>;
    fn capacity_sectors(&self) -> u64;
}

pub trait Memory {
    fn read8(&self, addr: Addr) -> Result<u8, Trap>;
    fn read16(&self, addr: Addr) -> Result<u16, Trap>;
    fn read32(&self, addr: Addr) -> Result<u32, Trap>;
    fn read64(&self, addr: Addr) -> Result<u64, Trap>;

    fn write8(&mut self, addr: Addr, val: u8) -> Result<(), Trap>;
    fn write16(&mut self, addr: Addr, val: u16) -> Result<(), Trap>;
    fn write32(&mut self, addr: Addr, val: u32) -> Result<(), Trap>;
    fn write64(&mut self, addr: Addr, val: u64) -> Result<(), Trap>;
}

// purev-core/src/journal.rs
#[derive(Debug, Clone, Copy)]
pub enum UndoAction {
    Register { reg: Register, old_val: RegValue },
    Memory8  { addr: Addr, old_val: u8 },
    Memory16 { addr: Addr, old_val: u16 },
    Memory32 { addr: Addr, old_val: u32 },
    Memory64 { addr: Addr, old_val: u64 },
}

// purev-core/src/cpu/decode.rs
pub fn decode(raw_bits: u32, isa: IsaConfig) -> Result<(Instruction, u64), Trap>;
```

### Intentional v0.1 Compromises (Zero-Rewrite Upgrade Path)

| Subsystem | v0.1 Shortcut | How It Upgrades Later Without Rewriting |
| :--- | :--- | :--- |
| **Trap Handling** | `TrapHandler` panics on `Err(Trap)`, printing `pc`, `a7`, and `a0` on `ecall`. | Replace panic body with `mepc`/`sepc`, `mcause`/`scause`, `mtval`/`stval`, `medeleg`/`mideleg`, and `mtvec`/`stvec` jump in Milestone 5. |
| **MMIO Devices** | Addresses `< 0x8000_0000` return `AccessFault`. | Add `Uart16550`, `Clint`, `Plic`, and `VirtioBlk` match arms inside `SystemBus` without changing `CpuCore`. |
| **CSRs & PMP** | No `CsrFile` in v0.1. | Add `CsrFile` + `Zicsr` in Milestone 5; stub `pmpaddr0..=15` and `pmpcfg0`/`pmpcfg2` as read/write registers so `xv6` `mret` succeeds. |
| **MMU / Paging** | Bare physical addressing (`vaddr == paddr`). | Pass `TranslationContext { mode, satp, mstatus }` + `AccessType` into `SystemBus::translate()` in Milestone 5 for `Sv39`. |
| **Host Frontend** | `purev-cli` (`std`) for rapid testing. | `purev-core` is `#![no_std]` + `alloc` with `&[u8]` ELF loading and `SerialBackend`/`BlockBackend` traits; `purev-limine` wraps it in Milestone 7. |

### Recommended `purev-core` Directory Structure

```text
crates/purev-core/src/
├── lib.rs               // #![no_std] + extern crate alloc; public Machine API
├── machine.rs           // Iterative Machine::run() / step() loop + TrapHandler
├── journal.rs           // 8-slot stack UndoAction rollback buffer
├── elf.rs               // no_std goblin elf64 loader (&[u8] -> entry_pc)
├── cpu/
│   ├── mod.rs           // CpuCore (regs, pc, PrivilegeMode, csrs)
│   ├── regs.rs          // RegFile ([u64; 32], silent x0 no-op)
│   ├── decode.rs        // Bitfield extraction + IsaConfig validation -> (Instruction, len)
│   ├── alu.rs           // Pure stateless 64-bit & *W arithmetic/shift functions
│   ├── branch.rs        // Pure stateless branch comparison functions
│   └── csr.rs           // Stubbed in M1, populated in M5 (with PMP stubs)
└── bus/
    ├── mod.rs           // Memory trait, SerialBackend, BlockBackend, SystemBus, Sv39 walker
    ├── ram.rs           // FlatRam (0x8000_0000..0x8800_0000, strict alignment, little-endian)
    ├── uart.rs          // PC16550 UART at 0x1000_0000 (Milestone 3)
    ├── clint.rs         // CLINT timer at 0x0200_0000 (Milestone 5)
    ├── plic.rs          // Minimal single-hart PLIC at 0x0C00_0000 (Milestone 5)
    └── virtio_blk.rs    // VirtIO-Block MMIO disk at 0x1000_1000 (Milestone 6)
```

---

## 7. Step-by-Step Milestone Roadmap (`v0.1.0` $\to$ `v1.0.0`)

### Milestone 1: Minimal Working `RV64I` Loop (`v0.1.0`)
* **Build:**
  1. Set up workspace (`purev-isa`, `purev-core`, `purev-cli`) with `#![no_std]` + `alloc` in `purev-core`.
  2. Implement `Register(u8)`, `RegFile([u64; 32])`, 8-slot stack `Journal`, and `FlatRam` (`0x8000_0000..0x8800_0000`, 128 MiB) wrapped in `SystemBus`.
  3. Implement `load_elf(&[u8], &mut impl Memory)` using `goblin` (`elf64`) to map `PT_LOAD` segments and zero-fill `.bss`.
  4. Implement `decode` and `execute` for **17 core `RV64I` instructions**: `lui`, `auipc`, `addi`, `addiw`, `add`, `addw`, `sub`, `subw`, `beq`, `bne`, `lw`, `ld`, `sw`, `sd`, `jal`, `jalr`, `ecall`.
  5. Configure `TrapHandler` to panic with `pc`, `a7`, and `a0` diagnostics on `Trap::EnvironmentCallFromMMode`.
* **Definition of Done:** Unit tests verify `x0` immutability, 2/4/8-byte alignment traps, and `Journal::rollback()`. A `no_std` Rust guest compiled with `riscv64imac-unknown-none-elf` (`-C target-feature=-m,-a,-c`) sums an array in a loop, triggers `ecall`, and halts with `a0 == expected_sum`.

### Milestone 2: Complete `RV64I` & Official Architectural Compliance (`v0.2.0`)
* **Build:**
  1. Complete all remaining `RV64I` ALU and immediate instructions: shifts (`sll`, `srl`, `sra`, `slli`, `srli`, `srai` with `& 0x3F`; `sllw`, `srlw`, `sraw`, `slliw`, `srliw`, `sraiw` with `& 0x1F` and bit-25 illegal check), logical ops (`xor`, `or`, `and`, `xori`, `ori`, `andi`), and comparisons (`slt`, `sltu`, `slti`, `sltiu`).
  2. Add sub-word loads/stores (`lb`, `lbu`, `lh`, `lhu`, `lwu`, `sb`, `sh` with proper sign/zero extension) and remaining branches (`blt`, `bge`, `bltu`, `bgeu`).
  3. Build an automated `cargo test` harness in `purev-cli` running the official `riscv-tests` (`rv64ui-p-*`) ELF binaries (checking `gp (x3) == 1` on `ecall`).
* **Definition of Done:** 100% pass rate across all `rv64ui-p-*` architectural tests.

### Milestone 3: 16550 UART MMIO & Interactive Rust Guests (`v0.3.0`)
* **Build:**
  1. Implement `Uart16550` mapped at `0x1000_0000..0x1000_0100` inside `SystemBus`, forwarding TX writes (`THR`) to `SerialBackend`, reading RX bytes (`RBR`), and reporting TX-ready (`0x20`) and RX-data-ready (`0x01`) on `LSR`.
  2. Create a guest crate implementing `core::fmt::Write` over `0x1000_0000` (`println!`).
* **Definition of Done:** Running compiled guest ELFs prints formatted text live to the host terminal and reads interactive keyboard input.

### Milestone 4: Hardware Extensions (`M`, `C`, `A`) (`v0.4.0`)
* **Build:**
  1. **`RV64M`:** Implement `mul`, `mulh`, `mulhsu`, `mulhu`, `div`, `divu`, `rem`, `remu` plus 32-bit `*W` variants (`mulw`, `divw`, `divuw`, `remw`, `remuw`), strictly obeying non-trapping division-by-zero rules. Verify against `rv64um-p-*`.
  2. **`RV64C`:** Detect 16-bit compressed instructions (`(raw_bits & 0b11) != 0b11`) when `isa.c` is enabled, expand into 32-bit `Instruction` equivalents, and return `Ok((inst, 2))`. Verify against `rv64uc-p-*`.
  3. **`RV64A`:** Add `reservation: Option<Addr>` to `SystemBus` and implement `lr.w`/`sc.w`, `lr.d`/`sc.d`, and `amo*.w` / `amo*.d` read-modify-write instructions using `Journal` for fault atomicity. Verify against `rv64ua-p-*`.
* **Definition of Done:** 100% pass rate across `rv64um-p-*`, `rv64uc-p-*`, and `rv64ua-p-*`. Standard Rust binaries compiled with `-O2` on `riscv64imac-unknown-none-elf` run without `-C target-feature=-m,-a,-c`.

### Milestone 5: Privileged Architecture, CSRs, Interrupts & `Sv39` Paging (`v0.5.0`)
* **Build:**
  1. Implement `PrivilegeMode` (`Machine`, `Supervisor`, `User`), `CsrFile` (`mstatus`, `sstatus`, `mie`, `sie`, `mip`, `sip`, `mtvec`, `stvec`, `mepc`, `sepc`, `mcause`, `scause`, `mtval`, `stval`, `mscratch`, `sscratch`, `medeleg`, `mideleg`, `satp`, `mhartid`, and stubbed `pmpaddr0..=15` / `pmpcfg0`/`pmpcfg2`), `Zicsr` instructions (`csrrw`, `csrrs`, `csrrc`, `csrrwi`, `csrrsi`, `csrrci`), and `mret` / `sret` / `sfence.vma`.
  2. Add **CLINT** (`0x0200_0000`, 64-bit `mtime`/`mtimecmp`/`msip`) and a minimal single-hart **PLIC** (`0x0C00_0000`, priority, pending, enable, threshold, claim/complete) inside `SystemBus`.
  3. Implement **Sv39 Virtual Memory** (3-level page-table walker via `satp` inside `SystemBus::translate()`), raising `InstructionPageFault` (12), `LoadPageFault` (13), or `StorePageFault` (15) on permission/valid violations.
* **Definition of Done:** A bare-metal test kernel boots in M-mode, delegates traps via `medeleg`/`mideleg`, enables `Sv39` paging in S-mode, drops to U-mode via `sret`, handles preemptive CLINT timer interrupts, and resumes cleanly.

### Milestone 6: Booting `xv6-riscv` & Optional Architecture Spike (`v0.6.0`)
* **Build:**
  1. Implement **VirtIO-Block MMIO** (`0x1000_1000..0x1000_2000`) backed by `BlockBackend` (wrapping a `&mut [u8]` disk image slice) and wired to PLIC IRQ 1.
  2. Build MIT `xv6-riscv` (`CFLAGS += -march=rv64imac -mabi=lp64`) to produce `kernel` ELF and `fs.img`.
  3. *(Optional Thesis Spike)* Plug an L1 I-Cache/D-Cache & branch-predictor profiler into `SystemBus`/`Machine`, add custom `custom-0` (`0b000_1011`) instructions to `purev-asm` and `purev-core`, and publish statistical `BENCHMARKS.md` reports.
* **Definition of Done:** `purev-cli` boots unmodified `xv6-riscv` at `0x8000_0000`, mounts `fs.img` over VirtIO-Block, and drops into an interactive Unix shell (`ls`, `cat`, `fork`, `pipe`) over the 16550 UART console.

### Milestone 7: Bare-Metal `x86_64` Limine Unikernel (`v1.0.0`)
* **Build:**
  1. Create `crates/purev-limine` targeting `x86_64-unknown-none` using the `limine` crate (`BaseRevision`, `MemoryMapRequest`, `ModuleRequest`, `FramebufferRequest`).
  2. Initialize minimal `x86_64` GDT, IDT (with a double-fault IST stack), and a static `.bss` heap (`linked_list_allocator`) so `purev-core`'s `alloc` collections run in Ring 0.
  3. Implement `SerialBackend` writing simultaneously to physical `COM1` (`0x3F8` via `outb`/`inb`) and an 8x16 bitmap font renderer on the Limine GOP framebuffer.
  4. Bundle the `xv6` kernel ELF and `fs.img` as Limine boot modules (`limine.conf`), read their raw slices via `ModuleRequest`, and launch `purev_core::Machine::run()`.
* **Definition of Done:** Build a bootable `.iso` with `xorriso` and boot it in QEMU (`qemu-system-x86_64 -cdrom purev.iso -serial stdio`) or off a USB flash drive on physical `x86_64` hardware, running `xv6-riscv` with zero host OS underneath.
