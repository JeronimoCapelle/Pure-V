//! Batch RISC-V assembler pipeline (`&str -> Vec<Token> -> Vec<Instruction> ->
//! Vec<u32>`).

#![no_std]

extern crate alloc;

pub mod encoder;
pub mod error;
pub mod lexer;
pub mod mnemonic;
pub mod parser;

use alloc::vec::Vec;

pub use error::{AsmError, EncodeError, LexError, ParseError, Span};

/// Assembles RISC-V assembly source text into 32-bit machine-code words.
pub fn assemble(source: &str) -> Result<Vec<u32>, AsmError> {
    let tokens = lexer::tokenize(source)?;
    let instructions = parser::parse(&tokens)?;
    Ok(encoder::encode(&instructions)?)
}

/// Assembles RISC-V assembly source text into a little-endian byte buffer.
pub fn assemble_bytes(source: &str) -> Result<Vec<u8>, AsmError> {
    let tokens = lexer::tokenize(source)?;
    let instructions = parser::parse(&tokens)?;
    Ok(encoder::encode_bytes(&instructions)?)
}
