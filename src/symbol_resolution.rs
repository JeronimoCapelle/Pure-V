use crate::structures::{ParsingError::NonIdentifier, Token, TrackedError};
use std::collections::HashMap;

pub fn collect_symbols(
    tokens: &[Token],
) -> Result<(HashMap<String, usize>, Vec<Token>), TrackedError> {
    let mut symbol_table: HashMap<String, usize> = HashMap::new();
    let mut symbol_free_tokens: Vec<Token> = Vec::new();
    let mut pc_counter: usize = 0;

    for line in tokens.split(|t| *t == Token::NewLine) {
        if line.ends_with(&[Token::Colon]) && line.len() == 2 {
            let Token::Identifier(name) = &line[0] else {
                return Err(TrackedError {
                    kind: NonIdentifier,
                    line: line!(),
                    file: file!(),
                });
            };
            symbol_table.insert(name.clone(), pc_counter);
        } else if !line.is_empty() {
            pc_counter += 4;
            symbol_free_tokens.append(&mut line.to_vec());
            symbol_free_tokens.push(Token::NewLine);
        }
    }

    symbol_free_tokens.pop();

    Ok((symbol_table, symbol_free_tokens))
}

#[cfg(test)]
mod tests {

    use crate::{lexical_analysis::tokenize, symbol_resolution::collect_symbols};

    #[test]
    fn empty_table() {
        let token_inputs = tokenize("addi x1, x0,x3").unwrap();

        let (hash_map, clean) = collect_symbols(&token_inputs).unwrap();

        assert!(hash_map.is_empty());
        assert_eq!(token_inputs, clean);
    }

    #[test]
    fn one_label() {
        let token_inputs = tokenize("addi x1, x0,x3\nlabel:").unwrap();
        let expected_clean = tokenize("addi x1, x0,x3").unwrap();

        let (hash_map, clean) = collect_symbols(&token_inputs).unwrap();

        assert_eq!(hash_map.len(), 1);
        assert!(hash_map.contains_key("label"));
        assert_eq!(*hash_map.get("label").unwrap(), 4);

        assert_eq!(clean, expected_clean);
    }
}
