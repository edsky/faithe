/// Creates global that resolves its address on the first access.
/// ```ignore
/// global! {
///     // Explicitly defined RVA offset relative to `01-hello` module.
///     extern COUNT: i32 = "01-hello.exe"@0x1234;
/// }
/// // On `get` the value of type `i32` will be read at address `base("01-hello.exe") + 0x1234`.
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
            unsafe impl ::std::marker::Sync for $name { }
            impl $name {
                $vs unsafe fn get(&self) -> $fty {
                    if !self.offset.is_resolved() {
                        $crate::__expect!(self.offset.try_resolve($lib_name, $crate::__define_offset2!($($add)?)), "Failed to resolve global's address");
                    }
                    (self.offset.address() as *const $fty).read()
                }
            }
        )*
    };
}
