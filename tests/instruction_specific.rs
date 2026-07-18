use pure_rv32i::{AssemblerError, assemble_string};

#[test]
fn add() -> Result<(), AssemblerError> {
    let result = assemble_string("add x1, x0, x2")?;
    let expected = vec![0xb3, 0x00, 0x20, 0x00];
    assert_eq!(result, expected);
    Ok(())
}
#[test]
fn addi() {
    let result = assemble_string("addi x1, x0, 100").unwrap();
    let expected = vec![0x93, 0x00, 0x40, 0x06];
    assert_eq!(result, expected);
}

#[test]
fn sub() {
    let result = assemble_string("sub x1, x10, x5").unwrap();
    let expected = vec![0xb3, 0x00, 0x55, 0x40];
    assert_eq!(result, expected);
}

#[test]
fn lw() {
    let result = assemble_string("lw x1, 34(x13)").unwrap();
    let expected = vec![0x83, 0xa0, 0x26, 0x02];
    assert_eq!(result, expected);
}

#[test]
fn lb() {
    let result = assemble_string("lb x1, 34(x13)").unwrap();
    let expected = vec![0x83, 0x80, 0x26, 0x02];
    assert_eq!(result, expected);
}

#[test]
fn sw() {
    let result = assemble_string("sw x1, 34(x13)").unwrap();
    let expected = vec![0x23, 0xa1, 0x16, 0x02];
    assert_eq!(result, expected);
}

#[test]
fn sb() {
    let result = assemble_string("sb x1, 34(x13)").unwrap();
    let expected = vec![0x23, 0x81, 0x16, 0x02];
    assert_eq!(result, expected);
}

#[test]
fn and() {
    let result = assemble_string("and x16, x31,x0").unwrap();
    let expected = vec![0x33, 0xf8, 0x0f, 0x00];
    assert_eq!(result, expected);
}

#[test]
fn or() {
    let result = assemble_string("or x16, x31,x0").unwrap();
    let expected = vec![0x33, 0xe8, 0x0f, 0x00];
    assert_eq!(result, expected);
}

#[test]
fn xor() {
    let result = assemble_string("xor x16, x31, x0").unwrap();
    let expected = vec![0x33, 0xc8, 0x0f, 0x00];
    assert_eq!(result, expected);
}

#[test]
fn xori() {
    let result = assemble_string("xori x16, x31, 2030").unwrap();
    let expected = vec![0x13, 0xc8, 0xef, 0x7e];
    assert_eq!(result, expected);
}

#[test]
fn ori() {
    let result = assemble_string("ori x16, x31,2030").unwrap();
    let expected = vec![0x13, 0xe8, 0xef, 0x7e];
    assert_eq!(result, expected);
}

#[test]
fn andi() {
    let result = assemble_string("andi x16, x31,2030").unwrap();
    let expected = vec![0x13, 0xf8, 0xef, 0x7e];
    assert_eq!(result, expected);
}

#[test]
fn slti() {
    let result = assemble_string("slti x16, x31,2030").unwrap();
    let expected = vec![0x13, 0xa8, 0xef, 0x7e];
    assert_eq!(result, expected);
}

#[test]
fn sltiu() {
    let result = assemble_string("sltiu x16, x31,2030").unwrap();
    let expected = vec![0x13, 0xb8, 0xef, 0x7e];
    assert_eq!(result, expected);
}

#[test]
fn slli() {
    let result = assemble_string("slli x16, x31,30").unwrap();
    let expected = vec![0x13, 0x98, 0xef, 0x01];
    assert_eq!(result, expected);
}

#[test]
fn srli() {
    let result = assemble_string("srli x16, x31,30").unwrap();
    let expected = vec![0x13, 0xd8, 0xef, 0x01];
    assert_eq!(result, expected);
}

#[test]
fn srai() {
    let result = assemble_string("srai x16, x31,30").unwrap();
    let expected = vec![0x13, 0xd8, 0xef, 0x41];
    assert_eq!(result, expected);
}
// ---- labels -----

#[test]
fn beq() {
    let result = assemble_string("beq x16, x31,START \n START:").unwrap();
    let expected = vec![0x63, 0x02, 0xf8, 0x01];
    assert_eq!(result, expected);
}
#[test]
fn bne() {
    let result = assemble_string("bne x16, x31,START \n START:").unwrap();
    let expected = vec![0x63, 0x12, 0xf8, 0x01];
    assert_eq!(result, expected);
}

#[test]
fn bge() {
    let result = assemble_string("bge x16, x31,START \n START:").unwrap();
    let expected = vec![0x63, 0x52, 0xf8, 0x01];
    assert_eq!(result, expected);
}

#[test]
fn blt() {
    let result = assemble_string("blt x16, x31,START \n START:").unwrap();
    let expected = vec![0x63, 0x42, 0xf8, 0x01];
    assert_eq!(result, expected);
}
#[test]
fn bltu() {
    let result = assemble_string("bltu x16, x31,START \n START:").unwrap();
    let expected = vec![0x63, 0x62, 0xf8, 0x01];
    assert_eq!(result, expected);
}
#[test]
fn bgeu() {
    let result = assemble_string("bgeu x16, x31,START \n START:").unwrap();
    let expected = vec![0x63, 0x72, 0xf8, 0x01];
    assert_eq!(result, expected);
}
#[test]
fn jal() {
    let result = assemble_string("jal x16,START \n START:").unwrap();
    let expected = vec![0x6f, 0x08, 0x40, 0x00];
    assert_eq!(result, expected);
}

#[test]
fn jalr() {
    let result_1 = assemble_string("jalr x16, 300(x31)").unwrap();
    let result_2 = assemble_string("jalr x16,x31,300").unwrap();
    let expected = vec![0x67, 0x88, 0xcf, 0x12];
    assert_eq!(result_1, expected);
    assert_eq!(result_2, expected);
}
#[test]
fn lui() {
    let result = assemble_string("lui x10,100").unwrap();
    let expected = vec![0x37, 0x45, 0x06, 0x00];
    assert_eq!(result, expected);
}

#[test]
fn auipc() {
    let result = assemble_string("auipc x10,100").unwrap();
    let expected = vec![0x17, 0x45, 0x06, 0x00];
    assert_eq!(result, expected);
}
