use crate::{
    opcode::ParsingError::{self, TokenizerError},
    tokens::Token,
};

pub fn tokenize_contents(contents: &str) -> Result<Vec<Token>, ParsingError> {
    let contents: Vec<char> = contents.chars().collect();

    let mut tokenized_contents: Vec<Token> = Vec::new();

    let mut i = 0;

    while i < contents.len() {
        match contents[i] {
            char if char.is_whitespace() => {
                i += 1;
            }
            '#' => {
                while contents[i] != '\n' {
                    i += 1;
                }
            }
            '/' => {
                if i + 1 < contents.len() && contents[i] == contents[i + 1] {
                    while contents[i] != '\n' {
                        i += 1;
                    }
                }
            }
            ',' => {
                tokenized_contents.push(Token::Coma);
                i += 1;
            }
            ':' => {
                tokenized_contents.push(Token::Colon);
                i += 1;
            }
            '(' => {
                tokenized_contents.push(Token::OpeningParenthesis);
                i += 1;
            }
            ')' => {
                tokenized_contents.push(Token::ClosingParenthesis);
                i += 1;
            }
            '\n' => {
                tokenized_contents.push(Token::NewLine);
                i += 1;
            }
            char if char.is_numeric() || char.eq(&'+') || char.eq(&'-') => {
                let end = contents
                    .iter()
                    .skip(i)
                    .position(|x| {
                        *x == '/'
                            || *x == '#'
                            || *x == '\n'
                            || *x == ' '
                            || *x == '\t'
                            || *x == ':'
                            || *x == ','
                            || *x == '('
                            || *x == ')'
                    })
                    .unwrap_or(contents.len());

                tokenized_contents.push(Token::Literal(
                    contents[i..i + end].iter().collect::<String>(),
                ));
                i += end;
            }

            char if char.is_alphabetic() || char.eq(&'_') => {
                let end = contents
                    .iter()
                    .skip(i)
                    .position(|x| {
                        *x == '/'
                            || *x == '#'
                            || *x == '\n'
                            || *x == ' '
                            || *x == '\t'
                            || *x == ':'
                            || *x == ','
                            || *x == '('
                            || *x == ')'
                    })
                    .unwrap_or(contents.len());

                tokenized_contents.push(Token::Identifier(
                    contents[i..i + end].iter().collect::<String>(),
                ));
                i += end;
            }
            _ => {
                return Err(TokenizerError);
            }
        };
    }

    Ok(tokenized_contents)
}
