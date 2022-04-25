use crate::{pattern::Pattern, FaitheError};
use std::cell::UnsafeCell;

mod global;
mod interface;
mod sizeof;
mod strings;
mod xstruct;

cfg_if::cfg_if! {
    if #[cfg(not(feature = "no-std"))] {
        mod function;
        pub use function::*;
    }
}

/// Macro for interal use. Provides functionality to hide panic messages if needed.
#[doc(hidden)]
#[macro_export]
macro_rules! __expect {
    ($var:expr, $msg:expr) => {
        if cfg!(feature = "no-msgs") {
            $var.unwrap()
        } else {
            $var.expect($msg)
        }
    };
}

enum InnerOffset {
    Explicit(usize),
    Pattern(&'static str),
    Resolved(usize),
}

#[doc(hidden)]
pub struct RuntimeOffset(UnsafeCell<InnerOffset>);
impl RuntimeOffset {
    #[inline(always)]
    pub fn address(&self) -> usize {
        unsafe {
            match *(self.0.get()) {
                InnerOffset::Resolved(address) => address,
                _ => unreachable!(),
            }
        }
    }

    #[inline(always)]
    pub fn is_resolved(&self) -> bool {
        unsafe { matches!(*(self.0.get()), InnerOffset::Resolved(_)) }
    }

    #[inline]
    pub fn try_resolve(&self, module: &'static str, add: usize) -> crate::Result<()> {
        unsafe {
            match *(self.0.get()) {
                InnerOffset::Explicit(offset) => {
                    let base = crate::internal::get_module_address(module)? as usize;
                    *self.0.get() = InnerOffset::Resolved(base + offset + add);
                    Ok(())
                }
                InnerOffset::Pattern(pattern) => {
                    let addr =
                        crate::internal::find_pattern(module, Pattern::from_ida_style(pattern))?
                            .ok_or(FaitheError::PatternNotFound)?
                            .as_ptr() as usize + add;
                    *self.0.get() = InnerOffset::Resolved(addr);
                    Ok(())
                }
                InnerOffset::Resolved(_) => Err(FaitheError::AlreadyResolved),
            }
        }
    }

    pub const fn explicit(offset: usize) -> Self {
        Self(UnsafeCell::new(InnerOffset::Explicit(offset)))
    }

    pub const fn pattern(pat: &'static str) -> Self {
        Self(UnsafeCell::new(InnerOffset::Pattern(pat)))
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_offset {
    (# $var:tt) => {
        $crate::RuntimeOffset::explicit($var)
    };
    (@ $var:tt) => {
        $crate::RuntimeOffset::pattern($var)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_offset2 {
    () => {
        0
    };
    ($val:tt) => {
        $val
    };
}
