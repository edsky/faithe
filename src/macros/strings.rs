/// Constructs a zero-terminated string at compile time.
/// ```
/// # use faithe::c_str;
/// assert_eq!(c_str!("Deceive me!").as_bytes(), b"Deceive me!\x00");
/// assert_eq!(c_str!("Hello", ", World!").as_bytes(), b"Hello, World!\x00");
/// ```
#[macro_export]
macro_rules! c_str {
    ($($str:tt),*) => {
        concat!($($str),*, '\x00')
    };
}

/// Constructs new zero terminated string of type [`windows::Win32::Foundation::PSTR`].
#[cfg(all(not(feature = "no-std"), windows))]
#[macro_export]
macro_rules! pc_str {
    ($($str:tt),*) => {
        windows::Win32::Foundation::PSTR(concat!($($str),*, '\x00').as_ptr() as _)
    };
}
