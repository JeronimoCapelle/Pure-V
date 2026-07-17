//! First step in the assembly pipeline, turning the program string into tokens
use crate::utils::{
    error::{AssemblerError, Stage::Tokenizer},
    token::Token,
};

/// Turns a program string into its respective tokens, strips comments
pub(super) fn tokenize(contents_str: &str) -> Result<Vec<Token>, AssemblerError> {
    let contents: Vec<char> = contents_str.chars().collect();

    let mut tokens: Vec<Token> = Vec::new();
    let mut line_count = 0;
    let mut i = 0;

    while i < contents.len() {
        match contents[i] {
            '\n' => {
                line_count += 1;
                tokens.push(Token::NewLine(line_count));
                i += 1;
            }
            char if char.is_whitespace() => {
                i += 1;
            }
            '#' => {
                while contents.get(i).unwrap_or(&'\n') != &'\n' {
                    i += 1;
                }
            }
            '/' => {
                if i + 1 < contents.len() && contents[i] == contents[i + 1] {
                    while contents.get(i).unwrap_or(&'\n') != &'\n' {
                        i += 1;
                    }
                }
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            ':' => {
                tokens.push(Token::Colon);
                i += 1;
            }
            '(' => {
                tokens.push(Token::OpeningParenthesis);
                i += 1;
            }
            ')' => {
                tokens.push(Token::ClosingParenthesis);
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
                    .unwrap_or(contents.len() - i);

                tokens.push(Token::Literal(
                    contents[i..i + end]
                        .iter()
                        .collect::<String>()
                        .trim()
                        .to_owned(),
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
                    .unwrap_or(contents.len() - i);

                tokens.push(Token::Identifier(
                    contents[i..i + end]
                        .iter()
                        .collect::<String>()
                        .trim()
                        .to_owned(),
                ));
                i += end;
            }
            _ => {
                return Err(AssemblerError::new_user(Tokenizer, line_count + 1));
            }
        }
    }
    if !matches!(tokens.last(), Some(Token::NewLine(_))) {
        tokens.push(Token::NewLine(line_count + 1));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::{
        pipeline::_1_lexical_analysis::tokenize,
        utils::{error::AssemblerError, token::Token},
    };

    #[test]
    fn empty() -> Result<(), AssemblerError> {
        let output = tokenize("")?;
        let expected = vec![Token::NewLine(1)];
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn simple_instruction() -> Result<(), AssemblerError> {
        let output = tokenize("add x1,x2,x3")?;

        let expected = vec![
            Token::Identifier("add".to_owned()),
            Token::Identifier("x1".to_owned()),
            Token::Comma,
            Token::Identifier("x2".to_owned()),
            Token::Comma,
            Token::Identifier("x3".to_owned()),
            Token::NewLine(1),
        ];
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn simple_label() -> Result<(), AssemblerError> {
        let output = tokenize("  label  :  ")?;
        let expected = vec![
            Token::Identifier("label".to_owned()),
            Token::Colon,
            Token::NewLine(1),
        ];
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn instruction_and_label() -> Result<(), AssemblerError> {
        let output = tokenize("add x1,    x2 ,x3\nlabel:")?;
        let expected = vec![
            Token::Identifier("add".to_owned()),
            Token::Identifier("x1".to_owned()),
            Token::Comma,
            Token::Identifier("x2".to_owned()),
            Token::Comma,
            Token::Identifier("x3".to_owned()),
            Token::NewLine(1),
            Token::Identifier("label".to_owned()),
            Token::Colon,
            Token::NewLine(2),
        ];
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn instruction_label_instruction() -> Result<(), AssemblerError> {
        let output = tokenize("add x1,    x2 ,x3\nlabel:\nxori x23,    sp ,300")?;
        let expected = vec![
            Token::Identifier("add".to_owned()),
            Token::Identifier("x1".to_owned()),
            Token::Comma,
            Token::Identifier("x2".to_owned()),
            Token::Comma,
            Token::Identifier("x3".to_owned()),
            Token::NewLine(1),
            Token::Identifier("label".to_owned()),
            Token::Colon,
            Token::NewLine(2),
            Token::Identifier("xori".to_owned()),
            Token::Identifier("x23".to_owned()),
            Token::Comma,
            Token::Identifier("sp".to_owned()),
            Token::Comma,
            Token::Literal("300".to_owned()),
            Token::NewLine(3),
        ];
        assert_eq!(output, expected);
        Ok(())
    }
}
