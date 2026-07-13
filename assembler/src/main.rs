use crate::{opcode::Opcode, parser::parse_opcode};

mod file_cleaner;
mod label_parse;
mod opcode;
mod parser;
mod tokenizer;
mod tokens;

fn main() {
    let mut args = std::env::args().skip(1);

    let input_filename = args.next().expect("Filename not provided");

    let file_contents = std::fs::read_to_string(input_filename).expect("Failed to open file");

    let clean_file_contents = file_cleaner::clean_file(file_contents);

    std::fs::write("output.txt", clean_file_contents);

    let mut instructions = Vec::<Opcode>::new();

    let labels = parse_labels(&file_contents);

    for i in file_contents.split('\n') {
        instructions.push(parse_opcode(i));
    }
}
