use windows::Win32::Foundation::HANDLE;

/// Returns a handle to the current process.
pub fn get_current_process() -> HANDLE {
    unsafe { windows::Win32::System::Threading::GetCurrentProcess() }
}

/// Returns the id of the current process
pub fn get_current_process_id() -> u32 {
    unsafe { windows::Win32::System::Threading::GetCurrentProcessId() }
}

/// Allocates console windows ig.
pub fn alloc_console() -> i32 {
    unsafe { windows::Win32::System::Console::AllocConsole().0 }
}

/// Frees console.
pub fn free_console() -> i32 {
    unsafe { windows::Win32::System::Console::FreeConsole().0 }
}
