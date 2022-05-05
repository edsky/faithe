use super::Processes;
use crate::{
    memory::{MemoryBasicInformation, MemoryProtection},
    module::Modules,
    pattern::{Pattern, PatternSearcher},
    size_of,
    thread::Threads,
    FaitheError,
};
use std::{
    mem::{self, size_of, zeroed},
    ptr::null,
};
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, HINSTANCE},
    System::{
        Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},
        Memory::{
            VirtualAllocEx, VirtualFreeEx, VirtualProtectEx, VirtualQueryEx,
            VIRTUAL_ALLOCATION_TYPE, VIRTUAL_FREE_TYPE,
        },
        ProcessStatus::K32GetModuleFileNameExW,
        Threading::{CreateRemoteThread, GetProcessId, OpenProcess, PROCESS_ACCESS_RIGHTS},
    },
};

/// Represents a handle to a process.
pub struct Process(HANDLE);

impl Process {
    /// Creates process from handle.
    /// # Safety
    /// Passed handle must never be used/closed after its move.
    /// This structure will close handle by itself when dropped.
    pub unsafe fn from_handle(h: HANDLE) -> Self {
        Self(h)
    }

    /// Opens process by it's id.
    pub fn open_by_id(
        id: u32,
        inherit_handle: bool,
        desired_access: PROCESS_ACCESS_RIGHTS,
    ) -> crate::Result<Self> {
        unsafe {
            OpenProcess(desired_access, inherit_handle, id)
                .map_err(|_| FaitheError::last_error())
                .map(|v| Self(v))
        }
    }

    /// Searches for runing processes and opens one if found.
    pub fn open_by_name(
        name: impl AsRef<str>,
        inherit_handle: bool,
        desired_access: PROCESS_ACCESS_RIGHTS,
    ) -> crate::Result<Self> {
        Processes::new()?
            .find_map(|pe| {
                if pe.sz_exe_file == name.as_ref() {
                    Some(pe.open(inherit_handle, desired_access))
                } else {
                    None
                }
            })
            .ok_or(FaitheError::ProcessNotFound)?
    }

    /// Returns an iterator over all modules in the process.
    pub fn modules(&self) -> crate::Result<Modules> {
        Modules::new(self.id())
    }

    /// Returns an iterator over running threads in the process.
    pub fn threads(&self) -> crate::Result<Threads> {
        Threads::new(self.id())
    }

    /// Returns process's id.
    pub fn id(&self) -> u32 {
        unsafe { GetProcessId(self.0) }
    }

    /// Returns the handle to the process.
    /// # Safety
    /// Do not close it and you will be alright.
    pub unsafe fn handle(&self) -> HANDLE {
        self.0
    }

    /// Retrieves full path to process's executable.
    pub fn path(&self) -> crate::Result<String> {
        unsafe {
            let mut buf = [0u16; 256];
            if K32GetModuleFileNameExW(self.0, HINSTANCE::default(), &mut buf) == 0
            {
                Err(FaitheError::last_error())
            } else {
                Ok(String::from_utf16_lossy(
                    &buf[..buf.iter().position(|b| *b == 0).unwrap_or(0)],
                ))
            }
        }
    }

    /// Searches for a specific pattern in the process's module.
    /// Returns `None` if failed to find specified pattern.
    /// Otherwise returns the address of the first occurence.
    pub fn find_pattern(
        &self,
        mod_name: impl AsRef<str>,
        pat: Pattern,
    ) -> crate::Result<Option<usize>> {
        self.modules()?
            .find(|me| me.sz_module == mod_name.as_ref())
            .ok_or(FaitheError::ModuleNotFound)?
            .find_first(pat)
    }

    /// Reads process's memory at address and returns read value.
    pub fn read<T>(&self, address: usize) -> crate::Result<T> {
        unsafe {
            let mut buf = zeroed();
            let mut _read = 0;
            if ReadProcessMemory(
                self.0,
                address as _,
                &mut buf as *mut T as _,
                size_of::<T>(),
                &mut _read,
            ) == false
            {
                Err(FaitheError::last_error())
            } else {
                Ok(buf)
            }
        }
    }

