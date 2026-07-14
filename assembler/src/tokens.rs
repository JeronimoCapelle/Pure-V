#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Literal(String),
    Identifier(String),

    Coma,
    Colon,
    OpeningParenthesis,
    ClosingParenthesis,
    NewLine,
}
