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
            $vs:vis $name:ident: $(extern $cc:tt)? fn($($arg_id:ident: $arg_ty:ty),*) $(-> $ret_ty:ty)? = $lib_name:tt$sep:tt$var:tt$([$add:tt])?;
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
                $vs unsafe fn call(&self, $($arg_id:$arg_ty),*) $(-> $ret_ty)? {
                    if !self.offset.is_resolved() {
                        $crate::__expect!(self.offset.try_resolve($lib_name, $crate::__define_offset2!($($add)?)), "Failed to resolve function's address");
                    }
                    ::std::mem::transmute::<_, $(extern $cc)? fn($($arg_ty),*) $(-> $ret_ty)?>(self.offset.address())($($arg_id),*);
                }
            }
        )*
    };
}
