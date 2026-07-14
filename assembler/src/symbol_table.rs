use crate::{
    opcode::ParsingError::{self, SymbolError},
    tokens::Token,
};
use std::collections::HashMap;

pub fn extract_labels(
    tokens: Vec<Token>,
) -> Result<(HashMap<String, u32>, Vec<Token>), ParsingError> {
    let mut symbol_table: HashMap<String, u32> = HashMap::new();
    let mut label_free_tokens: Vec<Token> = Vec::new();
    let mut pc_counter: u32 = 0;

    for line in tokens.split(|t| *t == Token::NewLine) {
        if line.ends_with(&[Token::Colon]) && line.len() == 2 {
            let name = match &line[0] {
                Token::Identifier(a) => a,
                _ => return Err(SymbolError),
            };
            symbol_table.insert(name.to_string(), pc_counter);
        } else {
            pc_counter += 4;
            label_free_tokens.append(&mut line.to_vec());
            label_free_tokens.push(Token::NewLine);
        }
    }

    Ok((symbol_table, label_free_tokens))
}
