#![allow(dead_code)]
/// Error type for all mistakes made in faithe.
#[derive(Debug)]
pub enum FaitheError {
    #[cfg(not(feature = "no-std"))]
    /// Error code returned from `GetLastError()` WinAPI.
    ErrorCode(windows::Win32::Foundation::WIN32_ERROR),
    /// No process with selected name were found.
    ProcessNotFound,
    /// No module with selected name were found.
    ModuleNotFound,
    /// Protection that cannot be represented with internal type.
    #[cfg(all(windows, not(feature = "no-std")))]
    UnknownProtection(u32),
    /// String is not a valid UTF-8/UTF-16 sequence.
    InvalidString,
    /// Pattern is not an ASCII sequence.
    NonAsciiPattern,
    /// Supplied string cannot be parsed as a pattern of given type.
    InvalidPattern,
}

pub(crate) type Result<T> = core::result::Result<T, FaitheError>;

cfg_if::cfg_if! {
    if #[cfg(not(feature = "no-std"))] {
        impl std::error::Error for FaitheError {}

        impl std::fmt::Display for FaitheError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl FaitheError {
            pub(crate) fn last_error() -> Self {
                unsafe { Self::ErrorCode(windows::Win32::Foundation::GetLastError()) }
            }
        }
    }
}
