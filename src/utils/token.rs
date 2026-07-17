//! Module containing Token enum

/// Token enum for use in the tokenization process
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    /// String representation of Literal numeric value, can be in decimal, hexadecimal, octal, binary, and includes negative symbol
    Literal(String),

    /// String representation of a word, could be a mnemonic, a label, a register, etc...
    Identifier(String),

    /// Token for a comma punctuation mark
    Comma,

    /// Token for a colon punctuation mark
    Colon,

    /// Token for an opening parenthesis punctuation mark
    OpeningParenthesis,

    /// Token for an opening parenthesis punctuation mark
    ClosingParenthesis,

    /// Token for a newline, contains the line this character was find, for signaling the user if they made in syntax errors
    NewLine(usize),
}
