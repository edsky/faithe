/// Error type for all mistakes made in radon.
#[derive(Debug)]
pub enum RadonError {
    /// Error code returned from `GetLastError()` WinAPI.
    #[cfg(feature = "win32")]
    ErrorCode(windows::Win32::Foundation::WIN32_ERROR),
    /// No processes with selected filters were found.
    ProcessNotFound,
}

impl RadonError {
    #[cfg(feature = "win32")]
    pub(crate) fn last_error() -> Self {
        unsafe { Self::ErrorCode(windows::Win32::Foundation::GetLastError()) }
    }
}

pub(crate) type Result<T> = std::result::Result<T, RadonError>;
