use crate::{FaitheError, pattern::Pattern};
use std::cell::UnsafeCell;

enum InnerOffset {
    Explicit(usize),
    Pattern(&'static str),
    Resolved(usize),
}

// @TODO: Replace with `OnceCell` from `once_cell` crate.
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
    pub fn try_resolve(&self, module: &'static str) -> crate::Result<()> {
        unsafe {
            match *(self.0.get()) {
                InnerOffset::Explicit(offset) => {
                    let base = crate::internal::get_module_address(module)? as usize;
                    *self.0.get() = InnerOffset::Resolved(base + offset);
                    Ok(())
                }
                InnerOffset::Pattern(pattern) => {
                    let addr = crate::internal::find_pattern(module, Pattern::from_ida_style(pattern))?
                        .ok_or(FaitheError::PatternNotFound)?
                        .as_ptr() as usize;
                    *self.0.get() = InnerOffset::Resolved(addr);
                    Ok(())
                },
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

/// Creates function that resolves its address on the first call.
/// ```ignore
/// function! {
///     // Explicitly defined RVA offset relative to `01-hello` module.
///     extern FUNC: extern "C" fn(a: i32) = "01-hello.exe"@0x1900;
/// }
/// FUNC.call(5);
/// ```
#[macro_export]
macro_rules! function {
    (
        $(
            $vs:vis $name:ident: $(extern $cc:tt)? fn($($arg_id:ident: $arg_ty:ty),*) $(-> $ret_ty:ty)? = $lib_name:tt$sep:tt$var:tt;
        )*
    ) => {
        $(
            $vs static $name: $name = $name {
                module: $lib_name,
                offset: $crate::__define_offset!($sep $var)
            };
            $vs struct $name {
                module: &'static str,
                offset: $crate::RuntimeOffset,
            }
            unsafe impl ::std::marker::Sync for $name { }
            impl $name {
                $vs unsafe fn call(&self, $($arg_id:$arg_ty),*) $(-> $ret_ty)? {
                    if !self.offset.is_resolved() {
                        $crate::__expect!(self.offset.try_resolve(self.module), "Failed to resolve function's address");
                    }
                    ::std::mem::transmute::<_, $(extern $cc)? fn($($arg_ty),*) $(-> $ret_ty)?>(self.offset.address())($($arg_id),*);
                }
            }
        )*
    };
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