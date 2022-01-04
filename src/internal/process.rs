use windows::Win32::{
    Foundation::{HANDLE, HWND, PWSTR},
    System::{Console, Threading},
    UI::WindowsAndMessaging::{MESSAGEBOX_STYLE, MessageBoxW},
};

/// Returns a handle to the current process.
pub fn get_current_process() -> HANDLE {
    unsafe { Threading::GetCurrentProcess() }
}

/// Returns the id of the current process.
pub fn get_current_process_id() -> u32 {
    unsafe { Threading::GetCurrentProcessId() }
}

/// Allocates console windows ig.
pub fn alloc_console() -> i32 {
    unsafe { Console::AllocConsole().0 }
}

/// Frees console.
pub fn free_console() -> i32 {
    unsafe { Console::FreeConsole().0 }
}

/// Creates new message box.
pub fn message_box(
    hwnd: Option<HWND>,
    text: impl AsRef<str>,
    caption: impl AsRef<str>,
    style: MESSAGEBOX_STYLE,
) -> i32 {
    unsafe {
        MessageBoxW(
            hwnd,
            PWSTR(format!("{}\x00", text.as_ref()).encode_utf16().collect::<Vec<_>>().as_mut_ptr()),
            PWSTR(format!("{}\x00", caption.as_ref()).encode_utf16().collect::<Vec<_>>().as_mut_ptr()),
            style
        )
    }
}
