/// Creates global that resolves its address on the first access.
/// ```ignore
/// global! {
///     // Explicitly defined RVA offset relative to `01-hello` module.
///     extern COUNT: i32 = "01-hello.exe"@0x1234;
/// }
/// // On `get` the value of type `i32` will be read at address `base("01-hello.exe") + 0x1234`.
/// // COUNT also implements `AsRef` and `AsMut` traits but be careful because these methods can cause crash because they don't require unsafe block.
/// assert_eq!(COUNT.get(), 123);
/// ```
#[macro_export]
macro_rules! global {
    (
        $(
            $vs:vis extern $name:ident: $fty:ty = $lib_name:tt$sep:tt$var:tt$([$add:tt])?;
        )*
    ) => {
        $(
            #[allow(non_upper_case_globals)]
            $vs static $name: $name = $name {
                offset: $crate::__define_offset!($sep $var)
            };
            #[allow(non_camel_case_types)]
            $vs struct $name {
                offset: $crate::RuntimeOffset,
            }
            unsafe impl ::core::marker::Sync for $name { }
            impl $name {
                #[inline]
                $vs unsafe fn get(&self) -> $fty {
                    std::ptr::read(self.get_ref() as _)
                }

                #[inline]
                $vs unsafe fn get_ref(&self) -> &$fty {
                    if !self.offset.is_resolved() {
                        $crate::__expect!(self.offset.try_resolve($lib_name, $crate::__define_offset2!($($add)?)), "Failed to resolve global's address");
                    }

                    (self.offset.address() as *const $fty).as_ref().unwrap()
                }

                #[inline]
                $vs unsafe fn get_mut(&mut self) -> &mut $fty {
                    if !self.offset.is_resolved() {
                        $crate::__expect!(self.offset.try_resolve($lib_name, $crate::__define_offset2!($($add)?)), "Failed to resolve global's address");
                    }

                    (self.offset.address() as *mut $fty).as_mut().unwrap()
                }
            }

            impl ::core::convert::AsRef<$fty> for $name {
                fn as_ref(&self) -> &$fty {
                    unsafe { self.get_ref() }
                }
            }

            impl ::core::convert::AsMut<$fty> for $name {
                fn as_mut(&mut self) -> &mut $fty {
                    unsafe { self.get_mut() }
                }
            }
        )*
    };
}
