//! Pure stateless 64-bit and 32-bit `*W` ALU helper functions.

use purev_isa::RegValue;

/// Sign-extends the lower 32 bits of a value to 64 bits.
#[inline]
pub fn sign_extend_word(val: u32) -> RegValue {
    i64::from(val.cast_signed()).cast_unsigned()
}

/// Truncates a 64-bit register value to its lower 32 bits.
#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "RV64 *W instructions explicitly operate on the lower 32 bits"
)]
pub const fn low_word(val: RegValue) -> u32 {
    val as u32
}

/// 64-bit wrapping addition.
#[inline]
pub const fn add64(lhs: RegValue, rhs: RegValue) -> RegValue {
    lhs.wrapping_add(rhs)
}

/// 64-bit wrapping addition with a signed immediate.
#[inline]
pub const fn addi64(lhs: RegValue, imm: i64) -> RegValue {
    lhs.wrapping_add(imm.cast_unsigned())
}

/// 64-bit wrapping subtraction.
#[inline]
pub const fn sub64(lhs: RegValue, rhs: RegValue) -> RegValue {
    lhs.wrapping_sub(rhs)
}

/// 64-bit logical left shift (masks shift amount to 6 bits).
#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "shift amount is masked with 0x3F (0..64)"
)]
pub const fn sll64(lhs: RegValue, shamt: RegValue) -> RegValue {
    lhs.wrapping_shl((shamt & 0x3F) as u32)
}

/// 64-bit logical right shift (masks shift amount to 6 bits).
#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "shift amount is masked with 0x3F (0..64)"
)]
pub const fn srl64(lhs: RegValue, shamt: RegValue) -> RegValue {
    lhs.wrapping_shr((shamt & 0x3F) as u32)
}

/// 64-bit arithmetic right shift (masks shift amount to 6 bits).
#[inline]
#[allow(
    clippy::cast_possible_truncation,
    reason = "shift amount is masked with 0x3F (0..64)"
)]
pub const fn sra64(lhs: RegValue, shamt: RegValue) -> RegValue {
    lhs.cast_signed()
        .wrapping_shr((shamt & 0x3F) as u32)
        .cast_unsigned()
}

/// Signed less-than comparison (`slt` / `slti`), returning `1` or `0`.
#[inline]
pub const fn slt64(lhs: RegValue, rhs: RegValue) -> RegValue {
    if lhs.cast_signed() < rhs.cast_signed() {
        1
    } else {
        0
    }
}

/// Unsigned less-than comparison (`sltu` / `sltiu`), returning `1` or `0`.
#[inline]
pub const fn sltu64(lhs: RegValue, rhs: RegValue) -> RegValue {
    if lhs < rhs { 1 } else { 0 }
}

/// 32-bit wrapping addition sign-extended to 64 bits (`addw` / `addiw`).
#[inline]
pub fn addw(lhs: RegValue, rhs: RegValue) -> RegValue {
    sign_extend_word(low_word(lhs).wrapping_add(low_word(rhs)))
}

/// 32-bit wrapping subtraction sign-extended to 64 bits (`subw`).
#[inline]
pub fn subw(lhs: RegValue, rhs: RegValue) -> RegValue {
    sign_extend_word(low_word(lhs).wrapping_sub(low_word(rhs)))
}

/// 32-bit logical left shift sign-extended to 64 bits (`sllw` / `slliw`, 5-bit
/// `shamt`).
#[inline]
pub fn sllw(lhs: RegValue, shamt: RegValue) -> RegValue {
    let amt = low_word(shamt) & 0x1F;
    sign_extend_word(low_word(lhs).wrapping_shl(amt))
}

/// 32-bit logical right shift sign-extended to 64 bits (`srlw` / `srliw`, 5-bit
/// `shamt`).
#[inline]
pub fn srlw(lhs: RegValue, shamt: RegValue) -> RegValue {
    let amt = low_word(shamt) & 0x1F;
    sign_extend_word(low_word(lhs).wrapping_shr(amt))
}

/// 32-bit arithmetic right shift sign-extended to 64 bits (`sraw` / `sraiw`,
/// 5-bit `shamt`).
#[inline]
pub fn sraw(lhs: RegValue, shamt: RegValue) -> RegValue {
    let amt = low_word(shamt) & 0x1F;
    sign_extend_word(
        low_word(lhs)
            .cast_signed()
            .wrapping_shr(amt)
            .cast_unsigned(),
    )
}
