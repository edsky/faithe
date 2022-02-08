/// Creates an trait that emulates virtual table behavior from C++.
/// ```
/// # use faithe::interface;
/// struct CPlayer;
/// interface! {
///     trait IEntity(CPlayer) {
///         // 1 - is an index of this function in the table.
///         extern "C" fn print() = 1;
///     }
/// }
/// ```
#[macro_export]
macro_rules! interface {
    (
        $(
            $vs:vis trait $name:ident$(($($target:ident),*))? {
                $(
                    extern $cc:tt fn $fn_id:ident($($arg_id:ident: $arg_ty:ty),*) $(-> $ret_ty:ty)? = $idx:tt;
                )*
            }
        )*
    ) => {
        $(
            $vs unsafe trait $name: ::std::marker::Sized {
                $(
                    #[inline(always)]
                    #[allow(non_snake_case)]
                    extern $cc fn $fn_id(&self, $($arg_id: $arg_ty),*) $(-> $ret_ty)? {
                        unsafe {
                            let slot = *(self as *const Self as *const usize) + $idx * std::mem::size_of::<usize>();
                            (*std::mem::transmute::<_, *const extern $cc fn(&Self, $($arg_ty),*) $(-> $ret_ty)?>(slot))
                            (self, $($arg_id),*);
                        }
                    }
                )*
            }
            $(
                $(
                    unsafe impl $name for $target { }
                )*
            )?
        )*
    };
}
