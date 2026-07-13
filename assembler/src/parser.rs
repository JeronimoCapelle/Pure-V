use crate::opcode::{
    BType, BigLabel, IType, ITypeJump, ITypeMemory, ITypeShifts, Immediate, JType, Label, Offset,
    Opcode,
    ParsingError::{
        self, EmptyLineError, NonExistentOpcodeError, NonExistentRegisterError, TexttoNumericError,
        WrongArgumentCountError,
    },
    RType, Register, STypeMemory, Shamt,
};

pub fn parse_opcode(input: &str) -> std::result::Result<Opcode, ParsingError> {
    let (inst, args) = match input.split_once(char::is_whitespace) {
        None => return Err(NonExistentOpcodeError),
        Some(a) => a,
    };

    Ok(match inst {
        "add" => Opcode::ADD(generate_rtype(args)?),
        "sub" => Opcode::SUB(generate_rtype(args)?),
        "or" => Opcode::OR(generate_rtype(args)?),
        "and" => Opcode::AND(generate_rtype(args)?),
        "xor" => Opcode::XOR(generate_rtype(args)?),

        "addi" => Opcode::ADDI(generate_itype(args)?),
        "andi" => Opcode::ANDI(generate_itype(args)?),
        "xori" => Opcode::XORI(generate_itype(args)?),
        "orii" => Opcode::ORI(generate_itype(args)?),

        "slli" => Opcode::SLLI(generate_itype_shifts(args)?),
        "srli" => Opcode::SRLI(generate_itype_shifts(args)?),

        "lw" => Opcode::LW(generate_itype_memory(args)?),
        "lb" => Opcode::LB(generate_itype_memory(args)?),

        "sw" => Opcode::SW(generate_stype_memory(args)?),
        "sb" => Opcode::SB(generate_stype_memory(args)?),

        "beq" => Opcode::BEQ(generate_btype(args)?),
        "bne" => Opcode::BNE(generate_btype(args)?),
        "blt" => Opcode::BLT(generate_btype(args)?),
        "bge" => Opcode::BGE(generate_btype(args)?),

        "jal" => Opcode::JAL(generate_jtype(args)?),

        "jalr" => Opcode::JALR(generate_itype_jump(args)?),

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

fn generate_itype(args: &str) -> Result<IType, ParsingError> {
    todo!()
}

fn generate_rtype(args: &str) -> Result<RType, ParsingError> {
    Ok(RType {
        destination: Register::new(args[0])?,
        first_source: Register::new(args[1])?,
        second_source: Register::new(args[2])?,
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
