//! Integration tests for `purev-asm` ported from `Pure-V` and extended for
//! RV64I.

use purev_asm::{AsmError, ParseError, assemble, assemble_bytes};

#[test]
fn instruction_add() -> Result<(), AsmError> {
    let result = assemble_bytes("add x1, x0, x2")?;
    assert_eq!(result, vec![0xb3, 0x00, 0x20, 0x00]);
    Ok(())
}

#[test]
fn instruction_addi() {
    let result = assemble_bytes("addi x1, x0, 100").unwrap();
    assert_eq!(result, vec![0x93, 0x00, 0x40, 0x06]);
}

#[test]
fn instruction_sub() {
    let result = assemble_bytes("sub x1, x10, x5").unwrap();
    assert_eq!(result, vec![0xb3, 0x00, 0x55, 0x40]);
}

#[test]
fn instruction_shifts_and_comparisons() {
    assert_eq!(
        assemble_bytes("sll x1, x10, x5").unwrap(),
        vec![0xb3, 0x10, 0x55, 0x00]
    );
    assert_eq!(
        assemble_bytes("srl x1, x10, x5").unwrap(),
        vec![0xb3, 0x50, 0x55, 0x00]
    );
    assert_eq!(
        assemble_bytes("sra x1, x10, x5").unwrap(),
        vec![0xb3, 0x50, 0x55, 0x40]
    );
    assert_eq!(
        assemble_bytes("slt x1, x10, x5").unwrap(),
        vec![0xb3, 0x20, 0x55, 0x00]
    );
    assert_eq!(
        assemble_bytes("sltu x1, x10, x5").unwrap(),
        vec![0xb3, 0x30, 0x55, 0x00]
    );
}

#[test]
fn instruction_loads_and_stores() {
    assert_eq!(
        assemble_bytes("lw x1, 34(x13)").unwrap(),
        vec![0x83, 0xa0, 0x26, 0x02]
    );
    assert_eq!(
        assemble_bytes("lh x1, 34(x13)").unwrap(),
        vec![0x83, 0x90, 0x26, 0x02]
    );
    assert_eq!(
        assemble_bytes("lb x1, 34(x13)").unwrap(),
        vec![0x83, 0x80, 0x26, 0x02]
    );
    assert_eq!(
        assemble_bytes("lhu x1, 34(x13)").unwrap(),
        vec![0x83, 0xd0, 0x26, 0x02]
    );
    assert_eq!(
        assemble_bytes("lbu x1, 34(x13)").unwrap(),
        vec![0x83, 0xc0, 0x26, 0x02]
    );
    assert_eq!(
        assemble_bytes("sw x1, 34(x13)").unwrap(),
        vec![0x23, 0xa1, 0x16, 0x02]
    );
    assert_eq!(
        assemble_bytes("sh x1, 34(x13)").unwrap(),
        vec![0x23, 0x91, 0x16, 0x02]
    );
    assert_eq!(
        assemble_bytes("sb x1, 34(x13)").unwrap(),
        vec![0x23, 0x81, 0x16, 0x02]
    );
}

