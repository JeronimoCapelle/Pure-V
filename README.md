# hart64

A deterministic, `#![no_std]` 64-bit RISC-V (`RV64IMAC`) system emulator and two-pass assembler written in Rust. Designed around strict hardware invariants, zero-allocation exception rollback, and a decoupled memory bus capable of running both under a host OS CLI and on bare-metal `x86_64`.

[![CI](https://github.com/YOUR_USERNAME/hart64/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/hart64/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![no_std](https://img.shields.io/badge/core-no__std-orange.svg)](#architecture)

---

## Architecture

The core simulation engine (`hart64-core`) has zero dependencies on the host operating system (`#![no_std]` + `alloc`). All external interactions—RAM, MMIO devices, serial consoles, and block storage—route through trait boundaries, allowing the identical CPU core to run inside desktop test harnesses or a bare-metal `x86_64` unikernel.

```text
 ┌───────────────────────────────────────────────────────────────┐
 │                            Machine                            │
 │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────┐  │
 │  │    CpuCore    │  │    Journal    │  │     SystemBus     │  │
 │  │ - regs (u64)  │  │ - 8 undo slots│  │ - FlatRam (128MB) │  │
 │  │ - pc   (u64)  │  └───────────────┘  │ - 16550 UART      │  │
 │  │ - priv_mode   │                     │ - CLINT / PLIC    │  │
 │  │ - csrs        │                     │ - VirtIO-Block    │  │
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
                       │ - mtvec / stvec │
                       └─────────────────┘
```

### Workspace Layout

| Crate         | Environment            | Responsibility                                                                                                                 |
| :------------ | :--------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `hart64-isa`  | `#![no_std]`           | Shared hardware types (`Addr`, `RegValue`, `Register`), decoded `Instruction` IR, `Trap` cause codes, and runtime `IsaConfig`. |
| `hart64-asm`  | `#![no_std]` + `alloc` | Two-pass RISC-V assembler (`&str -> Vec<Token> -> Vec<Instruction> -> Vec<u32>`) with custom opcode support.                   |
| `hart64-core` | `#![no_std]` + `alloc` | Execution engine, 8-slot stack rollback `Journal`, `SystemBus`, MMIO peripherals, `Sv39` MMU, and `goblin` ELF64 loader.       |
| `hart64-cli`  | `std`                  | Host runner, `riscv-tests` compliance harness, raw-mode UART terminal, and `criterion` benchmarks.                             |

---

## ISA & Hardware Implementation Status

- [x] **`RV64I`** — Full 64-bit base integer instruction set (49 instructions, 32-bit `*W` sign-extension semantics, strict alignment traps)
- [x] **ELF64 Ingestion** — `no_std` `PT_LOAD` segment mapping and `.bss` zero-initialization
- [ ] **`RV64M`** — 64-bit and `*W` integer multiply/divide (spec-compliant non-trapping divide-by-zero)
- [ ] **`RV64C`** — 16-bit compressed instruction expansion
- [ ] **`RV64A`** — Load-Reserved / Store-Conditional (`lr.w`/`sc.w`, `lr.d`/`sc.d`) and `AMO` read-modify-write operations
- [ ] **Peripherals** — PC16550 UART (`0x1000_0000`), CLINT (`0x0200_0000`), PLIC (`0x0C00_0000`), VirtIO-Block (`0x1000_1000`)
- [ ] **Privileged & MMU** — `M`/`S`/`U` privilege modes, `Zicsr`, trap delegation (`medeleg`/`mideleg`), and `Sv39` 3-level page tables
- [ ] **OS & Bare-Metal Targets** — Booting `xv6-riscv` and `x86_64` Ring-0 execution via Limine

---

## Key Design Invariants

- **Precise Exceptions via Stack Undo Journal:** Instructions mutate registers and memory in place on the hot path while recording prior values into a fixed 8-slot stack `Journal`. If any micro-step faults mid-instruction, `Journal::rollback()` unwinds state changes in LIFO order before `TrapHandler` executes—guaranteeing zero heap allocations and exact architectural state on traps.
- **Runtime ISA Gating:** Instruction decoding validates enabled extensions (`M`, `A`, `C`, custom opcodes) at runtime via `IsaConfig` rather than Cargo `#[cfg]` features, allowing a single binary to enforce strict architectural subsets (`--isa rv64i`).
- **Spec-Compliant Corner Cases:** Writes to `x0` execute as silent no-ops; integer division by zero returns `u64::MAX` without trapping; misaligned halfword, word, and doubleword accesses raise exact address-misaligned traps.

---

## Quickstart

```bash
# Run unit tests and architectural compliance suites
cargo test --workspace

# Execute a compiled RV64 ELF binary
cargo run --release -p hart64-cli -- --isa rv64i path/to/guest.elf
```

---

## Documentation & Benchmarks

- [`ARCHITECTURE.md`](ARCHITECTURE.md) — Physical memory map, privilege/trap flow, and decoder/bus contracts.
- [`BENCHMARKS.md`](BENCHMARKS.md) — Instruction throughput (MIPS), cache-miss profiling, and statistical methodology.
