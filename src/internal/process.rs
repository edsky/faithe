#![allow(missing_docs)]

use crate::FaitheError;
use std::mem::size_of as sof;
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
        )
        .0 == 0
    } {
        Err(FaitheError::last_error())
    } else {
        Ok(())
    }
}

/// Process Environmental Block.
#[repr(C)]
pub struct Peb {
    _pad0x2: [u8; 0x2],
    /// If process is being debugged.
    pub being_debugged: bool,
    pad0x10: [u8; 0xD],
    /// Base address of loaded image.
    pub image_base_address: *const (),
    /// Ldr data
    pub ldr_data: &'static PebLdrData,
}

#[repr(C)]
pub struct ListEntry<'a, T> {
    pub flink: &'a mut T,
    pub blink: &'a mut T,
}

#[repr(C)]
pub struct UnicodeString {
    pub len: u16,
    pub maximum_len: u16,
    pub buffer: *mut u16,
}

impl UnicodeString {
    /// Converts wide string into [`String`] by cloning data.
    pub fn decode_utf16(&self) -> String {
        unsafe {
            let utf16 = std::slice::from_raw_parts(self.buffer, self.len as usize / sof::<u16>());
            String::from_utf16_lossy(utf16)
        }
    }
}

#[repr(C)]
pub struct LdrDataTableEntry {
    _pad0x10: [u8; 0x10],
    pub in_memory_order_links: ListEntry<'static, LdrDataTableEntry>,
    _pad0x30: [u8; 0x10],
    pub dll_base: *const (),
    pub entry_point: *const (),
    pub image_size: u32,
    pub full_dll_name: UnicodeString,
    pub base_dll_name: UnicodeString,
}

#[repr(C)]
pub struct PebLdrData {
    pub len: u32,
    pub in_memory_order_links: ListEntry<'static, LdrDataTableEntry>,
}

/// Returns an address of PEB(Process Environmental Block).
#[cfg(feature = "nightly")]
#[inline(always)]
pub fn get_peb() -> &'static Peb {
    use super::get_teb;

    get_teb().process_environmental_block
}
