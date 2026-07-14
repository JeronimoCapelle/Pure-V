mod label_resolver;
mod opcode;
mod parser;
mod symbol_table;
mod tokenizer;
mod tokens;

fn main() {
    let mut args = std::env::args().skip(1);
    let input_filename = args.next().expect("Filename not provided");

    let file_contents = std::fs::read_to_string(input_filename).expect("Failed to open file");

    let tokens = tokenizer::tokenize_contents(&file_contents).unwrap();

    let (labels, clean_tokens) = symbol_table::extract_labels(tokens).unwrap();

    let program = parser::tokens_to_instructions(clean_tokens, &labels).unwrap();
}
