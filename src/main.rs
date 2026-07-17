//! Primitive CLI version of the Pure-RV32I Assembler

use pure_rv32i::assemble_string;
use std::io::Write;

fn main() {
    let mut args = std::env::args();

    let program_name = args.next().unwrap_or("cargo run".to_owned());

    let Some(input_filename) = args.next() else {
        eprintln!("Error:\n\nNo filename provided, try: \n\n{program_name} {{your_program.s}}\n");
        return;
    };

    let file_contents = match std::fs::read_to_string(&input_filename) {
        Ok(a) => a,
        Err(a) => {
            eprintln!(
                "\n\tCouldnt read the file: {}\n\tBecause:\n\t{a}",
                &input_filename
            );
            return;
        }
    };

    let binary = match assemble_string(&file_contents) {
        Ok(a) => a,
        Err(a) => {
            eprintln!("{a}");
            return;
        }
    };

    println!("Assembly successful");

    let output_filename = match input_filename.rsplit_once('.') {
        Some(a) => a.0,
        None => &input_filename,
    };

    let output_file = format!("{output_filename}.bin");

    let mut file = match std::fs::File::create(output_filename) {
        Ok(a) => a,
        Err(a) => {
            println!("{a}");
            return;
        }
    };

    if let Err(a) = file.write_all(binary.as_slice()) {
        println!("{a}");
    }

    println!("Binary successfully written to output.bin");
    println!("Exiting...");
}
