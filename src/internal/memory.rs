use crate::{size_of, types::MemoryProtection, FaitheError};
use std::mem::zeroed;
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualFree, VirtualQuery, VIRTUAL_ALLOCATION_TYPE, VIRTUAL_FREE_TYPE,
};

/// Changes the protection of memory pages of the target process.
/// For more info see [microsoft documentation](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualprotect).
#[rustfmt::skip]
pub fn protect(
    address: *mut (),
    size: usize,
    new_protection: MemoryProtection,
) -> crate::Result<MemoryProtection> {
    use windows::Win32::System::Memory::VirtualProtect;

    unsafe {
        let mut old = zeroed();
        if VirtualProtect(
            address as _,
            size,
            new_protection.to_os(),
            &mut old
        ) == false {
            Err(FaitheError::last_error())
        } else {
            MemoryProtection::from_os(old).ok_or(FaitheError::UnknownProtection(old.0))
        }
    }
}

/// Tries to allocate memory pages in the target process.
/// On success returns the address of allocated region.
#[rustfmt::skip]
pub fn allocate(
    address: usize,
    size: usize,
    allocation_type: VIRTUAL_ALLOCATION_TYPE,
    protection: MemoryProtection,
) -> crate::Result<*mut ()> {
    unsafe {
        let region = VirtualAlloc(
            Some(address as _),
            size,
            allocation_type,
            protection.to_os()
        );

        if region.is_null() {
            Err(FaitheError::last_error())
        } else {
            Ok(region as _)
        }
    }
}

/// Tries to free memory pages in the target process.
#[rustfmt::skip]
pub fn free(
    address: usize,
    size: usize,
    free_type: VIRTUAL_FREE_TYPE
) -> crate::Result<()>
{
    unsafe {
        if VirtualFree(
            address as _,
            size,
            free_type
        ) == false {
            Err(FaitheError::last_error())
        } else {
            Ok(())
        }
    }
}

/// Queries basic information about memory region at `address`.
#[cfg(windows)]
pub fn query(address: usize) -> crate::Result<crate::types::MemoryBasicInformation> {
    unsafe {
        let mut mem_info = zeroed();
        if VirtualQuery(Some(address as _), &mut mem_info, size_of!(@ mem_info)) == 0 {
            Err(FaitheError::last_error())
        } else {
            Ok(mem_info.into())
        }
    }
}
