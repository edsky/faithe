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
}

cfg_if::cfg_if! {
    if #[cfg(not(feature = "no-std"))] {
        impl std::error::Error for FaitheError {}

        pub(crate) type Result<T> = core::result::Result<T, FaitheError>;

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

