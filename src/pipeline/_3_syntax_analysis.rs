use std::collections::HashMap;

use crate::auxiliar::{
    error::{
        AssemblerError,
        Stage::Syntax,
        SyntaxError::{self, Empty, Impossible, WrongArguments},
    },
    instruction::{
        BType, IType, ITypeJump, ITypeMemory, ITypeShifts, Instruction, JType, RType, STypeMemory,
    },
    operands::{BigLabel, Immediate, Label, Offset, Register, Shamt},
    token::Token,
};

pub fn parse(
    tokens: &[Token],
    symbol_table: &HashMap<String, usize>,
) -> Result<Vec<Instruction>, AssemblerError> {
    let mut statements: Vec<Instruction> = Vec::new();
    for (index, line) in tokens
        .split_inclusive(|t| matches!(t, Token::NewLine(_)))
        .enumerate()
    {
        let (line, newline) = line.split_at(line.len() - 1);

        if line.is_empty() {
            return Err(AssemblerError::internal(Syntax(Empty)));
        }

        let Token::NewLine(newline) = newline[0] else {
            return Err(AssemblerError::internal(Syntax(Impossible)));
        };

        let pc_counter = index * 4;

        let instruction = match parse_statement(line, symbol_table, pc_counter) {
            Ok(a) => a,
            Err(err) => return Err(AssemblerError::new(Syntax(err), newline)),
        };
        statements.push(instruction);
    }
    Ok(statements)
}

fn parse_statement<'a>(
    tokens: &'a [Token],
    symbol_table: &'a HashMap<String, usize>,
    pc_counter: usize,
) -> Result<Instruction, SyntaxError> {
    let Token::Identifier(mnemonic) = &tokens[0] else {
        return Err(SyntaxError::InvalidStartingWord(tokens[0].clone()));
    };

    let operands = &tokens[1..];

    Ok(match mnemonic.as_str() {
        "add" => Instruction::ADD(generate_rtype(operands)?),
        "sub" => Instruction::SUB(generate_rtype(operands)?),
        "or" => Instruction::OR(generate_rtype(operands)?),
        "and" => Instruction::AND(generate_rtype(operands)?),
        "xor" => Instruction::XOR(generate_rtype(operands)?),

        "addi" => Instruction::ADDI(generate_itype(operands)?),
        "andi" => Instruction::ANDI(generate_itype(operands)?),
        "xori" => Instruction::XORI(generate_itype(operands)?),
        "ori" => Instruction::ORI(generate_itype(operands)?),

        "slli" => Instruction::SLLI(generate_itype_shifts(operands)?),
        "srli" => Instruction::SRLI(generate_itype_shifts(operands)?),

        "lw" => Instruction::LW(generate_itype_memory(operands)?),
        "lb" => Instruction::LB(generate_itype_memory(operands)?),

        "sw" => Instruction::SW(generate_stype_memory(operands)?),
        "sb" => Instruction::SB(generate_stype_memory(operands)?),

        "beq" => Instruction::BEQ(generate_btype(operands, pc_counter, symbol_table)?),
        "bne" => Instruction::BNE(generate_btype(operands, pc_counter, symbol_table)?),
        "blt" => Instruction::BLT(generate_btype(operands, pc_counter, symbol_table)?),
        "bge" => Instruction::BGE(generate_btype(operands, pc_counter, symbol_table)?),

        "jal" => Instruction::JAL(generate_jtype(operands, pc_counter, symbol_table)?),

        "jalr" => Instruction::JALR(generate_itype_jump(operands)?),

        _ => {
            return Err(SyntaxError::NonExistentMnemonic(tokens[0].clone()));
        }
    })
}

// ---

fn generate_jtype<'a>(
    operands: &'a [Token],
    pc_counter: usize,
    symbol_table: &'a HashMap<String, usize>,
) -> Result<JType, SyntaxError> {
    if operands.len() != 3 || !operands[1].eq(&Token::Comma) {
        return Err(WrongArguments);
    }

    Ok(JType {
        destination: Register::new(&operands[0])?,
        big_label: BigLabel::new(&operands[2], symbol_table, pc_counter)?,
    })
}

fn generate_btype<'a>(
    operands: &'a [Token],
    pc_counter: usize,
    symbol_table: &'a HashMap<String, usize>,
) -> Result<BType, SyntaxError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(WrongArguments);
    }

    Ok(BType {
        first_source: Register::new(&operands[0])?,
        second_source: Register::new(&operands[2])?,
        label: Label::new(&operands[4], symbol_table, pc_counter)?,
    })
}

fn generate_stype_memory(operands: &[Token]) -> Result<STypeMemory, SyntaxError> {
    if operands.len() != 6
        || !operands[1].eq(&Token::Comma)
        || !operands[3].eq(&Token::OpeningParenthesis)
        || !operands[5].eq(&Token::ClosingParenthesis)
    {
        return Err(WrongArguments);
    }

    Ok(STypeMemory {
        source: Register::new(&operands[0])?,
        offset: Offset::new(&operands[2])?,
        base_address: Register::new(&operands[4])?,
    })
}

fn generate_itype_memory(operands: &[Token]) -> Result<ITypeMemory, SyntaxError> {
    if operands.len() != 6
        || !operands[1].eq(&Token::Comma)
        || !operands[3].eq(&Token::OpeningParenthesis)
        || !operands[5].eq(&Token::ClosingParenthesis)
    {
        return Err(WrongArguments);
    }

    Ok(ITypeMemory {
        destination: Register::new(&operands[0])?,
        offset: Offset::new(&operands[2])?,
        base_address: Register::new(&operands[4])?,
    })
}

fn generate_itype_shifts(operands: &[Token]) -> Result<ITypeShifts, SyntaxError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(WrongArguments);
    }

    Ok(ITypeShifts {
        destination: Register::new(&operands[0])?,
        source: Register::new(&operands[2])?,
        shamt: Shamt::new(&operands[4])?,
    })
}

fn generate_itype_jump(operands: &[Token]) -> Result<ITypeJump, SyntaxError> {
    if operands.len() == 6
        && operands[1].eq(&Token::Comma)
        && operands[3].eq(&Token::OpeningParenthesis)
        && operands[5].eq(&Token::ClosingParenthesis)
    {
        return Ok(ITypeJump {
            destination: Register::new(&operands[0])?,
            offset: Offset::new(&operands[2])?,
            target_address: Register::new(&operands[4])?,
        });
    } else if operands.len() == 5 && operands[1].eq(&Token::Comma) && operands[3].eq(&Token::Comma)
    {
        return Ok(ITypeJump {
            destination: Register::new(&operands[0])?,
            target_address: Register::new(&operands[2])?,
            offset: Offset::new(&operands[4])?,
        });
    }

    Err(WrongArguments)
}

fn generate_itype(operands: &[Token]) -> Result<IType, SyntaxError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(WrongArguments);
    }

    Ok(IType {
        destination: Register::new(&operands[0])?,
        source: Register::new(&operands[2])?,
        immediate: Immediate::new(&operands[4])?,
    })
}

fn generate_rtype(operands: &[Token]) -> Result<RType, SyntaxError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(WrongArguments);
    }

    Ok(RType {
        destination: Register::new(&operands[0])?,
        first_source: Register::new(&operands[2])?,
        second_source: Register::new(&operands[4])?,
    })
}
