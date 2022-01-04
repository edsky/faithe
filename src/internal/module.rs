use super::get_current_process;
use crate::{pattern::Pattern, size_of, RadonError};
use std::mem::zeroed;
use windows::Win32::{
    Foundation::{HINSTANCE, PWSTR},
    System::{
        LibraryLoader::GetModuleHandleW,
        ProcessStatus::{K32GetModuleInformation, MODULEINFO},
    },
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

/// Returns a handle to a module.
pub fn get_module_handle(mod_name: impl AsRef<str>) -> crate::Result<HINSTANCE> {
    let mut utf16 = mod_name.as_ref().encode_utf16().collect::<Vec<u16>>();
    unsafe {
        let handle = GetModuleHandleW(PWSTR(utf16.as_mut_ptr()));

        if handle == 0 {
            Err(RadonError::last_error())
        } else {
            Ok(handle)
        }
    }
}

/// Returns information about module.
pub fn get_module_information(mod_name: impl AsRef<str>) -> crate::Result<ModuleInfo> {
    unsafe {
        let mut mod_info = zeroed();
        if K32GetModuleInformation(
            get_current_process(),
            get_module_handle(mod_name)?,
            &mut mod_info,
            size_of!(@mod_info) as _,
        ) == false
        {
            Err(RadonError::last_error())
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
