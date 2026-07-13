use crate::tokens::Token;

fn tokenize_contents(contents: &str) -> Vec<Token> {
    let chars: Vec<char> = contents.chars().collect();

    let mut holder: Vec<Token> = Vec::new();

    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            char if char.is_whitespace() => {
                i += 1;
            }
            '#' => {
                i += chars
                    .iter()
                    .skip(i)
                    .position(|x| *x == '\n')
                    .unwrap_or(chars.len());
            }
            '/' => {
                if i + 1 < chars.len() && chars[i] == chars[i + 1] {
                    i += chars
                        .iter()
                        .skip(i)
                        .position(|x| *x == '\n')
                        .unwrap_or(chars.len());
                }
            }
            ',' => {
                holder.push(Token::Coma);
                i += 1;
            }
            ':' => {
                holder.push(Token::Colon);
                i += 1;
            }
            '(' => {
                holder.push(Token::OpeningParenthesis);
                i += 1;
            }
            ')' => {
                holder.push(Token::ClosingParenthesis);
                i += 1;
            }

            char if char.is_numeric() || char.eq(&'+') || char.eq(&'-') => {
                let end = chars
                    .iter()
                    .skip(i)
                    .position(|x| {
                        *x == '\n'
                            || *x == ' '
                            || *x == '\t'
                            || *x == ':'
                            || *x == ','
                            || *x == '('
                            || *x == ')'
                    })
                    .unwrap_or(chars.len() - i);

                holder.push(Token::Literal(chars[i..i + end].iter().collect::<String>()));
                i += end;
            }

            char if char.is_alphabetic() || char.eq(&'_') => {
                let end = chars
                    .iter()
                    .skip(i)
                    .position(|x| {
                        *x == '\n'
                            || *x == ' '
                            || *x == '\t'
                            || *x == ':'
                            || *x == ','
                            || *x == '('
                            || *x == ')'
                    })
                    .unwrap_or(chars.len() - i);

                holder.push(Token::Identifier(
                    chars[i..i + end].iter().collect::<String>(),
                ));
                i += end;
            }
            _ => {
                i += 1;
            }
        };
    }

    holder
}
