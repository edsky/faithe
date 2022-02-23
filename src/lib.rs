#![warn(missing_docs)]
#![cfg_attr(feature = "no-std", no_std)]
//! # Faithe
//! Useful stuff for memory hacking in windows.

/// APIs for internal interation with current process.
#[cfg(not(feature = "no-std"))]
pub mod internal;
/// Useful memory APIs.
pub mod memory;
#[cfg(feature = "external")]
/// Module for dealing with processs' modules.
pub mod module;
#[cfg(feature = "external")]
/// Module for doing common things with processes.
pub mod process;
#[cfg(feature = "external")]
/// Iterator over threads and etc.
pub mod thread;

#[cfg(not(feature = "no-std"))]
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

pub use memoffset::offset_of;
pub use obfstr::wide;