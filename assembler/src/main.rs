mod file_cleaner;
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
    let clean_file_contents = file_cleaner::clean_file(file_contents);

    let tokens = tokenizer::tokenize_contents(&clean_file_contents);

    let labels = symbol_table::generate_symbol_table(&tokens);
    let label_free_tokens = label_resolver::resolve_labels(&tokens, &labels);
}
