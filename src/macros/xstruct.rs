/// **WARNING**: This may(or may not) generate not very effecient assembly.
/// This macros allows you to generate structure with explicitly defined
/// field's offsets.
/// ```
/// # use radon::xstruct;
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
        struct $name:ident {
            $(
                $offset:tt @ $field_name:ident: $field_ty:ty
            ),*
        }
    ) => {
        struct $name;
        impl $name {
            $(
                #[allow(non_snake_case)]
                #[inline(always)]
                pub fn $field_name(&self) -> $field_ty {
                    unsafe { *((self as *const Self as usize + $offset) as *const $field_ty) }
                }
            )*
        }
    };
}
