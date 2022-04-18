/// This macros allows you to generate structure with explicitly defined
/// fields' offsets.
/// # Warning
/// This macros might generate ineffecient assembly code.
/// ```
/// # use faithe::{xstruct, size_of};
/// xstruct! {
///     // STRUCT HAS SIZE OF ZERO.
///     struct Foo {
///         0x0 @ a: u32,
///         0x16 @ b: bool
///     }
///
///     // STRUCT HAS SIZE 20.
///     struct Bar(20) {
///         0x0 @ a: u32,
///         0x16 @ b: bool
///     }
/// }
/// assert_eq!(size_of!(Foo), 0);
/// assert_eq!(size_of!(Bar), 20);
/// ```
#[macro_export]
macro_rules! xstruct {
    (
        $(
            $vm:vis struct $name:ident$(($size:tt))? {
                $(
                    $offset:tt $mode:tt $flvm:vis $field_name:ident: $field_ty:ty
                ),*
            }
        )*
    ) => {
        $(
            #[repr(transparent)]
            $vm struct $name$(([u8; $size]))?;
            impl $name {
                $(
                    $crate::xstruct_field! { $mode $offset $flvm $field_name $field_ty }
                )*
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! xstruct_field {
    (@ $offset:tt $flvm:vis $field_name:ident $field_ty:ty) => {
        #[allow(non_snake_case)]
        #[inline(always)]
        $flvm unsafe fn $field_name(&self) -> &mut $field_ty {
            $crate::to_mut_ref((self as *const Self as usize + $offset) as _)
        }
    };
    (# $offset:tt $flvm:vis $field_name:ident $field_ty:ty) => {
        #[allow(non_snake_case)]
        #[inline(always)]
        $flvm unsafe fn $field_name(&self) -> &$field_ty {
            $crate::to_mut_ref((self as *const Self as usize + $offset) as _)
        }
    };
    (% $offset:tt $flvm:vis $field_name:ident $field_ty:ty) => {
        #[allow(non_snake_case)]
        #[inline(always)]
        $flvm unsafe fn $field_name(&mut self) -> &mut $field_ty {
            $crate::to_mut_ref((self as *const Self as usize + $offset) as _)
        }
    };
}
