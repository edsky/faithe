mod info;
pub use info::*;

mod protection;
pub use protection::*;

use crate::internal::protect;

/// Resolves multilevel pointer.
/// # Behavior
/// It begins from adding to base first offset and reading a value on this address, assigns to
/// base readed value and so on.
/// # Safety
/// You need to make sure beforehand that all offsets will lead to valid memory addresses.
pub unsafe fn follow_pointer_path<const I: usize, T>(
    mut base: *const u8,
    offsets: [usize; I],
) -> *const T {
    for offset in &offsets {
        base = *((base as usize + *offset) as *const usize) as _;
    }
    base as _
}

/// Protects memory of given size with new protection, calls `callback` and then restores previous protection.
/// # Panics
/// * Can not protect memory.
/// * Can not restore previous protection.
/// * Previous protection can not be represented with [`MemoryProtection`].
pub fn guard<T>(address: *mut (), size: usize, protection: MemoryProtection, callback: impl FnOnce() -> T) -> T {
    let old = protect(address, size, protection).expect("Failed to protect memory.");
    let val = callback();
    protect(address, size, old).expect("Failed to restore previous protection");
    val
}
