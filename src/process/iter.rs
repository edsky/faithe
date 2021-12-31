use crate::{module::Modules, thread::Threads, RadonError};
use std::mem::size_of;
use windows::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
            TH32CS_SNAPPROCESS,
        },
        Threading::PROCESS_ACCESS_RIGHTS,
    },
};

use super::Process;

/// Basic information about single process.
#[derive(Debug, Clone)]
pub struct ProcessEntry {
    /// Process's id
    pub process_id: u32,
    /// Number of running threads in the process.
    pub cnt_threads: u32,
    /// Id of parent process.
    pub parent_process_id: u32,
    /// Thread priority for any newly created thread.
    pub thread_base_priority: i32,
    /// Name of an executable file.
    pub sz_exe_file: String,
}

impl ProcessEntry {
    /// Returns an iterator over loaded modules in the process.
    pub fn modules(&self) -> crate::Result<Modules> {
        Modules::new(self.process_id)
    }

    /// Returns an iterator over running threads in the process.
    pub fn threads(&self) -> crate::Result<Threads> {
        Threads::new(self.process_id)
    }
}

impl ProcessEntry {
    /// Tries to open this particular process.
    pub fn open(
        &self,
        inherit_handle: bool,
        desired_access: PROCESS_ACCESS_RIGHTS,
    ) -> crate::Result<Process> {
        Process::open_by_id(self.process_id, inherit_handle, desired_access)
    }
}

impl From<PROCESSENTRY32W> for ProcessEntry {
    fn from(pe: PROCESSENTRY32W) -> Self {
        Self {
            process_id: pe.th32ProcessID,
            cnt_threads: pe.cntThreads,
            parent_process_id: pe.th32ParentProcessID,
            thread_base_priority: pe.pcPriClassBase,
            sz_exe_file: String::from_utf16_lossy(
                &pe.szExeFile[..pe.szExeFile.iter().position(|v| *v == 0).unwrap_or(0)],
            ),
        }
    }
}

/// Iterator over all running processes.
pub struct Processes {
    h_snap: HANDLE,
    entry: PROCESSENTRY32W,
    ret: bool,
}

impl Processes {
    /// Creates new iterator over processes
    pub fn new() -> crate::Result<Self> {
        unsafe {
            let h_snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            let entry = PROCESSENTRY32W {
                dwSize: size_of::<PROCESSENTRY32W>() as _,
                ..Default::default()
            };

            let mut this = Self {
                h_snap,
                entry,
                ret: true,
            };
            if Process32FirstW(this.h_snap, &mut this.entry) == false {
                Err(RadonError::last_error())
            } else {
                Ok(this)
            }
        }
    }
}

impl Iterator for Processes {
    type Item = ProcessEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.ret {
            None
        } else {
            let this = self.entry.into();

            unsafe {
                self.ret = Process32NextW(self.h_snap, &mut self.entry) == true;
            }

            Some(this)
        }
    }
}
