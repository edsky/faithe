use crate::{memory::MemoryBasicInformation, size_of, RadonError};
use std::mem::zeroed;
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualFree, VirtualProtect, VirtualQuery, PAGE_PROTECTION_FLAGS,
    VIRTUAL_ALLOCATION_TYPE, VIRTUAL_FREE_TYPE,
};

/// Changes the protection of memory pages of the target process.
/// For more info see [microsoft documentation](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualprotectex).
#[rustfmt::skip]
pub fn virtual_protect(
    address: usize,
    size: usize,
    new_protection: PAGE_PROTECTION_FLAGS,
) -> crate::Result<PAGE_PROTECTION_FLAGS> {
    unsafe {
        let mut old = zeroed();
        if VirtualProtect(
            address as _,
            size,
            new_protection,
            &mut old
        ) == false {
            Err(RadonError::last_error())
        } else {
            Ok(old)
        }
    }
}

/// Tries to allocate memory pages in the target process.
/// On success returns the address of allocated region.
#[rustfmt::skip]
pub fn virtual_allocate(
    address: usize,
    size: usize,
    allocation_type: VIRTUAL_ALLOCATION_TYPE,
    protection: PAGE_PROTECTION_FLAGS,
) -> crate::Result<usize> {
    unsafe {
        let region = VirtualAlloc(
            address as _,
            size,
            allocation_type,
            protection
        );

        if region.is_null() {
            Err(RadonError::last_error())
        } else {
            Ok(region as _)
        }
    }
}

/// Tries to free memory pages in the target process.
#[rustfmt::skip]
pub fn virtual_free(
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
            Err(RadonError::last_error())
        } else {
            Ok(())
        }
    }
}

/// Queries basic information about memory region at `address`.
pub fn virtual_query(address: usize) -> crate::Result<MemoryBasicInformation> {
    unsafe {
        let mut mem_info = zeroed();
        if VirtualQuery(address as _, &mut mem_info, size_of!(@ mem_info)) == 0 {
            Err(RadonError::last_error())
        } else {
            Ok(mem_info.into())
        }
    }
}
