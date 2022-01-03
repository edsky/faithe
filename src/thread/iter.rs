use crate::{size_of, RadonError};
use windows::{
    core::Handle,
    Win32::{
        Foundation::HANDLE,
        System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
        },
    },
};

/// Represents single running thread in a process.
#[derive(Debug, Clone)]
pub struct ThreadEntry {
    /// Id of the process this thread is running in.
    pub process_id: u32,
    /// Id of the thread.
    pub thread_id: u32,
    /// Priority of the thread.
    pub base_priority: i32,
}

impl From<THREADENTRY32> for ThreadEntry {
    fn from(te: THREADENTRY32) -> Self {
        Self {
            process_id: te.th32OwnerProcessID,
            thread_id: te.th32ThreadID,
            base_priority: te.tpBasePri,
        }
    }
}

/// Iterator over running threads in the process.
pub struct Threads {
    h_snap: HANDLE,
    entry: THREADENTRY32,
    ret: bool,
}

impl Threads {
    /// Creates new iterator over threads in process with id `process_id`.
    pub fn new(process_id: u32) -> crate::Result<Self> {
        unsafe {
            let h_snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, process_id);
            if h_snap.is_invalid() {
                return Err(RadonError::last_error());
            }

            let entry = THREADENTRY32 {
                dwSize: size_of!(THREADENTRY32) as _,
                ..Default::default()
            };

            let mut this = Self {
                h_snap,
                entry,
                ret: true,
            };

            if Thread32First(h_snap, &mut this.entry) == false {
                Err(RadonError::last_error())
            } else {
                Ok(this)
            }
        }
    }
}

impl Iterator for Threads {
    type Item = ThreadEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.ret {
            None
        } else {
            let this = self.entry.into();

            unsafe {
                self.ret = Thread32Next(self.h_snap, &mut self.entry) == true;
            }

            Some(this)
        }
    }
}
