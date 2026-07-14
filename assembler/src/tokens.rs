#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Literal(String),
    Identifier(String),

    Coma,
    Colon,
    OpeningParenthesis,
    ClosingParenthesis,
}
