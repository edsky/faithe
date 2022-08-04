use std::ptr::NonNull;

use crate::{pattern::Pattern, FaitheError};
use windows::{
    core::PCWSTR,
    Win32::System::{LibraryLoader::LoadLibraryW, ProcessStatus::MODULEINFO},
};

/// Basic information about process's module.
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    /// Base of the module.
    pub dll_base: *mut (),
    /// Size of the image.
    pub image_size: usize,
    /// Address of the dll's entry point.
    pub entry_point: *mut (),
}

impl From<MODULEINFO> for ModuleInfo {
    fn from(mi: MODULEINFO) -> Self {
        Self {
            dll_base: mi.lpBaseOfDll as _,
            image_size: mi.SizeOfImage as _,
            entry_point: mi.EntryPoint as _,
        }
    }
}

/// Either loads library from path or returns an address of already existing module.
pub fn load_library(lib_name: impl AsRef<str>) -> crate::Result<NonNull<()>> {
    unsafe {
        let utf16 = format!("{}\x00", lib_name.as_ref())
            .encode_utf16()
            .collect::<Vec<_>>();

        LoadLibraryW(PCWSTR(utf16.as_ptr()))
            .map_err(|_| FaitheError::last_error())
            .map(|v| NonNull::new_unchecked(v.0 as _))
    }
}

/// Returns an address(its handle) of the module or Err if failed to find the specified module.
#[cfg(feature = "nightly")]
pub fn get_module_address(mod_name: impl AsRef<str>) -> crate::Result<*mut ()> {
    use super::get_peb;
    use crate::{containing_record, internal::LdrDataTableEntry};

    unsafe {
        let first = containing_record!(
            get_peb().ldr_data.in_memory_order_links,
            LdrDataTableEntry,
            in_memory_order_links
        );
        let mut current = first;
        loop {
            if let Some(s) = (*current).base_dll_name.decode_utf16() {
                if s == mod_name.as_ref() {
                    break Ok((*current).dll_base as _);
                }
            }

            current = containing_record!(
                (*current).in_memory_order_links,
                LdrDataTableEntry,
                in_memory_order_links
            );
            if current == first {
                break Err(FaitheError::ModuleNotFound);
            }
        }
    }
}

/// Returns an address(its handle) of the module.
#[cfg(not(feature = "nightly"))]
pub fn get_module_address(mod_name: impl AsRef<str>) -> crate::Result<*mut ()> {
    use windows::Win32::{Foundation::PWSTR, System::LibraryLoader::GetModuleHandleW};

    let mut utf16 = format!("{}\x00", mod_name.as_ref())
        .encode_utf16()
        .collect::<Vec<u16>>();
    unsafe {
        let handle = GetModuleHandleW(PWSTR(utf16.as_mut_ptr()));

        if handle.is_invalid() {
            Err(FaitheError::last_error())
        } else {
            Ok(handle.0 as _)
        }
    }
}

/// Returns information about the specified module.
#[cfg(feature = "nightly")]
pub fn get_module_information(mod_name: impl AsRef<str>) -> crate::Result<ModuleInfo> {
    use super::get_peb;
    use crate::{containing_record, internal::LdrDataTableEntry};

    let mod_name = mod_name.as_ref().to_lowercase();

    unsafe {
        let first = containing_record!(
            get_peb().ldr_data.in_memory_order_links,
            LdrDataTableEntry,
            in_memory_order_links
        );
        let mut current = first;
        loop {
            if let Some(s) = (*current).base_dll_name.decode_utf16().map(|s| s.to_lowercase()) {
                if mod_name == s {
                    break Ok(ModuleInfo {
                        dll_base: (*current).dll_base,
                        image_size: (*current).image_size as _,
                        entry_point: (*current).entry_point,
                    });
                }
            }

            current = containing_record!(
                (*current).in_memory_order_links,
                LdrDataTableEntry,
                in_memory_order_links
            );
            if current == first {
                break Err(FaitheError::ModuleNotFound);
            }
        }
    }
}

/// Returns information about the specified module.
#[cfg(not(feature = "nightly"))]
pub fn get_module_information(mod_name: impl AsRef<str>) -> crate::Result<ModuleInfo> {
    use crate::{internal::get_current_process, size_of};
    use std::mem::zeroed;
    use windows::Win32::{Foundation::HINSTANCE, System::ProcessStatus::K32GetModuleInformation};

    unsafe {
        let mut mod_info = zeroed();
        if K32GetModuleInformation(
            get_current_process(),
            HINSTANCE(get_module_address(mod_name)? as _),
            &mut mod_info,
            size_of!(@mod_info) as _,
        ) == false
        {
            Err(FaitheError::last_error())
        } else {
            Ok(mod_info.into())
        }
    }
}

/// Searches module for specific memory pattern.
pub fn find_pattern(mod_name: impl AsRef<str>, pat: Pattern) -> crate::Result<Option<NonNull<()>>> {
    let info = get_module_information(mod_name)?;

    for addr in (info.dll_base as usize)..(info.dll_base as usize + info.image_size) {
        if unsafe { pat.matches(std::slice::from_raw_parts(addr as _, pat.len())) } {
            return Ok(NonNull::new(addr as _));
        }
    }
    Ok(None)
}
