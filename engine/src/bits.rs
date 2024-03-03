//! Helper functions for working with bits.

/// Returns the count of bits set to one.
#[inline(always)]
pub fn count_ones(value: u64) -> u32 {
    value.count_ones()
}

/// Returns `true` if at most one bit is set to one.
#[inline(always)]
pub fn at_most_one(value: u64) -> bool {
    value & (value - 1) == 0
}

/// Returns `true` if exactly one bit is set to one and the rest are zeroes.
#[inline(always)]
pub fn only_one(value: u64) -> bool {
    value != 0 && at_most_one(value)
}

/// Returns `true` if two or more bits are set to one.
#[inline(always)]
pub fn more_than_one(value: u64) -> bool {
    !at_most_one(value)
}
