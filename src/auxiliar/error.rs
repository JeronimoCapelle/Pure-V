use std::panic::Location;

use crate::auxiliar::{error::Responsable::Internal, token::Token};

// ----

#[derive(Debug)]
pub struct AssemblerError {
    rust_location: &'static Location<'static>,
    assembly_stage: Stage,
    offending_input_line_number: usize,
    who: Responsable,
}

impl AssemblerError {
    #[track_caller]
    pub const fn new(assembly_stage: Stage, offending_input_line_number: usize) -> Self {
        Self {
            rust_location: Location::caller(),
            assembly_stage,
            offending_input_line_number,
            who: Responsable::User,
        }
    }
    pub const fn internal(assembly_stage: Stage) -> Self {
        Self {
            rust_location: Location::caller(),
            assembly_stage,
            who: Internal,
            offending_input_line_number: 0,
        }
    }
}

#[derive(Debug)]
pub enum Responsable {
    Internal,
    User,
}

#[derive(Debug)]
pub enum Stage {
    Tokenizer,
    SymbolCollection,
    Syntax(SyntaxError),
    MathematicalProof,
}

#[derive(Debug)]
pub enum SyntaxError {
    BiggerValue(i128, i128),  //expected, recieved
    SmallerValue(i128, i128), //expected, recieved
    OddValue(Token),
    TexttoNumeric(Token),
    NonExistentRegister(Token),
    NonExistentMnemonic(Token),
    WrongArguments,
    InvalidStartingWord(Token),
    InvalidWord(Token),
    Translation(Token),
    Impossible,
    Empty,
}
