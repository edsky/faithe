use crate::{pattern::Pattern, FaitheError};
use iced_x86::{Decoder, DecoderOptions, Mnemonic, OpKind};
use std::cell::UnsafeCell;

mod global;
mod interface;
mod sizeof;
mod strings;
mod vmt;

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
    #[cfg(not(feature = "no-std"))]
    Smart(&'static str),
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
                InnerOffset::Pattern(pat) => {
                    let addr = crate::internal::find_pattern(module, Pattern::from_ida_style(pat))?
                        .ok_or(FaitheError::PatternNotFound)?
                        .as_ptr() as usize
                        + add;
                    *self.0.get() = InnerOffset::Resolved(addr);
                    Ok(())
                }
                #[cfg(not(feature = "no-std"))]
                InnerOffset::Smart(pat) => {
                    let addr = crate::internal::find_pattern(module, Pattern::from_ida_style(pat))?
                        .ok_or(FaitheError::PatternNotFound)?
                        .as_ptr() as usize
                        + add;
                    let asm = crate::__expect!(
                        Decoder::new(
                            if cfg!(target_pointer_width = "64") {
                                64
                            } else {
                                32
                            },
                            core::slice::from_raw_parts(addr as _, 15),
                            DecoderOptions::NONE,
                        )
                        .iter()
                        .next(),
                        "Failed to disassemble instruction at address"
                    );
                    let end = match asm.mnemonic() {
                        Mnemonic::Lea | Mnemonic::Mov if asm.op1_kind() == OpKind::Memory => {
                            addr.wrapping_add(asm.memory_displacement64() as usize)
                        }
                        _ => unimplemented!(),
                    };
                    *self.0.get() = InnerOffset::Resolved(end);
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

    pub const fn smart(pat: &'static str) -> Self {
        Self(UnsafeCell::new(InnerOffset::Smart(pat)))
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
    (% $var:tt) => {
        $crate::RuntimeOffset::smart($var)
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
