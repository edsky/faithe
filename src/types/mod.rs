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
    }
}

mod entry;
pub use entry::*;

/// Zero terminated ascii string. Have the same layout as `*const u8`.
/// Used for convenience so it can be returned from extern funcs.
#[derive(Debug, Clone)]
#[repr(transparent)]
#[cfg(feature = "alloc")]
pub struct StrPtr(core::ptr::NonNull<u8>);
#[cfg(feature = "alloc")]
impl StrPtr {
    /// Converts pointer into string slice.
    /// # Safety
    /// `StrPtr` must point to valid memory.
    #[inline]
    pub unsafe fn to_str<'a>(&self) -> &'a str {
        core::str::from_utf8_unchecked(crate::terminated_array(self.0.as_ptr(), &0))
    }

    /// Converts pointer to string by cloning data.
    /// # Safety
    /// `StrPtr` must point to valid memory.
    #[inline]
    pub unsafe fn into_string(self) -> alloc::string::String {
        self.to_str().into()
    }

    /// Creates new [`StrPtr`] from pointer without checking if it's null.
    /// # Safety
    /// `p` must not be `null`.
    #[inline(always)]
    pub const unsafe fn new_unchecked(p: *const u8) -> Self {
        Self(core::ptr::NonNull::new_unchecked(p as _))
    }

    /// Creates new [`StrPtr`] from pointer.
    #[inline(always)]
    pub fn new(p: *const u8) -> Option<Self> {
        Some(Self(core::ptr::NonNull::new(p as _)?))
    }

    /// Returns the pointer to the string.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Returns the mutable pointer to the string.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_ptr()
    }
}

/// Creates new StrPtr.
#[macro_export]
macro_rules! str_ptr {
    ($($s:expr),*) => {
        unsafe { $crate::types::StrPtr::new_unchecked(concat! { $($s),*, "\x00" }.as_bytes().as_ptr()) }
    };
}

/// Zero terminated UTF-16 string. Have the same layout as `*const u16`.
/// Used for convenience so it can be returned from extern funcs.
#[derive(Debug, Clone)]
#[repr(transparent)]
#[cfg(feature = "alloc")]
pub struct WidePtr(core::ptr::NonNull<u16>);
#[cfg(feature = "alloc")]
impl WidePtr {
    /// Converts pointer to string by cloning data.
    /// # Safety
    /// `WidePtr` must point to valid memory.
    #[inline]
    pub unsafe fn into_string(self) -> alloc::string::String {
        alloc::string::String::from_utf16_lossy(crate::terminated_array(self.0.as_ptr(), &0))
    }

    /// Creates new [`WidePtr`] from pointer without checking if it's null.
    /// # Safety
    /// `p` must not be `null`.
    #[inline(always)]
    pub const unsafe fn new_unchecked(p: *const u16) -> Self {
        Self(core::ptr::NonNull::new_unchecked(p as _))
    }

    /// Creates new [`WidePtr`] from pointer.
    #[inline(always)]
    pub fn new(p: *const u16) -> Option<Self> {
        Some(Self(core::ptr::NonNull::new(p as _)?))
    }

    /// Returns the pointer to the string.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }

    /// Returns the mutable pointer to the string.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut u16 {
        self.0.as_ptr()
    }
}
