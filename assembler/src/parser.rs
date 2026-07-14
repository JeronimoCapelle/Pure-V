use std::collections::HashMap;

use crate::{
    opcode::{ParsingError::SymbolError, *},
    tokens::Token,
};

pub fn tokens_to_instructions(
    tokens: Vec<Token>,
    symbol_table: &HashMap<String, u32>,
) -> Result<Vec<Mnemonic>, ParsingError> {
    let mut program: Vec<Mnemonic> = Vec::new();
    for line in tokens.split(|t| *t == Token::NewLine) {
        program.push(parse_instruction(&line.to_vec())?);
    }
    Ok(program)
}

fn parse_instruction(input: &Vec<Token>) -> std::result::Result<Mnemonic, ParsingError> {
    let mnemonic = match &input[0] {
        Token::Identifier(a) => a,
        _ => {
            return Err(SymbolError);
        }
    };

    let args = &input[1..];

    Ok(match mnemonic {
        "add" => Mnemonic::ADD(generate_rtype(args)?),
        "sub" => Mnemonic::SUB(generate_rtype(args)?),
        "or" => Mnemonic::OR(generate_rtype(args)?),
        "and" => Mnemonic::AND(generate_rtype(args)?),
        "xor" => Mnemonic::XOR(generate_rtype(args)?),

        "addi" => Mnemonic::ADDI(generate_itype(args)?),
        "andi" => Mnemonic::ANDI(generate_itype(args)?),
        "xori" => Mnemonic::XORI(generate_itype(args)?),
        "orii" => Mnemonic::ORI(generate_itype(args)?),

        "slli" => Mnemonic::SLLI(generate_itype_shifts(args)?),
        "srli" => Mnemonic::SRLI(generate_itype_shifts(args)?),

        "lw" => Mnemonic::LW(generate_itype_memory(args)?),
        "lb" => Mnemonic::LB(generate_itype_memory(args)?),

        "sw" => Mnemonic::SW(generate_stype_memory(args)?),
        "sb" => Mnemonic::SB(generate_stype_memory(args)?),

        "beq" => Mnemonic::BEQ(generate_btype(args)?),
        "bne" => Mnemonic::BNE(generate_btype(args)?),
        "blt" => Mnemonic::BLT(generate_btype(args)?),
        "bge" => Mnemonic::BGE(generate_btype(args)?),

        "jal" => Mnemonic::JAL(generate_jtype(args)?),

        "jalr" => Mnemonic::JALR(generate_itype_jump(args)?),

        _ => return Err(NonExistentOpcodeError),
    })
}

// ---

fn generate_jtype(args: &str) -> Result<JType, ParsingError> {
    todo!()
}

fn generate_btype(args: &str) -> Result<BType, ParsingError> {
    todo!()
}

fn generate_stype_memory(args: &str) -> Result<STypeMemory, ParsingError> {
    todo!()
}

fn generate_itype_memory(args: &str) -> Result<ITypeMemory, ParsingError> {
    todo!()
}

fn generate_itype_shifts(args: &str) -> Result<ITypeShifts, ParsingError> {
    todo!()
}

fn generate_itype_jump(args: &str) -> Result<ITypeJump, ParsingError> {
    todo!()
}

fn generate_itype(args: &[Token]) -> Result<IType, ParsingError> {
    Ok(IType {
        destination: Register::new(&args[0]),
        source: Register::new(&args[2]),
        immediate: Immediate,
    })
}

fn generate_rtype(args: &[Token]) -> Result<RType, ParsingError> {
    Ok(RType {
        destination: Register::new(&args[0])?,
        first_source: Register::new(&args[2])?,
        second_source: Register::new(&args[4])?,
    })
}

// ----

fn parse_immediate(string: &str) -> Result<Immediate, ParsingError> {
    Ok(Immediate::new(string)?)
}

fn parse_shamt(string: &str) -> Result<Shamt, ParsingError> {
    let value = match string.parse::<u8>() {
        Ok(a) => a,
        Err(_) => return Err(TexttoNumericError),
    };
    Ok(Shamt::new(value)?)
}

fn parse_offset(string: &str) -> Result<Offset, ParsingError> {
    let value = match string.parse::<i16>() {
        Ok(a) => a,
        Err(_) => return Err(TexttoNumericError),
    };

    Ok(Offset::new(value)?)
}

fn parse_label(string: &str) -> Result<Label, ParsingError> {
    todo!()
}

fn parse_big_label(string: &str) -> Result<BigLabel, ParsingError> {
    todo!()
}
