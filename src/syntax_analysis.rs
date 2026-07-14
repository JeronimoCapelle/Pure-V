use std::collections::HashMap;

use crate::structures::{ParsingError::*, *};

pub fn parse(
    tokens: &[Token],
    symbol_table: &HashMap<String, usize>,
) -> Result<Vec<Instruction>, TrackedError> {
    let mut statements: Vec<Instruction> = Vec::new();
    for (index, line) in tokens.split(|t| *t == Token::NewLine).enumerate() {
        if line.is_empty() {
            continue;
        }
        let pc_counter = index * 4;
        statements.push(parse_statement(line, symbol_table, pc_counter)?);
    }
    Ok(statements)
}

fn parse_statement(
    tokens: &[Token],
    symbol_table: &HashMap<String, usize>,
    pc_counter: usize,
) -> std::result::Result<Instruction, TrackedError> {
    let mnemonic = match &tokens[0] {
        Token::Identifier(a) => a,
        _ => {
            return Err(TrackedError {
                kind: NonIdentifier,
                line: line!(),
                file: file!(),
            });
        }
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
            return Err(TrackedError {
                kind: NonExistentMnemonic,
                line: line!(),
                file: file!(),
            });
        }
    })
}

// ---

fn generate_jtype(
    operands: &[Token],
    pc_counter: usize,
    symbol_table: &HashMap<String, usize>,
) -> Result<JType, TrackedError> {
    if operands.len() != 3 || !operands[1].eq(&Token::Comma) {
        return Err(TrackedError {
            kind: WrongArgument,
            line: line!(),
            file: file!(),
        });
    }

    Ok(JType {
        destination: Register::new(&operands[0])?,
        big_label: BigLabel::new(&operands[2], symbol_table, pc_counter)?,
    })
}

fn generate_btype(
    operands: &[Token],
    pc_counter: usize,
    symbol_table: &HashMap<String, usize>,
) -> Result<BType, TrackedError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(TrackedError {
            kind: WrongArgument,
            line: line!(),
            file: file!(),
        });
    }

    Ok(BType {
        first_source: Register::new(&operands[0])?,
        second_source: Register::new(&operands[2])?,
        label: Label::new(&operands[4], symbol_table, pc_counter)?,
    })
}

fn generate_stype_memory(operands: &[Token]) -> Result<STypeMemory, TrackedError> {
    if operands.len() != 6
        || !operands[1].eq(&Token::Comma)
        || !operands[3].eq(&Token::OpeningParenthesis)
        || !operands[5].eq(&Token::ClosingParenthesis)
    {
        return Err(TrackedError {
            kind: WrongArgument,
            line: line!(),
            file: file!(),
        });
    }

    Ok(STypeMemory {
        source: Register::new(&operands[0])?,
        offset: Offset::new(&operands[2])?,
        base_address: Register::new(&operands[4])?,
    })
}

fn generate_itype_memory(operands: &[Token]) -> Result<ITypeMemory, TrackedError> {
    if operands.len() != 6
        || !operands[1].eq(&Token::Comma)
        || !operands[3].eq(&Token::OpeningParenthesis)
        || !operands[5].eq(&Token::ClosingParenthesis)
    {
        return Err(TrackedError {
            kind: WrongArgument,
            line: line!(),
            file: file!(),
        });
    }

    Ok(ITypeMemory {
        destination: Register::new(&operands[0])?,
        offset: Offset::new(&operands[2])?,
        base_address: Register::new(&operands[4])?,
    })
}

fn generate_itype_shifts(operands: &[Token]) -> Result<ITypeShifts, TrackedError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(TrackedError {
            kind: WrongArgument,
            line: line!(),
            file: file!(),
        });
    }

    Ok(ITypeShifts {
        destination: Register::new(&operands[0])?,
        source: Register::new(&operands[2])?,
        shamt: Shamt::new(&operands[4])?,
    })
}

fn generate_itype_jump(operands: &[Token]) -> Result<ITypeJump, TrackedError> {
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

    Err(TrackedError {
        kind: WrongArgument,
        line: line!(),
        file: file!(),
    })
}

fn generate_itype(operands: &[Token]) -> Result<IType, TrackedError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(TrackedError {
            kind: WrongArgument,
            line: line!(),
            file: file!(),
        });
    }

    Ok(IType {
        destination: Register::new(&operands[0])?,
        source: Register::new(&operands[2])?,
        immediate: Immediate::new(&operands[4])?,
    })
}

fn generate_rtype(operands: &[Token]) -> Result<RType, TrackedError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(TrackedError {
            kind: WrongArgument,
            line: line!(),
            file: file!(),
        });
    }

    Ok(RType {
        destination: Register::new(&operands[0])?,
        first_source: Register::new(&operands[2])?,
        second_source: Register::new(&operands[4])?,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        lexical_analysis::tokenize,
        structures::{Instruction, Offset, Register, STypeMemory, Token},
        syntax_analysis::parse_statement,
    };

    #[test]
    fn load() {
        let inst = parse_statement(
            tokenize("sw x7, 2044(x0)").unwrap().as_slice(),
            &HashMap::new(),
            0,
        )
        .unwrap();

        let ideal = Instruction::SW(STypeMemory {
            source: Register::X7,
            offset: Offset::new(&Token::Literal("2044".to_string())).unwrap(),
            base_address: Register::X0,
        });

        assert_eq!(inst, ideal);
    }
}
