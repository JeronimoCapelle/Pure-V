use crate::tokens::Token;
use std::collections::HashMap;

pub fn generate_symbol_table(tokens: &Vec<Vec<Token>>) -> HashMap<String, u32> {
    let mut symbol_table: HashMap<String, u32> = HashMap::new();
    let mut pc_counter: u32 = 0;

    for line in tokens {
        if !(line).ends_with(&[Token::Colon]) {
            pc_counter += 4;
            continue;
        }
        let name = match &line[0] {
            Token::Identifier(a) => a,
            _ => {
                panic!("Syntax Error");
            }
        };

        symbol_table.insert(name.to_string(), pc_counter);
    }

    symbol_table
}
