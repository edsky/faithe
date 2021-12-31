/// Constructs a zero-terminated string at compile time.
/// ```
/// # use radon::c_str;
/// assert_eq!(c_str!("Hello", ", World!").as_bytes(), b"Hello, World!\x00");
/// ```
#[macro_export]
macro_rules! c_str {
    ($($str:tt),*) => {
        concat!($($str),*, '\x00')
    };
}
