use std::mem::zeroed;

use crate::RadonError;
use windows::{
    core::Handle,
    Win32::{
        Foundation::HANDLE,
        System::{
            Diagnostics::Debug::{GetThreadContext, SetThreadContext},
            Threading::{OpenThread, ResumeThread, SuspendThread, THREAD_ACCESS_RIGHTS},
        },
    },
};

pub use windows::Win32::System::Diagnostics::Debug::CONTEXT;

mod iter;
pub use iter::*;

/// Represents a handle to a thread.
pub struct Thread(HANDLE);

impl Thread {
    /// Tries to open thread by its id.
    pub fn open(
        thread_id: u32,
        inherit_handle: bool,
        desired_access: THREAD_ACCESS_RIGHTS,
    ) -> crate::Result<Self> {
        unsafe {
            let handle = OpenThread(desired_access, inherit_handle, thread_id);

            if handle.is_invalid() {
                Err(RadonError::last_error())
            } else {
                Ok(Self(handle))
            }
        }
    }

    /// Tries to suspend the thread.
    /// On success returns the previous suspend count.
    pub fn suspend(&self) -> crate::Result<u32> {
        unsafe {
            match SuspendThread(self.0) {
                u32::MAX => Err(RadonError::last_error()),
                sus => Ok(sus),
            }
        }
    }

    /// Tries to resume the thread.
    /// On success returns the previous suspend count.
    pub fn resume(&self) -> crate::Result<u32> {
        unsafe {
            match ResumeThread(self.0) {
                u32::MAX => Err(RadonError::last_error()),
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
                Err(RadonError::last_error())
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
                Err(RadonError::last_error())
            } else {
                Ok(())
            }
        }
    }
}
