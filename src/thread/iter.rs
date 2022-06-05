use crate::{size_of, FaitheError};
use windows::Win32::{
    Foundation::HANDLE,
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
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
    snap: HANDLE,
    entry: THREADENTRY32,
    ret: bool,
}

impl Threads {
    /// Creates new iterator over threads in process with id `process_id`.
    pub fn new(process_id: u32) -> crate::Result<Self> {
        unsafe {
            let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, process_id)
                .map_err(|_| FaitheError::last_error())?;
            let entry = THREADENTRY32 {
                dwSize: size_of!(THREADENTRY32) as _,
                ..Default::default()
            };

            let mut this = Self {
                snap,
                entry,
                ret: true,
            };

            if Thread32First(snap, &mut this.entry) == false {
                Err(FaitheError::last_error())
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
                self.ret = Thread32Next(self.snap, &mut self.entry) == true;
            }

            Some(this)
        }
    }
}