#[test]
fn instruction_logic_and_immediates() {
    assert_eq!(
        assemble_bytes("and x16, x31,x0").unwrap(),
        vec![0x33, 0xf8, 0x0f, 0x00]
    );
    assert_eq!(
        assemble_bytes("or x16, x31,x0").unwrap(),
        vec![0x33, 0xe8, 0x0f, 0x00]
    );
    assert_eq!(
        assemble_bytes("xor x16, x31, x0").unwrap(),
        vec![0x33, 0xc8, 0x0f, 0x00]
    );
    assert_eq!(
        assemble_bytes("xori x16, x31, 2030").unwrap(),
        vec![0x13, 0xc8, 0xef, 0x7e]
    );
    assert_eq!(
        assemble_bytes("ori x16, x31,2030").unwrap(),
        vec![0x13, 0xe8, 0xef, 0x7e]
    );
    assert_eq!(
        assemble_bytes("andi x16, x31,2030").unwrap(),
        vec![0x13, 0xf8, 0xef, 0x7e]
    );
    assert_eq!(
        assemble_bytes("slti x16, x31,2030").unwrap(),
        vec![0x13, 0xa8, 0xef, 0x7e]
    );
    assert_eq!(
        assemble_bytes("sltiu x16, x31,2030").unwrap(),
        vec![0x13, 0xb8, 0xef, 0x7e]
    );
    assert_eq!(
        assemble_bytes("slli x16, x31,30").unwrap(),
        vec![0x13, 0x98, 0xef, 0x01]
    );
    assert_eq!(
        assemble_bytes("srli x16, x31,30").unwrap(),
        vec![0x13, 0xd8, 0xef, 0x01]
    );
    assert_eq!(
        assemble_bytes("srai x16, x31,30").unwrap(),
        vec![0x13, 0xd8, 0xef, 0x41]
    );
}

#[test]
fn instruction_branches_and_jumps_with_labels() {
    assert_eq!(
        assemble_bytes("beq x16, x31,START \n START:").unwrap(),
        vec![0x63, 0x02, 0xf8, 0x01]
    );
    assert_eq!(
        assemble_bytes("bne x16, x31,START \n START:").unwrap(),
        vec![0x63, 0x12, 0xf8, 0x01]
    );
    assert_eq!(
        assemble_bytes("bge x16, x31,START \n START:").unwrap(),
        vec![0x63, 0x52, 0xf8, 0x01]
    );
    assert_eq!(
        assemble_bytes("blt x16, x31,START \n START:").unwrap(),
        vec![0x63, 0x42, 0xf8, 0x01]
    );
    assert_eq!(
        assemble_bytes("bltu x16, x31,START \n START:").unwrap(),
        vec![0x63, 0x62, 0xf8, 0x01]
    );
    assert_eq!(
        assemble_bytes("bgeu x16, x31,START \n START:").unwrap(),
        vec![0x63, 0x72, 0xf8, 0x01]
    );
    assert_eq!(
        assemble_bytes("jal x16,START \n START:").unwrap(),
        vec![0x6f, 0x08, 0x40, 0x00]
    );

    let jalr_mem = assemble_bytes("jalr x16, 300(x31)").unwrap();
    let jalr_reg = assemble_bytes("jalr x16,x31,300").unwrap();
    assert_eq!(jalr_mem, vec![0x67, 0x88, 0xcf, 0x12]);
    assert_eq!(jalr_reg, vec![0x67, 0x88, 0xcf, 0x12]);
}

#[test]
fn instruction_upper_fence_and_system() {
    assert_eq!(
        assemble_bytes("lui x10,100").unwrap(),
        vec![0x37, 0x45, 0x06, 0x00]
    );
    assert_eq!(
        assemble_bytes("auipc x10,100").unwrap(),
        vec![0x17, 0x45, 0x06, 0x00]
    );
    assert_eq!(
        assemble_bytes("fence (iw, rw)").unwrap(),
        vec![0x0f, 0x00, 0x30, 0x09]
    );
    assert_eq!(
        assemble_bytes("ebreak").unwrap(),
        vec![0x73, 0x00, 0x10, 0x00]
    );
    assert_eq!(
        assemble_bytes("ecall").unwrap(),
        vec![0x73, 0x00, 0x00, 0x00]
    );
}

#[test]
fn program_core_execution_loop() {
    let assembly = "
    _start:
        addi x1, x0, 15
        slli x2, x1, 2
        sw x2, 0(x0)
        jal x3, _start
    ";

    let expected_binary: Vec<u8> = vec![
        0x93, 0x00, 0xF0, 0x00, // addi x1, x0, 15
        0x13, 0x91, 0x20, 0x00, // slli x2, x1, 2
        0x23, 0x20, 0x20, 0x00, // sw x2, 0(x0)
        0xEF, 0xF1, 0x5F, 0xFF, // jal x3, _start (-12 bytes offset)
    ];

    assert_eq!(assemble_bytes(assembly).unwrap(), expected_binary);
}

