//! Bit manipulation utilities

/// Sets the `bit` of `x`.
const fn set(x: usize, bit: u32) -> usize {
    assert!(32 > bit);
    x | 1 << bit
}

/// Tests the `bit` of `x`.
const fn test(x: usize, bit: u32) -> bool {
    assert!(32 > bit);
    x & (1 << bit) != 0
}
