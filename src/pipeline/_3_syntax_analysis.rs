//! Third step of the pipeline, Parsing token lines into individual instructions, as well as computing label offsets
use std::collections::HashMap;

use crate::utils::{
    error::{
        AssemblerError,
        Stage::Syntax,
        SyntaxError::{self, Internal, WrongArguments},
    },
    instruction::{
        BType, IType, ITypeJump, ITypeMemory, ITypeShifts, Instruction, JType, RType, STypeMemory,
    },
    operands::{BLabel, Immediate, JLabel, Offset, Register, Shamt},
    token::Token,
};

/// Parses ``cleaned_tokens`` stream along with ``symbol_table`` into the corresponding ``instructions`` structs, as well as resolving label offsets
pub(super) fn parse(
    cleaned_tokens: &[Token],
    symbol_table: &HashMap<String, usize>,
) -> Result<Vec<Instruction>, AssemblerError> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for (index, line) in cleaned_tokens
        .split_inclusive(|t| matches!(t, Token::NewLine(_)))
        .enumerate()
    {
        let Some(line_split) = line.len().checked_sub(1) else {
            return Err(AssemblerError::internal(Syntax(Internal)));
        };

        let (line, newline) = line.split_at(line_split);

        if line.is_empty() {
            return Err(AssemblerError::internal(Syntax(Internal)));
        }

        let Token::NewLine(newline) = newline[0] else {
            return Err(AssemblerError::internal(Syntax(Internal)));
        };

        let Some(pc_counter) = index.checked_mul(4) else {
            return Err(AssemblerError::internal(Syntax(Internal)));
        };

        let instruction = match parse_instruction(line, symbol_table, pc_counter) {
            Ok(a) => a,
            Err(err) => return Err(AssemblerError::new_user(Syntax(err), newline)),
        };
        instructions.push(instruction);
    }
    Ok(instructions)
}

/// Parses individual token line into its instruction
fn parse_instruction(
    tokens: &[Token],
    symbol_table: &HashMap<String, usize>,
    pc_counter: usize,
) -> Result<Instruction, SyntaxError> {
    let Token::Identifier(mnemonic) = &tokens[0] else {
        return Err(SyntaxError::InvalidToken(tokens[0].clone()));
    };

    let operands = &tokens[1..];

    Ok(match mnemonic.as_str() {
        "add" => Instruction::Add(generate_rtype(operands)?),
        "sub" => Instruction::Sub(generate_rtype(operands)?),
        "or" => Instruction::Or(generate_rtype(operands)?),
        "and" => Instruction::And(generate_rtype(operands)?),
        "xor" => Instruction::Xor(generate_rtype(operands)?),

        "addi" => Instruction::Addi(generate_itype(operands)?),
        "andi" => Instruction::Andi(generate_itype(operands)?),
        "xori" => Instruction::Xori(generate_itype(operands)?),
        "ori" => Instruction::Ori(generate_itype(operands)?),

        "slli" => Instruction::Slli(generate_itype_shifts(operands)?),
        "srli" => Instruction::Srli(generate_itype_shifts(operands)?),

        "lw" => Instruction::Lw(generate_itype_memory(operands)?),
        "lb" => Instruction::Lb(generate_itype_memory(operands)?),

        "sw" => Instruction::Sw(generate_stype_memory(operands)?),
        "sb" => Instruction::Sb(generate_stype_memory(operands)?),

        "beq" => Instruction::Beq(generate_btype(operands, pc_counter, symbol_table)?),
        "bne" => Instruction::Bne(generate_btype(operands, pc_counter, symbol_table)?),
        "blt" => Instruction::Blt(generate_btype(operands, pc_counter, symbol_table)?),
        "bge" => Instruction::Bge(generate_btype(operands, pc_counter, symbol_table)?),

        "jal" => Instruction::Jal(generate_jtype(operands, pc_counter, symbol_table)?),

        "jalr" => Instruction::Jalr(generate_itype_jump(operands)?),

        _ => {
            return Err(SyntaxError::NonExistentMnemonic(mnemonic.to_owned()));
        }
    })
}

