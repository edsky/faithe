/// Creates an trait that emulates virtual table behavior from C++.
/// You can use `'this` lifetime for arguments that requires `self` lifetime.
/// ```
/// # use faithe::interface;
/// struct CPlayer;
/// interface! {
///     trait IEntity(CPlayer) {
///         // 1 - is an index of this function in the table.
///         extern "C" fn print() = 1;
///         // Lifetimes works too
///         extern "C" fn other<'a>(num: &'a i32) -> &'a i32 = 2;
///     }
/// }
/// // You can use `IInterface::vaddress(obj, "func_name")` to get the address of the function by its name or
/// // `IInterface::vaddress(obj, <function_index>)` to get the address by the function's index.
/// ```
#[macro_export]
macro_rules! interface {
    (
        $(
            $vs:vis trait $name:ident$(($($target:ident$(<$($tlf:tt),*>)?),*))? {
                $(
                    $(extern $($cc:tt)?)? fn $fn_id:ident$(<$($gen:tt),*>)?($($arg_id:ident: $arg_ty:ty),*) $(-> $ret_ty:ty)? = $idx:expr;
                )*
            }
        )*
    ) => {
        $(
            $vs unsafe trait $name: ::core::marker::Sized {
                $(
                    #[inline(always)]
                    #[allow(non_snake_case)]
                    $(extern $($cc)?)? fn $fn_id<'this$(,$($gen),*)?>(&'this self, $($arg_id: $arg_ty),*) $(-> $ret_ty)? {
                        unsafe {
                            let slot = *(self as *const Self as *const usize) + $idx * core::mem::size_of::<usize>();
                            (*core::mem::transmute::<_, *const $(extern $($cc)?)? fn(&Self, $($arg_ty),*) $(-> $ret_ty)?>(slot))
                            (self, $($arg_id),*)
                        }
                    }
                )*

                /// Returns the address of the virtual function by its name.
                unsafe fn vaddress(&self, name: &'static str) -> usize {
                    match name {
                        $(
                            stringify!( $fn_id ) => {
                                (*(self as *const Self as *const &'static [usize; $idx + 1]))[$idx]
                            }
                        )*
                        _ => panic!("Unknown function")
                    }
                }

                /// Returns the address of the virtual function by its index.
                unsafe fn vindex(&self, idx: usize) -> usize {
                    *(*(self as *const Self as *const *const usize)).add(idx)
                }
            }
            $(
                $(
                    unsafe impl$(<$($tlf),*>)? $name for $target$(<$($tlf),*>)? { }
                )*
            )?
        )*
    };
}
