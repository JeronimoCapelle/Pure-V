//! Lexical analysis stage (`&str -> Vec<Token>`).

mod cursor;
mod literal;

use alloc::vec::Vec;

use purev_isa::Register;

use self::{cursor::Cursor, literal::parse_literal};
use crate::{
    error::{LexError, Span},
    mnemonic::Mnemonic,
};

/// Lexical token kinds emitted by [`tokenize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind<'a> {
    /// Register operand (`x0`..`x31` or ABI name).
    Reg(Register),
    /// Instruction mnemonic.
    Mnemonic(Mnemonic),
    /// Parsed integer literal.
    Imm(i128),
    /// Label or symbol identifier.
    Identifier(&'a str),
    /// Comma separator (`,`).
    Comma,
    /// Label colon (`:`).
    Colon,
    /// Opening parenthesis (`(`).
    LParen,
    /// Closing parenthesis (`)`).
    RParen,
    /// End-of-line separator (`\n`).
    Newline,
}

/// Source token with half-open byte span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    /// Token kind and borrowed payload.
    pub kind: TokenKind<'a>,
    /// Source span in the input `&str`.
    pub span: Span,
}

/// Tokenizes assembly source text into a discrete `Vec<Token>`.
pub fn tokenize(source: &str) -> Result<Vec<Token<'_>>, LexError> {
    let mut cursor = Cursor::new(source);
    let mut tokens = Vec::new();

    loop {
        cursor.skip_trivia();
        let start = cursor.pos();
        let Some(ch) = cursor.bump() else {
            break;
        };
        let span_1 = Span {
            start,
            end: cursor.pos(),
        };

        let token = match ch {
            '\n' => Token {
                kind: TokenKind::Newline,
                span: span_1,
            },
            ',' => Token {
                kind: TokenKind::Comma,
                span: span_1,
            },
            ':' => Token {
                kind: TokenKind::Colon,
                span: span_1,
            },
            '(' => Token {
                kind: TokenKind::LParen,
                span: span_1,
            },
            ')' => Token {
                kind: TokenKind::RParen,
                span: span_1,
            },
            'a'..='z' | 'A'..='Z' | '_' | '.' => scan_ident(&mut cursor, start),
            '0'..='9' => scan_numeric(&mut cursor, start)?,
            '+' | '-' if cursor.first().is_some_and(|next| next.is_ascii_digit()) => {
                scan_numeric(&mut cursor, start)?
            },
            _ => return Err(LexError::UnexpectedChar { span: span_1 }),
        };

        tokens.push(token);
    }

    Ok(tokens)
}

fn scan_ident<'a>(cursor: &mut Cursor<'a>, start: usize) -> Token<'a> {
    cursor.eat_while(|c| c.is_alphanumeric() || c == '_' || c == '.' || c == '$');
    let end = cursor.pos();
    let word = cursor.slice(start, end);
    let span = Span { start, end };

    let kind = if let Some(reg) = Register::try_from_str(word) {
        TokenKind::Reg(reg)
    } else if let Some(mne) = Mnemonic::try_from_str(word) {
        TokenKind::Mnemonic(mne)
    } else {
        TokenKind::Identifier(word)
    };

    Token { kind, span }
}

fn scan_numeric<'a>(cursor: &mut Cursor<'a>, start: usize) -> Result<Token<'a>, LexError> {
    cursor.eat_while(|c| c.is_alphanumeric() || c == '_');
    let end = cursor.pos();
    let lit = cursor.slice(start, end);
    let span = Span { start, end };

    match parse_literal(lit) {
        Some(val) => Ok(Token {
            kind: TokenKind::Imm(val),
            span,
        }),
        None => Err(LexError::InvalidLiteral { span }),
    }
}
