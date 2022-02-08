use std::cell::UnsafeCell;

enum InnerOffset {
    Explicit(usize),
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
        unsafe {
            match *(self.0.get()) {
                InnerOffset::Resolved(_) => true,
                _ => false,
            }
        }
    }

    #[inline]
    pub fn resolve(&self, module: &'static str) {
        let address = crate::internal::get_module_handle(module).unwrap().0 as usize;
        unsafe {
            match *(self.0.get()) {
                InnerOffset::Explicit(offset) => {
                    *self.0.get() = InnerOffset::Resolved(address + offset);
                }
                InnerOffset::Resolved(_) => unreachable!(),
            }
        }
    }

    pub const fn explicit(offset: usize) -> Self {
        Self(UnsafeCell::new(InnerOffset::Explicit(offset)))
    }
}

/// Creates function that resolves its address on the first call.
#[macro_export]
macro_rules! function {
    (
        $(
            $vs:vis extern $name:ident: extern $cc:tt fn($($arg_id:ident: $arg_ty:ty),*) $(-> $ret_ty:ty)? = $lib_name:tt@$offset:tt;
        )*
    ) => {
        $(
            $vs static $name: $name = $name {
                module: $lib_name,
                offset: $crate::RuntimeOffset::explicit($offset)
            };
            $vs struct $name {
                module: &'static str,
                offset: $crate::RuntimeOffset,
            }
            unsafe impl ::std::marker::Sync for $name { }
            impl $name {
                $vs unsafe extern $cc fn call(&self, $($arg_id:$arg_ty),*) $(-> $ret_ty)? {
                    if !self.offset.is_resolved() {
                        self.offset.resolve(self.module);
                    }
                    ::std::mem::transmute::<_, extern $cc fn($($arg_ty),*) $(-> $ret_ty)?>(self.offset.address())($($arg_id),*);
                }
            }
        )*
    };
}
