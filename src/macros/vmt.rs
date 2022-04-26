/// Creates a virtual method table.
/// # Behaviour
/// Each time macro is used, it will create new virtual method table via [`Box::leak`]
/// ```
/// # use faithe::vmt
/// fn first() {
///     println!("First");
/// }
/// fn second() {
///     println!("Second");
/// }
///
/// let vmt = vmt! {
///     first,
///     second
/// };
/// ```
#[macro_export]
macro_rules! vmt {
    (
        $var:ident => [
            $($fn:ident),*
        ]
    ) => {
        *core::mem::transmute::<_, *mut usize>(&$var) = Box::leak(
            Box::new(
                [
                    $($fn as usize),*
                ]
            )
        ).as_ptr() as usize
    };
}
