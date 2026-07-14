use crate::structures::{ParsingError::Tokenizer, Token, TrackedError};

pub fn tokenize(contents: &str) -> Result<Vec<Token>, TrackedError> {
    let contents: Vec<char> = contents.chars().collect();

    let mut tokens: Vec<Token> = Vec::new();

    let mut i = 0;

    while i < contents.len() {
        match contents[i] {
            '\n' => {
                tokens.push(Token::NewLine);
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
                        .to_string(),
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
                        .to_string(),
                ));
                i += end;
            }
            _ => {
                return Err(TrackedError {
                    kind: Tokenizer,
                    line: line!(),
                    file: file!(),
                });
            }
        };
    }

    Ok(tokens)
}
#[cfg(test)]
mod tests {
    use crate::{lexical_analysis::tokenize, structures::Token};

    #[test]
    fn label() {
        assert_eq!(
            tokenize(" label_generic :   ").unwrap(),
            vec![Token::Identifier("label_generic".to_string()), Token::Colon]
        )
    }

    #[test]
    fn label_with_slash_comment() {
        assert_eq!(
            tokenize(" my_label : //Hello! ").unwrap(),
            vec![Token::Identifier("my_label".to_string()), Token::Colon]
        )
    }

    #[test]
    fn label_with_hashtag_comment() {
        assert_eq!(
            tokenize(" my_label : #    Hello!    ").unwrap(),
            vec![Token::Identifier("my_label".to_string()), Token::Colon]
        )
    }

    #[test]
    fn empty() {
        assert_eq!(tokenize("").unwrap(), vec![])
    }

    #[test]
    fn whitespace() {
        assert_eq!(tokenize("\t      ").unwrap(), vec![])
    }

    #[test]
    fn slash_comment() {
        assert_eq!(tokenize("   //COmments      ").unwrap(), vec![])
    }

    #[test]
    fn hashtag_comment() {
        assert_eq!(tokenize("   # COmments      ").unwrap(), vec![])
    }

    #[test]
    fn register_instruction() {
        assert_eq!(
            tokenize("add a1,  x10,  sp").unwrap(),
            vec![
                Token::Identifier("add".to_string()),
                Token::Identifier("a1".to_string()),
                Token::Comma,
                Token::Identifier("x10".to_string()),
                Token::Comma,
                Token::Identifier("sp".to_string())
            ]
        )
    }

    #[test]
    fn identifier() {
        assert_eq!(
            tokenize("  hello     ").unwrap(),
            vec![Token::Identifier("hello".to_string())]
        )
    }

    #[test]
    fn literal() {
        assert_eq!(
            tokenize("    123   ").unwrap(),
            vec![Token::Literal("123".to_string())]
        )
    }

    #[test]
    fn two_identifiers_and_comma() {
        assert_eq!(
            tokenize("  hello , there     ").unwrap(),
            vec![
                Token::Identifier("hello".to_string()),
                Token::Comma,
                Token::Identifier("there".to_string()),
            ]
        )
    }

    #[test]
    fn parenthesis() {
        assert_eq!(
            tokenize("  (hello , there)     ").unwrap(),
            vec![
                Token::OpeningParenthesis,
                Token::Identifier("hello".to_string()),
                Token::Comma,
                Token::Identifier("there".to_string()),
                Token::ClosingParenthesis,
            ]
        )
    }
}
