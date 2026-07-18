//! module containing everything regarding the custom error type

use core::fmt;
use core::{error::Error, fmt::Debug, panic::Location};

use crate::utils::error::Stage::{Internal, Syntax};
use crate::utils::token::Token;

// ----
/// Error type used throughout the assembler for idiomatic error handling and custom error feedback
#[derive(Debug)]
pub struct AssemblerError {
    /// File and line number of the assembler source code where the error was created
    rust_location: &'static Location<'static>,
    /// The pipeline stage where the error occurred
    assembly_stage: Stage,
    /// The assembly source code line where the error occurred
    input_line_number: usize,
    /// The string containing the offending line, for display purposes
    input_line: String,
    /// Whether the error is caused by the user, or by the internal assembler design
    who: Responsible,
}

impl Error for AssemblerError {}

impl AssemblerError {
    /// Create a new error, in which the user is at fault, line contents is not initialized, it set later
    #[track_caller]
    pub(crate) const fn new_user(assembly_stage: Stage, input_line_number: usize) -> Self {
        Self {
            rust_location: Location::caller(),
            assembly_stage,
            input_line_number,
            who: Responsible::User,
            input_line: String::new(),
        }
    }
    /// Create a new error, in which the internal assembler is at fault
    pub(crate) const fn internal(assembly_stage: Stage) -> Self {
        Self {
            rust_location: Location::caller(),
            assembly_stage,
            who: Responsible::Internal,
            input_line_number: 0,
            input_line: String::new(),
        }
    }
    /// Fills in the line content for displaying the error to the user. (This is very hacky, change later)
    pub(crate) fn fill_line(&mut self, file: &str) -> Result<(), Self> {
        let Some(actual_line_num) = self.input_line_number.checked_sub(1) else {
            return Err(Self::internal(Internal));
        };
        let Some(line_content) = file.lines().nth(actual_line_num) else {
            return Err(Self::internal(Internal));
        };

        line_content.clone_into(&mut self.input_line);
        Ok(())
    }
}

impl fmt::Display for AssemblerError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "
  [ Error encountered: {:?} ]
 
  {:?}

  --> line:{}
  | 
  |  {} 
  |

  [ Location:{} ]
",
            self.who,
            self.assembly_stage,
            self.input_line_number,
            self.input_line,
            self.rust_location
        )
    }
}

impl From<SyntaxError> for AssemblerError {
    #[inline]
    fn from(value: SyntaxError) -> Self {
        Self::new_user(Syntax(value), 0)
    }
}

/// Enum for detailing who is at fault for the generated error
#[derive(Debug)]
pub enum Responsible {
    /// The error is the assemblers fault, unrecoverable error, these shouldn't occur
    Internal,
    /// The error is the users fault, usually typos or syntax errors
    User,
}

/// At what stage of the pipeline did the error occur, note that stages that are not present are impossible to fail
pub enum Stage {
    /// Error occurred at the tokenizer stage.
    Tokenizer,
    /// Error occurred at the symbol table and token cleaning stage
    SymbolCollection,
    /// Error occurred at the instruction parsing stage.
    Syntax(SyntaxError),
    /// Error occurred during operations which were impossible to happen
    Internal,
}

impl Debug for Stage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Tokenizer => write!(f, "Tokenizer error"),
            Self::SymbolCollection => write!(f, "Symbol Collection and Token Cleaning error"),
            Self::Internal => write!(f, "Internal unrecoverable error"),
            Self::Syntax(arg0) => f.debug_tuple("Syntax Error:\n  ").field(arg0).finish(),
        }
    }
}

/// Error sub-type for providing useful information during the syntax stage.
pub enum SyntaxError {
    /// The value provided is bigger than the maximum expected. (Expected, Received)
    BiggerValue(i128, i128), //expected, received

    /// The value provided is smaller than the minimum expected. (Expected, Received)
    SmallerValue(i128, i128), //expected, received

    /// The value provided is odd, which is not allowed in offsets.
    OddValue(i128),

    /// The value couldn't be converted into a numeric
    TexttoNumeric(String),

    /// The register provided is not a known register name or alias
    NonExistentRegister(String),

    /// The Mnemonic provided is not known or implemented
    NonExistentMnemonic(String),

    /// Arguments provided weren't correct with what the instruction indicated. (Make this more verbose)
    WrongArguments,

    /// Flags provided for fence were incorrect
    WrongFlags,

    /// The token type provided is not what was expected. (Literal when looking for a register name, or an Identifier when computing an offset)
    InvalidToken(Token),

    /// The label couldn't be found in the symbols table. Usually a typo.
    Translation(String),

    /// Reserved variant for internal Assembler Errors, This ones shouldn't occur during the assemblers use.
    Internal,
}

impl Debug for SyntaxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BiggerValue(max, value) => write!(
                f,
                " Value provided {{ {value} }} is bigger than max {{ {max} }} ",
            ),
            Self::SmallerValue(min, value) => write!(
                f,
                " Value provided {{ {value} }} is smaller than min {{ {min} }} "
            ),
            Self::OddValue(offset) => write!(f, "Offset {{{offset}}} is an Odd value."),
            Self::TexttoNumeric(string) => write!(
                f,
                " Could not convert {{{string}}} to a number, make sure it is one. "
            ),
            Self::NonExistentRegister(string) => {
                write!(f, " Could not identify {{{string}}} register ")
            }
            Self::NonExistentMnemonic(string) => write!(
                f,
                " Could not identify {{{string}}} mnemonic, it might be misspelled or not yet implemented "
            ),
            Self::Translation(string) => {
                write!(f, " Couldnt find reference to label {{{string}}} ")
            }
            Self::Internal => write!(f, "Internal assembler error"),
            Self::WrongArguments => {
                write!(f, " Invalid arguments were passed to this instruction ")
            }
            Self::InvalidToken(token) => write!(f, " Did not expect the token {{{token:?}}} "),
            Self::WrongFlags => write!(
                f,
                "Invalid flags were passed to the instruction, valid flags are iorw"
            ),
        }
    }
}
