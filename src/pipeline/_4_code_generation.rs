//! Fourth step of the pipeline, encoding the instruction types into their binary forms
use crate::utils::instruction::{
    BType, IType, ITypeJump, ITypeMemory, ITypeShifts, Instruction, JType, RType, STypeMemory,
};

/// Encodes the provided ``instructions`` vector into their binary encoded form according to RV32I
pub(super) fn assemble(instructions: Vec<Instruction>) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();

    for i in instructions {
        buffer.push(encode_instruction(i));
    }

    buffer
}

/// Encode individual ``instruction`` into its 4 byte form
fn encode_instruction(instruction: Instruction) -> u32 {
    match instruction {
        Instruction::Addi(itype) => {
            let funct3 = 0;
            generate_itype(funct3, itype)
        }

        Instruction::Add(rtype) => {
            let funct3 = 0;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }

        Instruction::Sub(rtype) => {
            let funct3 = 0;
            let funct7 = 32;
            generate_rtype(funct3, funct7, rtype)
        }

        Instruction::Bne(btype) => {
            let funct3 = 1;
            generate_btype(funct3, btype)
        }

        Instruction::Beq(btype) => {
            let funct3 = 0;
            generate_btype(funct3, btype)
        }

        Instruction::Blt(btype) => {
            let funct3 = 4;
            generate_btype(funct3, btype)
        }

        Instruction::Bge(btype) => {
            let funct3 = 5;
            generate_btype(funct3, btype)
        }

        Instruction::Jal(jtype) => generate_jtype(jtype),
        Instruction::Jalr(itype_jump) => {
            let funct3 = 0;
            generate_itype_jump(funct3, itype_jump)
        }

        Instruction::Lw(itype_memory) => {
            let funct3 = 2;
            generate_itype_memory(funct3, itype_memory)
        }

        Instruction::Sw(stype_memory) => {
            let funct3 = 2;
            generate_stype_memory(funct3, stype_memory)
        }

        Instruction::Lb(itype_memory) => {
            let funct3 = 0;
            generate_itype_memory(funct3, itype_memory)
        }

        Instruction::Sb(stype_memory) => {
            let funct3 = 0;
            generate_stype_memory(funct3, stype_memory)
        }

        Instruction::Slli(itype_shifts) => {
            let funct3 = 1;
            let funct7 = 0;
            generate_itype_shifts(funct3, funct7, itype_shifts)
        }

        Instruction::Srli(itype_shifts) => {
            let funct3 = 5;
            let funct7 = 0;
            generate_itype_shifts(funct3, funct7, itype_shifts)
        }

        Instruction::And(rtype) => {
            let funct3 = 7;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }

        Instruction::Or(rtype) => {
            let funct3 = 6;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }

        Instruction::Xor(rtype) => {
            let funct3 = 4;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }

        Instruction::Andi(itype) => {
            let funct3 = 7;
            generate_itype(funct3, itype)
        }

        Instruction::Ori(itype) => {
            let funct3 = 6;
            generate_itype(funct3, itype)
        }

        Instruction::Xori(itype) => {
            let funct3 = 4;
            generate_itype(funct3, itype)
        }
    }
}

const fn generate_rtype(funct3: u32, funct7: u32, rtype: RType) -> u32 {
    let opcode = 51;
    let destination = (rtype.rd.encode()) << 7;
    let funct3 = funct3 << 12;
    let first_source = (rtype.rs1.encode()) << 15;
    let second_source = (rtype.rs2.encode()) << 20;
    let funct7 = funct7 << 25;

    funct7 | second_source | first_source | funct3 | destination | opcode
}

fn generate_itype(funct3: u32, itype: IType) -> u32 {
    let opcode = 19;
    let destination = (itype.rd.encode()) << 7;
    let funct3 = funct3 << 12;
    let source = (itype.rs1.encode()) << 15;
    let immediate = itype.imm.encode() << 20;

    immediate | source | funct3 | destination | opcode
}

fn generate_itype_shifts(funct3: u32, funct7: u32, itype_shifts: ITypeShifts) -> u32 {
    let opcode = 19;
    let destination = (itype_shifts.rd.encode()) << 7;
    let funct3 = funct3 << 12;
    let source = (itype_shifts.rs1.encode()) << 15;
    let shamt = itype_shifts.shamt.encode() << 20;
    let funct7 = funct7 << 25;

    funct7 | shamt | source | funct3 | destination | opcode
}

fn generate_stype_memory(funct3: u32, stype_memory: STypeMemory) -> u32 {
    let opcode = 35;
    let offset_1 = (stype_memory.offset.encode() & 0b11111) << 7;
    let funct3 = funct3 << 12;
    let base_address = (stype_memory.rbase.encode()) << 15;
    let source = (stype_memory.rs1.encode()) << 20;
    let offset_2 = ((stype_memory.offset.encode() >> 5) & 0b_1111111) << 25;

    offset_2 | base_address | source | funct3 | offset_1 | opcode
}

fn generate_itype_memory(funct3: u32, itype_memory: ITypeMemory) -> u32 {
    let opcode = 3;
    let destination = (itype_memory.rd.encode()) << 7;
    let funct3 = funct3 << 12;
    let base_address = (itype_memory.rs1.encode()) << 15;
    let offset = itype_memory.offset.encode() << 20;

    offset | base_address | funct3 | destination | opcode
}

fn generate_btype(funct3: u32, btype: BType) -> u32 {
    let opcode = 99;
    let label_1 = ((btype.blabel.encode() >> 10) & 0b1) << 7;
    let label_2 = ((btype.blabel.encode()) & 0b1111) << 8;
    let funct3 = funct3 << 12;
    let first_source = (btype.rs1.encode()) << 15;
    let second_source = (btype.rs2.encode()) << 20;
    let label_3 = ((btype.blabel.encode() >> 4) & 0b11_1111) << 25;
    let label_4 = ((btype.blabel.encode() >> 11) & 0b1) << 31;

    label_4 | label_3 | second_source | first_source | funct3 | label_2 | label_1 | opcode
}

const fn generate_jtype(jtype: JType) -> u32 {
    let opcode = 111;
    let destination = (jtype.rd.encode()) << 7;
    let label_1 = ((jtype.jlabel.encode() >> 11) & 0b1111_1111) << 12;
    let label_2 = ((jtype.jlabel.encode() >> 10) & 0b1) << 20;
    let label_3 = ((jtype.jlabel.encode()) & 0b11_1111_1111) << 21;
    let label_4 = ((jtype.jlabel.encode() >> 19) & 0b1) << 31;

    label_4 | label_3 | label_2 | label_1 | destination | opcode
}

fn generate_itype_jump(funct3: u32, itypejump: ITypeJump) -> u32 {
    let opcode = 103;
    let destination = (itypejump.rd.encode()) << 7;
    let funct3 = funct3 << 12;
    let source = (itypejump.rs1.encode()) << 15;
    let immediate = itypejump.offset.encode() << 20;

    immediate | source | funct3 | destination | opcode
}
