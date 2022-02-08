use crate::FaitheError;
use windows::Win32::{
    Foundation::{HANDLE, HWND, PWSTR},
    System::{Console, Threading},
    UI::WindowsAndMessaging::{MessageBoxW, MESSAGEBOX_STYLE},
};

/// Returns a handle to the current process.
pub fn get_current_process() -> HANDLE {
    HANDLE(usize::MAX as isize)
}

/// Returns the id of the current process.
pub fn get_current_process_id() -> u32 {
    unsafe { Threading::GetCurrentProcessId() }
}

/// Allocates console windows ig.
pub fn alloc_console() -> crate::Result<()> {
    if unsafe { Console::AllocConsole().0 == 0 } {
        Err(FaitheError::last_error())
    } else {
        Ok(())
    }
}

/// Frees console.
pub fn free_console() -> crate::Result<()> {
    if unsafe { Console::FreeConsole().0 == 0 } {
        Err(FaitheError::last_error())
    } else {
        Ok(())
    }
}

/// Creates new message box.
pub fn message_box(
    hwnd: Option<HWND>,
    text: impl AsRef<str>,
    caption: impl AsRef<str>,
    style: MESSAGEBOX_STYLE,
) -> crate::Result<()> {
    if unsafe {
        MessageBoxW(
            hwnd,
            PWSTR(
                format!("{}\x00", text.as_ref())
                    .encode_utf16()
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
            ),
            PWSTR(
                format!("{}\x00", caption.as_ref())
                    .encode_utf16()
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
            ),
            style,
        ).0 == 0
    } {
        Err(FaitheError::last_error())
    } else {
        Ok(())
    }
}

/// Process Environmental Block.
#[repr(C)]
pub struct PEB {
    _pad0x2: [u8; 0x2],
    /// If process is being debugged.
    pub being_debugged: bool,
    pad0x10: [u8; 0xD],
    /// Base address of loaded image.
    pub image_base_address: *const ()
}

/// Returns an address of PEB(Process Environmental Block).
#[cfg(feature = "nightly")]
#[inline(always)]
pub fn get_peb() -> &'static PEB {
    use super::get_teb;

    get_teb().process_environmental_block
}