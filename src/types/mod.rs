cfg_if::cfg_if! {
    if #[cfg(not(feature = "no-std"))] {
        mod winapi;
        pub use winapi::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(any(not(feature = "no-std"), feature = "alloc"))] {
        mod string;
        pub use string::UnicodeString;

        extern crate alloc;
        use alloc::string::String;
    }
}

mod entry;
pub use entry::*;

use core::ptr::NonNull;

/// Zero terminated ascii string. Have the same layout as `*const u8`.
/// Used for convenience so it can be returned from extern funcs.
#[repr(transparent)]
pub struct StrPtr(NonNull<u8>);

impl StrPtr {
    /// Converts pointer into string slice.
    /// # Safety
    /// `StrPtr` must point to valid memory.
    #[inline]
    pub unsafe fn to_str<'a>(&self) -> &'a str {
        std::str::from_utf8_unchecked(crate::terminated_array(self.0.as_ptr(), &0))
    }

    /// Converts pointer to string by cloning data.
    /// # Safety
    /// `StrPtr` must point to valid memory.
    #[inline]
    pub unsafe fn into_string(self) -> String {
        self.to_str().into()
    }

    /// Creates new [`StrPtr`] from pointer without checking if it's null.
    /// # Safety
    /// `p` must not be `null`.
    #[inline(always)]
    pub const unsafe fn new_unchecked(p: *const u8) -> Self {
        Self(NonNull::new_unchecked(p as _))
    }

    /// Creates new [`StrPtr`] from pointer.
    #[inline(always)]
    pub fn new(p: *const u8) -> Option<Self> {
        Some(Self(NonNull::new(p as _)?))
    }
}

/// Creates new StrPtr.
#[macro_export]
macro_rules! str_ptr {
    ($($s:expr),*) => {
        $crate::types::StrPtr::new(concat! { $($s),*, "\x00" }.as_bytes().as_ptr())
    };
}

/// Zero terminated UTF-16 string. Have the same layout as `*const u16`.
/// Used for convenience so it can be returned from extern funcs.
#[repr(transparent)]
pub struct WidePtr(NonNull<u16>);
impl WidePtr {
    /// Converts pointer to string by cloning data.
    /// # Safety
    /// `WidePtr` must point to valid memory.
    #[inline]
    pub unsafe fn into_string(self) -> String {
        String::from_utf16_lossy(crate::terminated_array(self.0.as_ptr(), &0))
    }

    /// Creates new [`WidePtr`] from pointer without checking if it's null.
    /// # Safety
    /// `p` must not be `null`.
    #[inline(always)]
    pub const unsafe fn new_unchecked(p: *const u16) -> Self {
        Self(NonNull::new_unchecked(p as _))
    }

    /// Creates new [`WidePtr`] from pointer.
    #[inline(always)]
    pub fn new(p: *const u16) -> Option<Self> {
        Some(Self(NonNull::new(p as _)?))
    }
}
