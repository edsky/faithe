use windows::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
        },
        Threading::THREAD_ACCESS_RIGHTS,
    },
};
use crate::{size_of, FaitheError};
use super::OwnedThread;

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

impl ThreadEntry {
    /// Opens thread
    pub fn open(
        &self,
        inherit_handle: bool,
        desired_access: THREAD_ACCESS_RIGHTS,
    ) -> crate::Result<OwnedThread> {
        OwnedThread::open(self.thread_id, inherit_handle, desired_access)
    }
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
pub struct ThreadIterator {
    snapshot: HANDLE,
    entry: THREADENTRY32,
    process_id: u32,
    should_return: bool,
}

impl ThreadIterator {
    /// Creates new iterator over threads in process with id `process_id`.
    pub fn new(process_id: u32) -> crate::Result<Self> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, process_id)
                .map_err(|_| FaitheError::last_error())?;

            let entry = THREADENTRY32 {
                dwSize: size_of!(THREADENTRY32) as _,
                ..Default::default()
            };

            let mut this = Self {
                should_return: false,
                process_id,
                snapshot,
                entry,
            };

            if Thread32First(snapshot, &mut this.entry) == false {
                Err(FaitheError::last_error())
            } else {
                Ok(this)
            }
        }
    }
}

impl Iterator for ThreadIterator {
    type Item = ThreadEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.should_return {
            None
        } else {
            let mut this: ThreadEntry = self.entry.into();

            unsafe {
                self.should_return = Thread32Next(self.snapshot, &mut self.entry) == false;

                while this.process_id != self.process_id {
                    this = self.entry.into();
                    self.should_return = Thread32Next(self.snapshot, &mut self.entry) == false;
                    if self.should_return {
                        return None;
                    }
                }
            }

            Some(this)
        }
    }
}
