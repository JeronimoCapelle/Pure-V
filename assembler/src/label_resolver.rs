use std::collections::HashMap;

use crate::{opcode::Mnemonic, tokens::Token};

pub fn tokens_to_inst(tokens: Vec<Token>, symbols: &HashMap<String, u32>)->Vec<Mnemonic> {
    let program: Vec<Mnemonic>
    
    for token in tokens {
        let id = match token {
            Token::Identifier(a) => a,
            _ => {
                continue;
            }
        };

        if (symbols.contains_key(&id));
    }
}
