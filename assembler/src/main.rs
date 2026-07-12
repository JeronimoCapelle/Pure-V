use crate::{opcode::Opcode, parser::parse_opcode};

mod opcode;
mod parser;

fn main() {
    let input_filename = get_input();

    let file_contents = std::fs::read_to_string(input_filename).expect("Failed to open file");

    let mut instructions = Vec::<Opcode>::new();

    for i in file_contents.split('\n') {
        instructions.push(parse_opcode(i));
    }
}

fn get_input() -> String {
    let mut args = std::env::args().skip(1);
    args.next().expect("Filename not provided")
}
