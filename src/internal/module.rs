use crate::{pattern::Pattern, FaitheError};
use windows::Win32::System::ProcessStatus::MODULEINFO;

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

/// Returns an address(its handle) of the module.
/// Always returns Ok(address).
#[cfg(feature = "nightly")]
pub fn get_module_address(mod_name: impl AsRef<str>) -> crate::Result<*mut ()> {
    use crate::{containing_record, internal::LdrDataTableEntry};
    use super::get_peb;

    unsafe {
        let first = containing_record!(get_peb().ldr_data.in_memory_order_links, LdrDataTableEntry, in_memory_order_links);
        let mut current = first;
        loop {
            if let Some(s) = (*current).base_dll_name.decode_utf16() {
                if s == mod_name.as_ref()  {
                    break Ok((*current).dll_base as _);
                }
            }
            
            current = containing_record!((*current).in_memory_order_links, LdrDataTableEntry, in_memory_order_links);
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
    use crate::{containing_record, internal::LdrDataTableEntry};
    use super::get_peb;

    unsafe {
        let first = containing_record!(get_peb().ldr_data.in_memory_order_links, LdrDataTableEntry, in_memory_order_links);
        let mut current = first;
        loop {
            if let Some(s) = (*current).base_dll_name.decode_utf16() {
                if s == mod_name.as_ref()  {
                    break Ok(ModuleInfo {
                        dll_base: (*current).dll_base,
                        image_size: (*current).image_size as _,
                        entry_point: (*current).entry_point,
                    });
                }
            }

            current = containing_record!((*current).in_memory_order_links, LdrDataTableEntry, in_memory_order_links);
            if current == first {
                break Err(FaitheError::ModuleNotFound);
            }
        }
    }
}

/// Returns information about the specified module.
#[cfg(not(feature = "nightly"))]
pub fn get_module_information(mod_name: impl AsRef<str>) -> crate::Result<ModuleInfo> {
    use windows::Win32::{Foundation::HINSTANCE, System::ProcessStatus::K32GetModuleInformation};
    use crate::{internal::get_current_process, size_of};
    use std::mem::zeroed;

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
pub fn find_pattern(mod_name: impl AsRef<str>, pat: Pattern) -> crate::Result<Option<*mut ()>> {
    let info = get_module_information(mod_name)?;

    for addr in (info.dll_base as usize)..(info.dll_base as usize + info.image_size) {
        if unsafe { pat.matches(std::slice::from_raw_parts(addr as _, pat.len())) } {
            return Ok(Some(addr as _));
        }
    }
    Ok(None)
}
