#![cfg_attr(feature = "no-std", no_std)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// APIs for internal interation with current process.
#[cfg(not(feature = "no-std"))]
pub mod internal;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "external", not(feature = "no-std")))] {
        /// Iterator over threads and etc.
        pub mod thread;
        /// Module for doing common things with processes.
        pub mod process;
        /// Module for dealing with processs' modules.
        pub mod module;
    }
}

#[cfg(any(not(feature = "no-std"), feature = "alloc"))]
/// Pattern searching.
pub mod pattern;

/// Re-exports of types used in windows.
pub mod types;

mod error;
pub use error::*;

mod macros;
pub use macros::*;

/// Casts a pointer to an immutable reference.
/// # Safety
/// NO
#[inline(always)]
pub unsafe fn to_ref<'a, T>(ptr: *const T) -> &'a T {
    &*ptr
}

/// Casts a pointer to a mutable reference.
/// # Safety
/// NO
#[inline(always)]
pub unsafe fn to_mut_ref<'a, T>(ptr: *const T) -> &'a mut T {
    &mut *(ptr as *mut T)
}

/// Creates an immutable slice from the terminated array by finding it's last element. Returns a slice **NOT INCLUDING** the last element.
/// Maximum length is `usize::MAX`
/// # Safety
/// `ptr` must be valid, properly alligned.
/// ```
/// # use faithe::terminated_array;
/// let arr: [u8; 4] = [1, 2, 3, 0];
/// let terminated = unsafe { terminated_array(arr.as_ptr(), &0) };
/// assert_eq!(terminated, &[1, 2, 3]);
/// ```
#[inline]
pub unsafe fn terminated_array<'a, T: PartialEq>(mut ptr: *const T, last: T) -> &'a [T] {
    let mut len = 0;
    while &*ptr != &last {
        ptr = ptr.add(1);
        len += 1;
    }
    core::slice::from_raw_parts(ptr.sub(len), len)
}

/// Creates a mutable slice from the terminated array by finding it's last element. Returns a slice **NOT INCLUDING** the last element.
/// Maximum length is `usize::MAX`
/// # Safety
/// `ptr` must be valid, properly alligned.
/// ```
/// # use faithe::terminated_array_mut;
/// let mut arr: [u8; 4] = [1, 2, 3, 0];
/// let terminated = unsafe { terminated_array_mut(arr.as_mut_ptr(), &0) };
/// assert_eq!(terminated, &[1, 2, 3]);
/// terminated[1] = 5;
/// assert_eq!(arr, [1, 5, 3, 0]);
/// ```
#[inline]
pub unsafe fn terminated_array_mut<'a, T: PartialEq>(mut ptr: *mut T, last: T) -> &'a mut [T] {
    let mut len = 0;
    while &*ptr != &last || len >= usize::MAX {
        ptr = ptr.add(1);
        len += 1;
    }
    core::slice::from_raw_parts_mut(ptr.sub(len), len)
}

/// Creates an immutable slice from the terminated slice by finding it's last element. Returns a slice **NOT INCLUDING** the last element.
/// Returns an empty slice if failed to find the last element.
/// # Safety
/// `ptr` must be valid, properly alligned.
/// ```
/// # use faithe::terminated_slice;
/// let arr: [u8; 4] = [1, 2, 3, 0];
/// let terminated = terminated_slice(&arr, &0);
/// assert_eq!(terminated, &[1, 2, 3]);
/// ```
pub fn terminated_slice<'a, T: PartialEq>(slice: &'a [T], last: T) -> &'a [T] {
    if let Some(i) = slice.iter().position(|i| i == &last) {
        &slice[..i]
    } else {
        &[]
    }
}

/// Creates a mutable slice from the terminated slice by finding it's last element. Returns a slice **NOT INCLUDING** the last element.
/// Returns an empty slice if failed to find the last element.
/// # Safety
/// `ptr` must be valid, properly alligned.
/// ```
/// # use faithe::terminated_slice_mut;
/// let mut arr: [u8; 4] = [1, 2, 3, 0];
/// let terminated = terminated_slice_mut(&mut arr, &0);
/// assert_eq!(terminated, &[1, 2, 3]);
/// terminated[1] = 4;
/// assert_eq!(terminated, &[1, 4, 3]);
/// ```
pub fn terminated_slice_mut<'a, T: PartialEq>(slice: &'a mut [T], last: T) -> &'a mut [T] {
    if let Some(i) = slice.iter().position(|i| i == &last) {
        &mut slice[..i]
    } else {
        &mut []
    }
}

pub use memoffset::offset_of;
pub use obfstr::wide;
