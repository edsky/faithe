use windows::Win32::Foundation::HANDLE;

/// Returns a handle to the current process.
pub fn get_current_process() -> HANDLE {
    unsafe { windows::Win32::System::Threading::GetCurrentProcess() }
}

/// Returns the id of the current process
pub fn get_current_process_id() -> u32 {
    unsafe { windows::Win32::System::Threading::GetCurrentProcessId() }
}
