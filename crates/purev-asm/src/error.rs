//! Error types and source spans for the `purev-asm` batch pipeline.

use core::fmt;

/// Half-open byte offset span `[start, end)` in assembly source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    /// Inclusive starting byte offset.
    pub start: usize,
    /// Exclusive ending byte offset.
    pub end: usize,
}

/// Errors emitted by the lexical analysis stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexError {
    /// Encountered an unexpected character at `span`.
    UnexpectedChar {
        /// Source span of the invalid character.
        span: Span,
    },
    /// Encountered a malformed numeric literal at `span`.
    InvalidLiteral {
        /// Source span of the invalid literal.
        span: Span,
    },
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar { span } => {
                write!(f, "unexpected character at {}..{}", span.start, span.end)
            },
            Self::InvalidLiteral { span } => {
                write!(f, "invalid numeric literal at {}..{}", span.start, span.end)
            },
        }
    }
}

impl core::error::Error for LexError {}

/// Errors emitted by the parsing and symbol-resolution stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// Expected a register operand at `span`.
    ExpectedRegister(Span),
    /// Expected a comma separator at `span`.
    ExpectedComma(Span),
    /// Expected an immediate literal or label symbol at `span`.
    ExpectedImmOrLabel(Span),
    /// Expected `'('` at `span`.
    ExpectedLParen(Span),
    /// Expected `')'` at `span`.
    ExpectedRParen(Span),
    /// Expected newline or end-of-file after a statement at `span`.
    ExpectedStatementEnd(Span),
    /// Expected a valid `fence` ordering operand (`i`, `o`, `r`, `w`) at
    /// `span`.
    InvalidFenceOperand(Span),
    /// Label defined more than once at `span`.
    DuplicateLabel(Span),
    /// Referenced branch/jump target label was not defined at `span`.
    UndefinedLabel(Span),
    /// Immediate or branch/jump offset exceeded architectural bit-width at
    /// `span`.
    ImmediateOutOfRange(Span),
    /// Unexpected end of token stream while parsing an instruction.
    UnexpectedEof,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedRegister(s) => write!(f, "expected register at {}..{}", s.start, s.end),
            Self::ExpectedComma(s) => write!(f, "expected ',' at {}..{}", s.start, s.end),
            Self::ExpectedImmOrLabel(s) => {
                write!(f, "expected immediate or label at {}..{}", s.start, s.end)
            },
            Self::ExpectedLParen(s) => write!(f, "expected '(' at {}..{}", s.start, s.end),
            Self::ExpectedRParen(s) => write!(f, "expected ')' at {}..{}", s.start, s.end),
            Self::ExpectedStatementEnd(s) => {
                write!(f, "expected end of statement at {}..{}", s.start, s.end)
            },
            Self::InvalidFenceOperand(s) => {
                write!(
                    f,
                    "invalid fence ordering operand at {}..{}",
                    s.start, s.end
                )
            },
            Self::DuplicateLabel(s) => write!(f, "duplicate label at {}..{}", s.start, s.end),
            Self::UndefinedLabel(s) => write!(f, "undefined label at {}..{}", s.start, s.end),
            Self::ImmediateOutOfRange(s) => {
                write!(f, "immediate out of range at {}..{}", s.start, s.end)
            },
            Self::UnexpectedEof => write!(f, "unexpected end of input"),
        }
    }
}

impl core::error::Error for ParseError {}

/// Errors emitted by the machine-code encoding stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeError {
    /// Immediate, shift amount, or offset cannot be represented in the
    /// instruction format.
    ImmediateOutOfRange,
    /// Branch or jump offset is not 2-byte aligned.
    UnalignedBranchTarget,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImmediateOutOfRange => write!(f, "immediate or offset out of range"),
            Self::UnalignedBranchTarget => write!(f, "branch or jump offset is not 2-byte aligned"),
        }
    }
}

impl core::error::Error for EncodeError {}

/// Unified error type returned by end-to-end assembly helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsmError {
    /// Error during lexical analysis.
    Lex(LexError),
    /// Error during parsing or label resolution.
    Parse(ParseError),
    /// Error during instruction encoding.
    Encode(EncodeError),
}

impl From<LexError> for AsmError {
    fn from(err: LexError) -> Self {
        Self::Lex(err)
    }
}

impl From<ParseError> for AsmError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}

impl From<EncodeError> for AsmError {
    fn from(err: EncodeError) -> Self {
        Self::Encode(err)
    }
}

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lex(err) => write!(f, "lex error: {err}"),
            Self::Parse(err) => write!(f, "parse error: {err}"),
            Self::Encode(err) => write!(f, "encode error: {err}"),
        }
    }
}

impl core::error::Error for AsmError {}
