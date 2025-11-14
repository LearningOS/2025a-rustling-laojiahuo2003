/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
/// The address must be properly aligned and point to a valid u32 in memory.
/// The caller must ensure that no other references exist to this memory location.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: 
    // - The caller guarantees that `address` is a valid pointer to a mutable u32
    // - The pointer is properly aligned for u32 (4-byte alignment)
    // - No other references exist to this memory, ensuring no data races
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}