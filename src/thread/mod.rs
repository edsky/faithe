use std::mem::zeroed;

use crate::{size_of, FaitheError};
use windows::Win32::{
    Foundation::{HANDLE, CloseHandle},
    System::{
        Diagnostics::Debug::{GetThreadContext, SetThreadContext},
        Threading::{
            NtQueryInformationThread, OpenThread, ResumeThread, SuspendThread, THREADINFOCLASS,
            THREAD_ACCESS_RIGHTS,
        },
    },
};

pub use windows::Win32::System::Diagnostics::Debug::CONTEXT;

mod iter;
pub use iter::*;

/// Represents a handle to a thread.
pub struct OwnedThread(HANDLE);

impl OwnedThread {
    /// Tries to open thread by its id.
    pub fn open(
        thread_id: u32,
        inherit_handle: bool,
        desired_access: THREAD_ACCESS_RIGHTS,
    ) -> crate::Result<Self> {
        unsafe {
            OpenThread(desired_access, inherit_handle, thread_id)
                .map_err(|_| FaitheError::last_error())
                .map(|v| Self(v))
        }
    }

    /// Returns the handle to the thread.
    /// # Safety
    /// Do not close it until [`OwnedThread`] is in use.
    pub unsafe fn handle(&self) -> HANDLE {
        self.0
    }

    /// Converts [`OwnedThread`] into inner `HANDLE`.
    pub fn into_handle(self) -> HANDLE {
        self.0
    }

    /// Returns the start address of the thread
    pub fn start_address(&self) -> crate::Result<usize> {
        let mut addr = 0;
        unsafe {
            NtQueryInformationThread(
                self.0,
                THREADINFOCLASS(9), // ThreadQuerySetWin32StartAddress
                &mut addr as *mut _ as _,
                size_of!(usize) as _,
                0 as _,
            )?;
        }
        Ok(addr)
    }

    /// Tries to suspend the thread.
    /// On success returns the previous suspend count.
    pub fn suspend(&self) -> crate::Result<u32> {
        unsafe {
            match SuspendThread(self.0) {
                u32::MAX => Err(FaitheError::last_error()),
                sus => Ok(sus),
            }
        }
    }

    /// Tries to resume the thread.
    /// On success returns the previous suspend count.
    pub fn resume(&self) -> crate::Result<u32> {
        unsafe {
            match ResumeThread(self.0) {
                u32::MAX => Err(FaitheError::last_error()),
                sus => Ok(sus),
            }
        }
    }

    /// Returns the context of the thread.
    /// For more info see [microsoft documentation](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadcontext)
    pub fn get_context(&self) -> crate::Result<CONTEXT> {
        unsafe {
            let mut ctx = zeroed();
            if GetThreadContext(self.0, &mut ctx) == false {
                Err(FaitheError::last_error())
            } else {
                Ok(ctx)
            }
        }
    }

    /// Sets the context for the thread.
    /// For more info see [microsoft documentation](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadcontext)
    pub fn set_context(&self, ctx: &CONTEXT) -> crate::Result<()> {
        unsafe {
            if SetThreadContext(self.0, ctx as _) == false {
                Err(FaitheError::last_error())
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for OwnedThread {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}