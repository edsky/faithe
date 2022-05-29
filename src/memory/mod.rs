extern crate alloc;

mod info;
pub use info::*;

mod protection;
pub use protection::*;

use crate::{terminated_array, FaitheError};

/// Resolves multilevel pointer.
/// # Behavior
/// It begins from adding to base first offset and reading a value on this address, assigns to
/// base readed value and so on.
/// # Safety
/// You need to make sure beforehand that all offsets will lead to valid memory addresses.
#[inline]
pub unsafe fn follow_pointer_path<const I: usize, T>(
    mut base: *const u8,
    offsets: [usize; I],
) -> *const T {
    for offset in &offsets {
        base = *((base as usize + *offset) as *const usize) as _;
    }
    base as _
}

/// Reads zero terminated string at `ptr`.
#[inline]
pub unsafe fn read_string<'a>(ptr: *const i8) -> crate::Result<&'a str> {
    core::str::from_utf8(terminated_array(ptr as *const u8, &0))
        .map_err(|_| FaitheError::InvalidString)
}

/// Reads zero terminated string at `ptr`.
#[inline]
pub unsafe fn read_string_unchecked<'a>(ptr: *const i8) -> &'a str {
    read_string(ptr).unwrap()
}

/// Reads zero terminated string at `ptr`.
#[inline]
pub unsafe fn read_wide_string<'a>(ptr: *const u16) -> crate::Result<alloc::string::String> {
    alloc::string::String::from_utf16(terminated_array(ptr, &0))
        .map_err(|_| FaitheError::InvalidString)
}

/// Reads zero terminated string at `ptr`.
#[inline]
pub unsafe fn read_wide_string_unchecked<'a>(ptr: *const u16) -> alloc::string::String {
    read_wide_string(ptr).unwrap()
}

/// Protects memory of given size with new protection, calls `callback` and then restores previous protection.
/// # Panics
/// * Can not protect memory.
/// * Can not restore previous protection.
/// * Previous protection can not be represented with [`MemoryProtection`].
#[inline]
#[cfg(not(feature = "no-std"))]
pub fn guard<T>(
    address: *mut (),
    size: usize,
    protection: MemoryProtection,
    callback: impl FnOnce() -> T,
) -> T {
    let old = crate::__expect!(
        crate::internal::protect(address, size, protection),
        "Failed to protect memory."
    );
    let val = callback();
    crate::__expect!(
        crate::internal::protect(address, size, old),
        "Failed to restore previous protection"
    );
    val
}