    /// Reads process's memory at address and returns read value and amount of bytes read.
    pub fn read_ext<T>(&self, address: usize) -> crate::Result<(T, usize)> {
        unsafe {
            let mut buf = zeroed();
            let mut read = 0;
            if ReadProcessMemory(
                self.0,
                address as _,
                &mut buf as *mut T as _,
                size_of::<T>(),
                &mut read,
            ) == false
            {
                Err(FaitheError::last_error())
            } else {
                Ok((buf, read))
            }
        }
    }

    /// Reads process's memory at address and copy `buf.len()` bytes into buffer.
    /// Returns the amount of bytes read.
    pub fn read_buf(&self, address: usize, mut buf: impl AsMut<[u8]>) -> crate::Result<usize> {
        unsafe {
            let mut read = 0;
            if ReadProcessMemory(
                self.0,
                address as _,
                buf.as_mut().as_mut_ptr() as _,
                buf.as_mut().len(),
                &mut read,
            ) == false
            {
                Err(FaitheError::last_error())
            } else {
                Ok(read)
            }
        }
    }

    /// Writes process's memory at address by copying value into the target memory.
    /// Returns the amount of bytes written.
    pub fn write<T>(&self, address: usize, value: T) -> crate::Result<usize>
    where
        T: Clone,
    {
        unsafe {
            let mut written = 0;
            if WriteProcessMemory(
                self.0,
                address as _,
                &value as *const T as _,
                size_of::<T>(),
                &mut written,
            ) == false
            {
                Err(FaitheError::last_error())
            } else {
                Ok(written)
            }
        }
    }

    /// Writes process's memory at address by coping while buffer into the target memory.
    /// Returns the amount of bytes written.
    pub fn write_ext(
        &self,
        address: usize,
        written: &mut usize,
        buf: impl AsRef<[u8]>,
    ) -> crate::Result<()> {
        unsafe {
            if WriteProcessMemory(
                self.0,
                address as _,
                buf.as_ref().as_ptr() as _,
                buf.as_ref().len(),
                written,
            ) == false
            {
                Err(FaitheError::last_error())
            } else {
                Ok(())
            }
        }
    }

    /// Writes process's memory at address by copying whole buffer into the target memory.
    /// Returns the amount of bytes written.
    pub fn write_buf(&self, address: usize, buf: impl AsRef<[u8]>) -> crate::Result<usize> {
        unsafe {
            let mut written = 0;
            if WriteProcessMemory(
                self.0,
                address as _,
                buf.as_ref().as_ptr() as _,
                buf.as_ref().len(),
                &mut written,
            ) == false
            {
                Err(FaitheError::last_error())
            } else {
                Ok(written)
            }
        }
    }

    /// Changes the protection of memory pages of the target process.
    /// For more info see [microsoft documentation](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualprotectex).
    #[rustfmt::skip]
    pub fn protect(
        &self,
        address: usize,
        size: usize,
        new_protection: MemoryProtection,
    ) -> crate::Result<MemoryProtection> {
        unsafe {
            let mut old = zeroed();
            if VirtualProtectEx(
                self.0,
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
        &self,
        address: usize,
        size: usize,
        allocation_type: VIRTUAL_ALLOCATION_TYPE,
        protection: MemoryProtection,
    ) -> crate::Result<usize> {
        unsafe {
            let region = VirtualAllocEx(
                self.0,
                address as _,
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
        &self,
        address: usize,
        size: usize,
        free_type: VIRTUAL_FREE_TYPE
    ) -> crate::Result<()>
    {
        unsafe {
            if VirtualFreeEx(
                self.0,
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
    pub fn query(&self, address: usize) -> crate::Result<MemoryBasicInformation> {
        unsafe {
            let mut mem_info = zeroed();
            if VirtualQueryEx(self.0, address as _, &mut mem_info, size_of!(@ mem_info)) == 0 {
                Err(FaitheError::last_error())
            } else {
                Ok(mem_info.into())
            }
        }
    }

    /// Creates remote thread in the process.
    /// On success returns thread's handle and it's thread id.
    pub fn create_remote_thread<T>(
        &self,
        address: usize,
        param: *const T,
    ) -> crate::Result<(HANDLE, u32)> {
        unsafe {
            let mut tid = 0;
            CreateRemoteThread(
                self.0,
                null(),
                0,
                mem::transmute(address),
                param as _,
                0,
                &mut tid,
            ).map_err(|_| FaitheError::last_error())
                .map(|v| (v, tid))
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}
