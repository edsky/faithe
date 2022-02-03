use std::fmt::Display;

/// Error type for all mistakes made in radon.
#[derive(Debug)]
pub enum RadonError {
    /// Error code returned from `GetLastError()` WinAPI.
    ErrorCode(windows::Win32::Foundation::WIN32_ERROR),
    /// No process with selected name were found.
    ProcessNotFound,
    /// No module with selected name were found.
    ModuleNotFound,
}

impl RadonError {
    pub(crate) fn last_error() -> Self {
        unsafe { Self::ErrorCode(windows::Win32::Foundation::GetLastError()) }
    }
}

impl Display for RadonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RadonError { }

pub(crate) type Result<T> = std::result::Result<T, RadonError>;
