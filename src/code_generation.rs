use crate::structures::{
    BType, IType, ITypeJump, ITypeMemory, ITypeShifts, Instruction, JType, RType, STypeMemory,
};

pub fn assemble(instructions: Vec<Instruction>) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();

    for i in instructions {
        buffer.push(encode_instruction(i));
    }

    buffer
}

fn encode_instruction(instruction: Instruction) -> u32 {
    match instruction {
        Instruction::ADDI(itype) => {
            let funct3 = 0;
            generate_itype(funct3, itype)
        }

        Instruction::ADD(rtype) => {
            let funct3 = 0;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }

        Instruction::SUB(rtype) => {
            let funct3 = 0;
            let funct7 = 32;
            generate_rtype(funct3, funct7, rtype)
        }

        Instruction::BNE(btype) => {
            let funct3 = 1;
            generate_btype(funct3, btype)
        }

        Instruction::BEQ(btype) => {
            let funct3 = 0;
            generate_btype(funct3, btype)
        }

        Instruction::BLT(btype) => {
            let funct3 = 4;
            generate_btype(funct3, btype)
        }

        Instruction::BGE(btype) => {
            let funct3 = 5;
            generate_btype(funct3, btype)
        }

        Instruction::JAL(jtype) => generate_jtype(jtype),
        Instruction::JALR(itype_jump) => {
            let funct3 = 0;
            generate_itype_jump(funct3, itype_jump)
        }

        Instruction::LW(itype_memory) => {
            let funct3 = 2;
            generate_itype_memory(funct3, itype_memory)
        }

        Instruction::SW(stype_memory) => {
            let funct3 = 2;
            generate_stype_memory(funct3, stype_memory)
        }

        Instruction::LB(itype_memory) => {
            let funct3 = 0;
            generate_itype_memory(funct3, itype_memory)
        }

        Instruction::SB(stype_memory) => {
            let funct3 = 0;
            generate_stype_memory(funct3, stype_memory)
        }

        Instruction::SLLI(itype_shifts) => {
            let funct3 = 1;
            let funct7 = 0;
            generate_itype_shifts(funct3, funct7, itype_shifts)
        }

        Instruction::SRLI(itype_shifts) => {
            let funct3 = 5;
            let funct7 = 0;
            generate_itype_shifts(funct3, funct7, itype_shifts)
        }

        Instruction::AND(rtype) => {
            let funct3 = 7;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }

        Instruction::OR(rtype) => {
            let funct3 = 6;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }

        Instruction::XOR(rtype) => {
            let funct3 = 4;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }

        Instruction::ANDI(itype) => {
            let funct3 = 7;
            generate_itype(funct3, itype)
        }

        Instruction::ORI(itype) => {
            let funct3 = 6;
            generate_itype(funct3, itype)
        }

        Instruction::XORI(itype) => {
            let funct3 = 4;
            generate_itype(funct3, itype)
        }
    }
}

fn generate_rtype(funct3: u32, funct7: u32, rtype: RType) -> u32 {
    let opcode = 51;
    let destination = (rtype.destination as u32) << 7;
    let funct3 = funct3 << 12;
    let first_source = (rtype.first_source as u32) << 15;
    let second_source = (rtype.second_source as u32) << 20;
    let funct7 = funct7 << 25;

    funct7 | second_source | first_source | funct3 | destination | opcode
}

fn generate_itype(funct3: u32, itype: IType) -> u32 {
    let opcode = 19;
    let destination = (itype.destination as u32) << 7;
    let funct3 = funct3 << 12;
    let source = (itype.source as u32) << 15;
    let immediate = itype.immediate.encode() << 20;

    immediate | source | funct3 | destination | opcode
}

fn generate_itype_shifts(funct3: u32, funct7: u32, itype_shifts: ITypeShifts) -> u32 {
    let opcode = 19;
    let destination = (itype_shifts.destination as u32) << 7;
    let funct3 = funct3 << 12;
    let source = (itype_shifts.source as u32) << 15;
    let shamt = itype_shifts.shamt.encode() << 20;
    let funct7 = funct7 << 25;

    funct7 | shamt | source | funct3 | destination | opcode
}

fn generate_stype_memory(funct3: u32, stype_memory: STypeMemory) -> u32 {
    let opcode = 35;
    let offset_1 = (stype_memory.offset.encode() & 0b11111) << 7;
    let funct3 = funct3 << 12;
    let base_address = (stype_memory.base_address as u32) << 15;
    let source = (stype_memory.source as u32) << 20;
    let offset_2 = ((stype_memory.offset.encode() >> 5) & 0b_1111111) << 25;

    offset_2 | base_address | source | funct3 | offset_1 | opcode
}

fn generate_itype_memory(funct3: u32, itype_memory: ITypeMemory) -> u32 {
    let opcode = 3;
    let destination = (itype_memory.destination as u32) << 7;
    let funct3 = funct3 << 12;
    let base_address = (itype_memory.base_address as u32) << 15;
    let offset = itype_memory.offset.encode() << 20;

    offset | base_address | funct3 | destination | opcode
}

fn generate_btype(funct3: u32, btype: BType) -> u32 {
    let opcode = 99;
    let label_1 = ((btype.label.encode() >> 11) & 0b1) << 7;
    let label_2 = ((btype.label.encode() >> 1) & 0b1111) << 8;
    let funct3 = funct3 << 12;
    let first_source = (btype.first_source as u32) << 15;
    let second_source = (btype.second_source as u32) << 20;
    let label_3 = ((btype.label.encode() >> 5) & 0b11_1111) << 25;
    let label_4 = ((btype.label.encode() >> 12) & 0b1) << 31;

    label_4 | label_3 | second_source | first_source | funct3 | label_2 | label_1 | opcode
}

fn generate_jtype(jtype: JType) -> u32 {
    let opcode = 111;
    let destination = (jtype.destination as u32) << 7;
    let label_1 = ((jtype.big_label.encode() >> 12) & 0b1111_1111) << 12;
    let label_2 = ((jtype.big_label.encode() >> 11) & 0b1) << 20;
    let label_3 = ((jtype.big_label.encode() >> 1) & 0b11_1111_1111) << 21;
    let label_4 = ((jtype.big_label.encode() >> 20) & 0b1) << 31;

    label_4 | label_3 | label_2 | label_1 | destination | opcode
}

fn generate_itype_jump(funct3: u32, itypejump: ITypeJump) -> u32 {
    let opcode = 103;
    let destination = (itypejump.destination as u32) << 7;
    let funct3 = funct3 << 12;
    let source = (itypejump.target_address as u32) << 15;
    let immediate = itypejump.offset.encode() << 20;

    immediate | source | funct3 | destination | opcode
}
