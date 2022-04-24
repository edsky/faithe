extern crate alloc;
type String = alloc::string::String;

use core::mem::size_of;

/// Creates new unicode string.
#[macro_export]
macro_rules! unicode_string {
    ($str:expr) => {
        $crate::types::UnicodeString {
            len: ($str.len() * 2) as _,
            maximum_len: ($str.len() * 2) as _,
            buffer: $crate::wide!($str).as_ptr() as _,
        }
    };
}

/// UTF16 String.
#[repr(C)]
pub struct UnicodeString {
    /// Length of the string, in bytes.
    pub len: u16,
    /// Allocated size, in bytes.
    pub maximum_len: u16,
    /// Pointer to actual UTF-16 data.
    pub buffer: *mut u16,
}

impl UnicodeString {
    /// Converts wide string into [`alloc::string::String`] by cloning data.
    /// # Safety
    /// This function doesn't check if `self.buffer` == 0.
    #[inline]
    pub unsafe fn decode_utf16_unchecked(&self) -> String {
        let utf16 = core::slice::from_raw_parts(self.buffer, self.len as usize / size_of::<u16>());
        alloc::string::String::from_utf16_lossy(utf16)
    }

    /// Converts wide string into [`alloc::string::String`] by cloning data.
    /// # Safety
    /// This function only checks if `self.buffer` != 0 and if `self.len` != 0.
    #[inline]
    pub unsafe fn decode_utf16(&self) -> Option<String> {
        if self.is_null() {
            None
        } else {
            Some(self.decode_utf16_unchecked())
        }
    }

    /// Checks if either `self.len`, `self.maximum_len` or `self.buffer` == 0.
    /// If this function returns `true` it doesn't guarantee that `self.buffer` points to valid memory.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.len == 0 || self.maximum_len == 0 || self.buffer.is_null()
    }
}

impl PartialEq for UnicodeString {
    fn eq(&self, other: &Self) -> bool {
        if self.is_null() || other.is_null() || self.len != other.len {
            false
        } else {
            unsafe { libc::memcmp(self.buffer as _, other.buffer as _, self.len as _) == 0 }
        }
    }
}