// ---

fn generate_jtype(
    operands: &[Token],
    pc_counter: usize,
    symbol_table: &HashMap<String, usize>,
) -> Result<JType, SyntaxError> {
    if operands.len() != 3 || !operands[1].eq(&Token::Comma) {
        return Err(WrongArguments);
    }

    Ok(JType {
        rd: Register::new(&operands[0])?,
        jlabel: JLabel::new(&operands[2], symbol_table, pc_counter)?,
    })
}

fn generate_btype(
    operands: &[Token],
    pc_counter: usize,
    symbol_table: &HashMap<String, usize>,
) -> Result<BType, SyntaxError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(WrongArguments);
    }

    Ok(BType {
        rs1: Register::new(&operands[0])?,
        rs2: Register::new(&operands[2])?,
        blabel: BLabel::new(&operands[4], symbol_table, pc_counter)?,
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
        rs1: Register::new(&operands[0])?,
        offset: Offset::new(&operands[2])?,
        rbase: Register::new(&operands[4])?,
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
        rd: Register::new(&operands[0])?,
        offset: Offset::new(&operands[2])?,
        rs1: Register::new(&operands[4])?,
    })
}

fn generate_itype_shifts(operands: &[Token]) -> Result<ITypeShifts, SyntaxError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(WrongArguments);
    }

    Ok(ITypeShifts {
        rd: Register::new(&operands[0])?,
        rs1: Register::new(&operands[2])?,
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
            rd: Register::new(&operands[0])?,
            offset: Offset::new(&operands[2])?,
            rs1: Register::new(&operands[4])?,
        });
    } else if operands.len() == 5 && operands[1].eq(&Token::Comma) && operands[3].eq(&Token::Comma)
    {
        return Ok(ITypeJump {
            rd: Register::new(&operands[0])?,
            rs1: Register::new(&operands[2])?,
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
        rd: Register::new(&operands[0])?,
        rs1: Register::new(&operands[2])?,
        imm: Immediate::new(&operands[4])?,
    })
}

fn generate_rtype(operands: &[Token]) -> Result<RType, SyntaxError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Comma) || !operands[3].eq(&Token::Comma) {
        return Err(WrongArguments);
    }

    Ok(RType {
        rd: Register::new(&operands[0])?,
        rs1: Register::new(&operands[2])?,
        rs2: Register::new(&operands[4])?,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        pipeline::_3_syntax_analysis::parse,
        utils::{
            error::AssemblerError,
            instruction::{IType, Instruction, RType},
            operands::{Immediate, Register},
            token::Token,
        },
    };

    #[test]
    fn two_instructions() -> Result<(), AssemblerError> {
        let tokens = vec![
            Token::Identifier("add".to_owned()),
            Token::Identifier("x1".to_owned()),
            Token::Comma,
            Token::Identifier("x2".to_owned()),
            Token::Comma,
            Token::Identifier("x3".to_owned()),
            Token::NewLine(1),
            Token::Identifier("xori".to_owned()),
            Token::Identifier("x23".to_owned()),
            Token::Comma,
            Token::Identifier("sp".to_owned()),
            Token::Comma,
            Token::Literal("300".to_owned()),
            Token::NewLine(3),
        ];
        let output = parse(tokens.as_slice(), &HashMap::new())?;

        let expected = vec![
            Instruction::Add(RType {
                rd: Register::X1,
                rs1: Register::X2,
                rs2: Register::X3,
            }),
            Instruction::Xori(IType {
                rd: Register::X23,
                rs1: Register::X2,
                imm: Immediate::new(&Token::Literal("300".to_string())).unwrap(),
            }),
        ];

        assert_eq!(output, expected);

        Ok(())
    }
}
