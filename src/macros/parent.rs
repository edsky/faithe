/// Creates C-like inherited structures. First member of the struct is instance of the parent class.
/// This macro also implements [`core::ops::Deref`] for child class so you can easily access parent's fields. 
/// # Note
/// You will problaly want to put `#[repr(C)]` on structures because this macro doesn't do this by default.
/// ```
/// # use faithe::parent;
/// 
/// parent! {
///     #[repr(C)]
///     pub struct Parent {
///         pub c: i32,
///         pub d: i32
///     }
/// 
///     #[repr(C)]
///     pub struct Child(pub Parent) {
///         pub a: i32,
///         pub b: i32
///     }
/// }
/// 
/// struct Parent {
///     
/// }
/// ```
#[macro_export]
macro_rules! parent {
    {
        $(
            $(#[$($attr:tt)*])*
            $svs:vis struct $name:ident$(($pvs:vis $parent:ident))? {
                $($fvs:vis $fname:ident: $fty:ty),*
            }
        )*
    } => {
        $(
            $(
                #[$($attr)*]
            )*
            $svs struct $name {
                $($pvs parent: $parent,)?
                $(
                    $fvs $fname: $fty,
                )*
            }

            $(
                impl core::ops::Deref for $name {
                    type Target = $parent;

                    fn deref(&self) -> &Self::Target {
                        &self.parent
                    }
                }
            )?
        )*
    };
}