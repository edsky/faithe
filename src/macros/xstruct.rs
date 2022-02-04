/// This macros allows you to generate structure with explicitly defined
/// field's offsets. **SHOULD NEVER BE USED AS A MEMBER** because it has zero
/// size.
/// # Warning
/// This macros might generate ineffecient assembly code.
/// ```
/// # use faithe::xstruct;
/// xstruct! {
///     struct Bar {
///         0x0 @ a: u32,
///         0x16 @ b: bool
///     }
/// }
/// ```
#[macro_export]
macro_rules! xstruct {
    (
        $vm:vis struct $name:ident {
            $(
                $offset:tt @ $flvm:vis $field_name:ident: $field_ty:ty
            ),*
        }
    ) => {
        $vm struct $name;
        impl $name {
            $(
                #[allow(non_snake_case)]
                #[inline(always)]
                $flvm fn $field_name(&self) -> &mut $field_ty {
                    unsafe { $crate::to_mut_ref((self as *const Self as usize + $offset) as _) }
                }
            )*
        }
    };
}
