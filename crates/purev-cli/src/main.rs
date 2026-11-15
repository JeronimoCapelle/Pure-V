//! Desktop CLI runner for the `purev` RV64IMAC emulator.

use std::{env, fs, process::ExitCode};

use purev_core::Machine;
use purev_isa::{IsaConfig, Register, Trap};
use tracing_subscriber::EnvFilter;

#[allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "CLI binary output"
)]
fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let Some(elf_path) = env::args().nth(1) else {
        eprintln!("Usage: purev-cli <guest-elf>");
        return ExitCode::FAILURE;
    };

    let bytes = match fs::read(&elf_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to read '{elf_path}': {err}");
            return ExitCode::FAILURE;
        },
    };

    let mut machine = Machine::new(IsaConfig::RV64I);
    if let Err(err) = machine.load_elf(&bytes) {
        eprintln!("Failed to load ELF '{elf_path}': {err:?}");
        return ExitCode::FAILURE;
    }

    tracing::info!(
        pc = format_args!("{:#018x}", machine.cpu.pc),
        "starting guest"
    );

    match machine.run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(Trap::EnvironmentCallFromMMode) => {
            let pc = machine.cpu.pc;
            let a7 = machine.cpu.regs.read(Register::A7);
            let a0 = machine.cpu.regs.read(Register::A0);
            println!("Halted on ecall at pc={pc:#018x} a7={a7} a0={a0}");
            ExitCode::SUCCESS
        },
        Err(trap) => {
            let pc = machine.cpu.pc;
            eprintln!("Unhandled trap at pc={pc:#018x}: {trap:?}");
            ExitCode::FAILURE
        },
    }
}
