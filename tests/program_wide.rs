use pure_rv32i::compile_string;

#[test]
fn test_core_execution_loop() {
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

    let result = compile_string(assembly).expect("Compilation failed");

    assert_eq!(result, expected_binary);
}

#[test]
fn stress_test() {
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

    let result = compile_string(source_code).expect("Compilation failed");

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
        0xe3, 0x0a, 0x94, 0xfc, // beq x8, x9, _start //broken
        0x63, 0x96, 0x84, 0x00, // bne x9, x8, _forward
        0xe3, 0xc6, 0x20, 0xfc, // blt x1, x2, _start
        0x63, 0x52, 0x11, 0x00, // bge x2, x1, _forward
        0x6f, 0x05, 0x40, 0x00, // jal x10, _end
        0x67, 0x00, 0x05, 0x00, // jalr x0, 0(x10)
    ];
    assert_eq!(result, binary);
}
