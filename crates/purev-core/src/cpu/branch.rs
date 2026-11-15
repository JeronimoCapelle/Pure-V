//! Pure stateless branch comparison functions.

use purev_isa::RegValue;

/// Branch if equal (`beq`).
#[inline]
pub const fn eq(lhs: RegValue, rhs: RegValue) -> bool {
    lhs == rhs
}

/// Branch if not equal (`bne`).
#[inline]
pub const fn ne(lhs: RegValue, rhs: RegValue) -> bool {
    lhs != rhs
}

/// Branch if less than, signed (`blt`).
#[inline]
pub const fn lt(lhs: RegValue, rhs: RegValue) -> bool {
    lhs.cast_signed() < rhs.cast_signed()
}

/// Branch if greater than or equal, signed (`bge`).
#[inline]
pub const fn ge(lhs: RegValue, rhs: RegValue) -> bool {
    lhs.cast_signed() >= rhs.cast_signed()
}

/// Branch if less than, unsigned (`bltu`).
#[inline]
pub const fn ltu(lhs: RegValue, rhs: RegValue) -> bool {
    lhs < rhs
}

/// Branch if greater than or equal, unsigned (`bgeu`).
#[inline]
pub const fn geu(lhs: RegValue, rhs: RegValue) -> bool {
    lhs >= rhs
}
