/// This macro allows you to create a non object safe trait that
/// allows calling cpp-like virtual methods in classes.
/// ```
/// # use radon::interface;
/// struct Bar;
///
/// interface! {
///     trait Foo {
///         2 @ fn get_a() -> u32;
///         3 @ fn set_b(val: u16);
///     }
///     // you also specify structs you want to implement this trait for.
///     impl for Bar;
/// }
/// ```
#[macro_export]
macro_rules! interface {
    (
        $vm:vis trait $name:ident {
            $(
                $idx:tt @ fn $fn_ident:ident(
                    $(
                        $arg_name:ident: $arg_ty:ty
                    ),*
                ) $(-> $ret_ty:ty)?;
            )*
        }
        $(impl for $($impl_target:ty),*;)?

    ) => {
        $vm trait $name {
            const __NO_OBJ_SAFETY: () = ();
            $(
                #[allow(non_snake_case)]
                #[inline(always)]
                unsafe fn $fn_ident(&self, $($arg_name:$arg_ty,)*) $(-> $ret_ty)? {
                    let vmt = *(self as *const Self as *const *const [extern "C" fn($($arg_name:$arg_ty),*) $(-> $ret_ty)?; $idx + 1]);
                    (*vmt)[$idx]($($arg_name),*)
                }
            )*
        }
        $(
            $(
                impl $name for $impl_target { }
            )*
        )?
    };
}