#[test]
fn program_stress_test() {
    let source_code = "
        _start:
            addi x1, x0, -2048
            ori x2, x1, 255
            xori x3, x2, -1
            add x4, x1, x2
            sub x5, x4, x3
            slli x6, x5, 31
            srli x7, x6, 15

        _memory:
            sw x7, 2044(x0)
            sb x6, -2048(x1)
            lw x8, 2044(x0)
            lb x9, -2048(x1)

        _branches:
            beq x8, x9, _start //fails
            bne x9, x8, _forward
            blt x1, x2, _start
            bge x2, x1, _forward

        _forward:
            jal x10, _end

        _end:
            jalr x0, 0(x10)";

    let binary: Vec<u8> = vec![
        0x93, 0x00, 0x00, 0x80, // addi x1, x0, -2048
        0x13, 0xe1, 0xf0, 0x0f, // ori x2, x1, 255
        0x93, 0x41, 0xf1, 0xff, // xori x3, x2, -1
        0x33, 0x82, 0x20, 0x00, // add x4, x1, x2
        0xb3, 0x02, 0x32, 0x40, // sub x5, x4, x3
        0x13, 0x93, 0xf2, 0x01, // slli x6, x5, 31
        0x93, 0x53, 0xf3, 0x00, // srli x7, x6, 15
        0x23, 0x2e, 0x70, 0x7e, // sw x7, 2044(x0)
        0x23, 0x80, 0x60, 0x80, // sb x6, -2048(x1)
        0x03, 0x24, 0xc0, 0x7f, // lw x8, 2044(x0)
        0x83, 0x84, 0x00, 0x80, // lb x9, -2048(x1)
        0xe3, 0x0a, 0x94, 0xfc, // beq x8, x9, _start
        0x63, 0x96, 0x84, 0x00, // bne x9, x8, _forward
        0xe3, 0xc6, 0x20, 0xfc, // blt x1, x2, _start
        0x63, 0x52, 0x11, 0x00, // bge x2, x1, _forward
        0x6f, 0x05, 0x40, 0x00, // jal x10, _end
        0x67, 0x00, 0x05, 0x00, // jalr x0, 0(x10)
    ];

    assert_eq!(assemble_bytes(source_code).unwrap(), binary);
}

#[test]
fn rv64i_extensions_and_abi_aliases() {
    let words = assemble(
        "
        ld a0, 8(sp)
        lwu a1, -4(fp)
        sd a0, (sp)
        addiw a2, a0, -1
        slli a3, a0, 40
        slliw a4, a0, 31
        addw a5, a1, a2
        subw a6, a5, a4
        ",
    )
    .unwrap();
    assert_eq!(words.len(), 8);
    assert_eq!(words[0], 0x0081_3503); // ld x10, 8(x2)
    assert_eq!(words[1], 0xFFC4_6583); // lwu x11, -4(x8)
    assert_eq!(words[2], 0x00A1_3023); // sd x10, 0(x2)
    assert_eq!(words[3], 0xFFF5_061B); // addiw x12, x10, -1
    assert_eq!(words[4], 0x0285_1693); // slli x13, x10, 40
    assert_eq!(words[5], 0x01F5_171B); // slliw x14, x10, 31
}

#[test]
fn rejects_duplicate_and_undefined_labels() {
    let dup = assemble("dup:\ndup:\nnop");
    assert!(matches!(
        dup,
        Err(AsmError::Parse(ParseError::DuplicateLabel(_)))
    ));

    let undef = assemble("beq x1, x2, missing");
    assert!(matches!(
        undef,
        Err(AsmError::Parse(ParseError::UndefinedLabel(_)))
    ));
}
