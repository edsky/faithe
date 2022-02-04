use std::fmt::Display;

/// Error type for all mistakes made in faithe.
#[derive(Debug)]
pub enum FaitheError {
    /// Error code returned from `GetLastError()` WinAPI.
    ErrorCode(windows::Win32::Foundation::WIN32_ERROR),
    /// No process with selected name were found.
    ProcessNotFound,
    /// No module with selected name were found.
    ModuleNotFound,
}

impl FaitheError {
    pub(crate) fn last_error() -> Self {
        unsafe { Self::ErrorCode(windows::Win32::Foundation::GetLastError()) }
    }
}

impl Display for FaitheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FaitheError {}

pub(crate) type Result<T> = std::result::Result<T, FaitheError>;
