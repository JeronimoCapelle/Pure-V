use std::io::Write;

use assembler::compile_string;

fn main() {
    let mut args = std::env::args().skip(1);
    let input_filename = args.next().expect("Filename not provided");

    let file_contents = std::fs::read_to_string(input_filename).expect("Failed to open file");

    let binary = compile_string(&file_contents).unwrap();

    let mut file = std::fs::File::create("output.bin").unwrap();

    file.write_all(binary.as_slice()).unwrap();
}
