use std::collections::HashMap;

use crate::auxiliar::{
    error::{
        AssemblerError,
        Stage::{self, SymbolCollection},
    },
    token::Token,
};

pub fn collect_symbols(
    tokens: &[Token],
) -> Result<(HashMap<String, usize>, Vec<Token>), AssemblerError> {
    let mut symbol_table: HashMap<String, usize> = HashMap::new();
    let mut symbol_free_tokens: Vec<Token> = Vec::new();
    let mut pc_counter: usize = 0;

    for line in tokens.split_inclusive(|t| matches!(t, Token::NewLine(_))) {
        // label statement
        if (line.len() == 3) && (line[1] == Token::Colon) {
            // expected identifier before colon
            let Token::Identifier(name) = &line[0] else {
                let Some(last_token) = line.last() else {
                    return Err(AssemblerError::internal(SymbolCollection));
                };

                let Token::NewLine(line_number) = last_token else {
                    return Err(AssemblerError::internal(SymbolCollection));
                };

                return Err(AssemblerError::new(Stage::SymbolCollection, *line_number));
            };

            symbol_table.insert(name.clone(), pc_counter);

            // TODO: WE ARE ASSUMING (if (not empty) and (not label) then inst) <= this could be false!!!
        } else if !line.len() == 1 {
            pc_counter += 4;
            symbol_free_tokens.append(&mut line.to_vec());
        }
    }

    Ok((symbol_table, symbol_free_tokens))
}
