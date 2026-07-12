use crate::opcode::{Opcode, RType, Register};

pub fn parse_opcode(input: &str) -> Opcode {
    let split: Vec<&str> = input.split_whitespace().collect();

    if split.is_empty() {
        return Opcode::NOP;
    }

    let (inst, args) = split.split_first().unwrap();
    let inst = *inst;
    let args = {
        let mut new_args = Vec::<&str>::new();
        for i in args {
            new_args.push(i.trim_matches(','));
        }
        new_args
    };

    match inst {
        "add" => Opcode::ADD(generate_R_type(args)),
        "sub" => Opcode::SUB(generate_R_type(args)),
        "or" => Opcode::OR(generate_R_type(args)),
        "and" => Opcode::AND(generate_R_type(args)),
        "xor" => Opcode::XOR(generate_R_type(args)),
        _ => todo!(),
    }
}

fn generate_r_type(args: Vec<&str>) -> RType {
    assert!(args.len() == 3);

    RType(
        parse_register(args[0]),
        parse_register(args[1]),
        parse_register(args[2]),
    )
}

fn parse_register(string: &str) -> Register {
    match string {
        "x0" => Register::X0,
        "zero" => Register::X0,

        "x1" => Register::X1,
        "ra" => Register::X1,

        "x2" => Register::X2,
        "sp" => Register::X2,

        "x3" => Register::X3,
        "gp" => Register::X3,

        "x4" => Register::X4,
        "tp" => Register::X4,

        //---
        "x5" => Register::X5,
        "t0" => Register::X5,

        "x6" => Register::X6,
        "t1" => Register::X6,

        "x7" => Register::X7,
        "t2" => Register::X7,

        //---
        "x8" => Register::X8,
        "fp" => Register::X8,
        "s0" => Register::X8,

        "x9" => Register::X9,
        "s1" => Register::X9,

        "x10" => Register::X10,
        "a0" => Register::X10,

        //---
        "x11" => Register::X11,
        "a1" => Register::X11,

        "x12" => Register::X12,
        "a2" => Register::X12,

        "x13" => Register::X13,
        "a3" => Register::X13,

        "x14" => Register::X14,
        "a4" => Register::X14,

        "x15" => Register::X15,
        "a5" => Register::X15,

        "x16" => Register::X16,
        "a6" => Register::X16,

        "x17" => Register::X17,
        "a7" => Register::X17,

        //---
        "x18" => Register::X18,
        "s2" => Register::X18,

        "x19" => Register::X19,
        "s3" => Register::X19,

        "x20" => Register::X20,
        "s4" => Register::X20,

        "x21" => Register::X21,
        "s5" => Register::X21,

        "x22" => Register::X22,
        "s6" => Register::X22,

        "x23" => Register::X23,
        "s7" => Register::X23,

        "x24" => Register::X24,
        "s8" => Register::X24,

        "x25" => Register::X25,
        "s9" => Register::X25,

        "x26" => Register::X26,
        "s10" => Register::X26,

        "x27" => Register::X27,
        "s11" => Register::X27,

        //---
        "x28" => Register::X28,
        "t3" => Register::X28,

        "x29" => Register::X29,
        "t4" => Register::X29,

        "x30" => Register::X30,
        "t5" => Register::X30,

        "x31" => Register::X31,
        "t6" => Register::X31,

        _ => panic!("Non existent register"),
    }
}
