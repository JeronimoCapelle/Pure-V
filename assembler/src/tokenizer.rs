use crate::tokens::Token;

pub fn tokenize_contents(contents: &str) -> Vec<Vec<Token>> {
    let contents: Vec<&str> = contents.split('\n').collect();

    let mut tokenized_contents: Vec<Vec<Token>> = Vec::new();

    for i in 0..contents.len() {
        let chars = contents[i].chars().collect();
        let token_line = tokenize_line(chars);
        if !token_line.is_empty() {
            tokenized_contents.push(token_line);
        }
    }
    tokenized_contents
}

fn tokenize_line(chars: Vec<char>) -> Vec<Token> {
    let mut holder: Vec<Token> = Vec::new();

    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            char if char.is_whitespace() => {
                i += 1;
            }
            '#' => {
                i = chars.len();
            }
            '/' => {
                if i + 1 < chars.len() && chars[i] == chars[i + 1] {
                    i = chars.len();
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
                        *x == ' ' || *x == '\t' || *x == ':' || *x == ',' || *x == '(' || *x == ')'
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
                        *x == ' ' || *x == '\t' || *x == ':' || *x == ',' || *x == '(' || *x == ')'
                    })
                    .unwrap_or(chars.len() - i);

                holder.push(Token::Identifier(
                    chars[i..i + end].iter().collect::<String>(),
                ));
                i += end;
            }
            _ => {}
        };
    }

    holder
}
