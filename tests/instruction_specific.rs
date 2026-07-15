use pure_rv32i::compile_string;

#[test]
fn add() {
    let result = compile_string("add x1, x0, x2").unwrap();
    let expected = vec![0xb3, 0x00, 0x20, 0x00];
    assert_eq!(result, expected);
}
#[test]
fn addi() {
    let result = compile_string("addi x1, x0, 100").unwrap();
    let expected = vec![0x93, 0x00, 0x40, 0x06];
    assert_eq!(result, expected);
}

#[test]
fn sub() {
    let result = compile_string("sub x1, x10, x5").unwrap();
    let expected = vec![0xb3, 0x00, 0x55, 0x40];
    assert_eq!(result, expected);
}

#[test]
fn lw() {
    let result = compile_string("lw x1, 34(x13)").unwrap();
    let expected = vec![0x83, 0xa0, 0x26, 0x02];
    assert_eq!(result, expected);
}

#[test]
fn lb() {
    let result = compile_string("lb x1, 34(x13)").unwrap();
    let expected = vec![0x83, 0x80, 0x26, 0x02];
    assert_eq!(result, expected);
}

#[test]
fn sw() {
    let result = compile_string("sw x1, 34(x13)").unwrap();
    let expected = vec![0x23, 0xa1, 0x16, 0x02];
    assert_eq!(result, expected);
}

#[test]
fn sb() {
    let result = compile_string("sb x1, 34(x13)").unwrap();
    let expected = vec![0x23, 0x81, 0x16, 0x02];
    assert_eq!(result, expected);
}

#[test]
fn and() {
    let result = compile_string("and x16, x31,x0").unwrap();
    let expected = vec![0x33, 0xf8, 0x0f, 0x00];
    assert_eq!(result, expected);
}

#[test]
fn or() {
    let result = compile_string("or x16, x31,x0").unwrap();
    let expected = vec![0x33, 0xe8, 0x0f, 0x00];
    assert_eq!(result, expected);
}

#[test]
fn xor() {
    let result = compile_string("xor x16, x31, x0").unwrap();
    let expected = vec![0x33, 0xc8, 0x0f, 0x00];
    assert_eq!(result, expected);
}

#[test]
fn xori() {
    let result = compile_string("xori x16, x31, 2030").unwrap();
    let expected = vec![0x13, 0xc8, 0xef, 0x7e];
    assert_eq!(result, expected);
}

#[test]
fn ori() {
    let result = compile_string("ori x16, x31,2030").unwrap();
    let expected = vec![0x13, 0xe8, 0xef, 0x7e];
    assert_eq!(result, expected);
}

#[test]
fn andi() {
    let result = compile_string("andi x16, x31,2030").unwrap();
    let expected = vec![0x13, 0xf8, 0xef, 0x7e];
    assert_eq!(result, expected);
}

#[test]
fn slli() {
    let result = compile_string("slli x16, x31,30").unwrap();
    let expected = vec![0x13, 0x98, 0xef, 0x01];
    assert_eq!(result, expected);
}

#[test]
fn srli() {
    let result = compile_string("srli x16, x31,30").unwrap();
    let expected = vec![0x13, 0xd8, 0xef, 0x01];
    assert_eq!(result, expected);
}

// ---- labels -----

#[test]
fn beq() {
    let result = compile_string("beq x16, x31,START \n START:").unwrap();
    let expected = vec![0x63, 0x02, 0xf8, 0x01];
    assert_eq!(result, expected);
}
#[test]
fn bne() {
    let result = compile_string("bne x16, x31,START \n START:").unwrap();
    let expected = vec![0x63, 0x12, 0xf8, 0x01];
    assert_eq!(result, expected);
}

#[test]
fn bge() {
    let result = compile_string("bge x16, x31,START \n START:").unwrap();
    let expected = vec![0x63, 0x52, 0xf8, 0x01];
    assert_eq!(result, expected);
}

#[test]
fn blt() {
    let result = compile_string("blt x16, x31,START \n START:").unwrap();
    let expected = vec![0x63, 0x42, 0xf8, 0x01];
    assert_eq!(result, expected);
}

#[test]
fn jal() {
    let result = compile_string("jal x16,START \n START:").unwrap();
    let expected = vec![0x6f, 0x08, 0x40, 0x00];
    assert_eq!(result, expected);
}

#[test]
fn jalr() {
    let result_1 = compile_string("jalr x16, 300(x31)").unwrap();
    let result_2 = compile_string("jalr x16,x31,300").unwrap();
    let expected = vec![0x67, 0x88, 0xcf, 0x12];
    assert_eq!(result_1, expected);
    assert_eq!(result_2, expected);
}
