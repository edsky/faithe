/// Get size in bytes of types and variables.
/// ```
/// # use radon::size_of;
/// /// Get's size of the type in bytes.
/// assert_eq!(size_of!(u32), 4);
///
/// let var = 5u64;
/// // Use `@` sign to get size of variable.
/// assert_eq!(size_of!(@ var), 8);
/// ```
#[macro_export]
macro_rules! size_of {
    ($type:ty) => {
        std::mem::size_of::<$type>()
    };
    (@ $var:ident) => {
        std::mem::size_of_val(&$var)
    };
}
