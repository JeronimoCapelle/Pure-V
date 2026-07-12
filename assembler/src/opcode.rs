pub enum Opcode {
    ADDI(IType),
    ADD(RType),
    SUB(RType),
    BNE(BType),
    BEQ(BType),
    BLT(BType),
    BGE(BType),
    JAL(JType),
    JALR(IType),
    LW(ITypeMemory),
    SW(STypeMemory),
    LB(ITypeMemory),
    SB(STypeMemory),
    SLLI(ITypeShifts),
    SRLI(ITypeShifts),
    AND(RType),
    OR(RType),
    XOR(RType),
    ANDI(IType),
    ORI(IType),
    XORI(IType),
    NOP,
}

//--------------------------------------------------

pub struct RType(pub Register, pub Register, pub Register);
pub struct IType(Register, Register, Immediate);

pub struct ITypeShifts(Register, Register, Shamt);

pub struct ITypeMemory(Register, Offset, Register);

pub struct STypeMemory(Register, Offset, Register);

pub struct BType(Register, Register, Label);

pub struct JType(Register, BigLabel);

//--------------------------------------------------

pub struct Immediate(i16); // 12-bit signed integer (range: -2048 to 2047). Limit artificially

pub struct Offset(i16); //12-bit signed immediate offset (range: -2048 to 2047 bytes). Limit artificially

pub struct Shamt(u8); //5-bit unsigned integer (range: 0 to 31 for 32-bit registers). Limit artificially

pub struct Label(i16); //12-bit signed PC-relative offset. limit artificially.

pub struct BigLabel(i32); //20-bit signed PC-relative offset. Limit artificially

//--------------------------------------------------

pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}
