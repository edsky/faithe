#![warn(missing_docs)]
//! # Radon
//! Useful stuff for memory hacking in windows.

#[cfg(feature = "win32")]
/// APIs for internal interation with current process.
pub mod internal;
/// Useful memory APIs.
pub mod memory;
#[cfg(feature = "win32")]
/// Module for dealing with processs' modules.
pub mod module;
#[cfg(feature = "win32")]
/// Module for doing common things with processes.
pub mod process;
#[cfg(feature = "win32")]
/// Iterator over threads and etc.
pub mod thread;

/// Pattern searching.
pub mod pattern;

/// Re-exports of types used in windows.
pub mod types;

mod error;
pub use error::*;

mod macros;

/// Casts a pointer to an immutable reference for a convenient use.
/// # Safety
/// NO
#[inline(always)]
pub unsafe fn to_ref<'a, T>(ptr: *const T) -> &'a T {
    &*ptr
}

/// Casts a pointer to a mutable reference for a convenient use.
/// # Safety
/// NO
#[inline(always)]
pub unsafe fn to_mut_ref<'a, T>(ptr: *const T) -> &'a mut T {
    &mut *(ptr as *mut T)
}
