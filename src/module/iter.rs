use crate::FaitheError;
use std::mem::size_of;
use windows::Win32::{
    Foundation::{HANDLE, HINSTANCE},
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W, TH32CS_SNAPMODULE,
        TH32CS_SNAPMODULE32,
    },
};

/// Represents a single module in a running process.
#[derive(Debug, Clone)]
pub struct ModuleEntry {
    /// Id of the process.
    pub process_id: u32,
    /// Base address of the module.
    pub base_address: usize,
    /// Size of the module in bytes.
    pub size: usize,
    /// Handle to the module.
    pub handle: HINSTANCE,
    /// Name of the module.
    pub name: String,
    /// Full path to the module.
    pub path: String,
}

impl From<MODULEENTRY32W> for ModuleEntry {
    fn from(me: MODULEENTRY32W) -> Self {
        Self {
            process_id: me.th32ProcessID,
            base_address: me.modBaseAddr as _,
            size: me.modBaseSize as _,
            handle: me.hModule,
            name: String::from_utf16_lossy(
                &me.szModule[..me.szModule.iter().position(|b| *b == 0).unwrap_or(0)],
            ),
            path: String::from_utf16_lossy(
                &me.szExePath[..me.szExePath.iter().position(|b| *b == 0).unwrap_or(0)],
            ),
        }
    }
}

/// Iterator over process's loaded modules.
pub struct ModuleIterator {
    snap: HANDLE,
    entry: MODULEENTRY32W,
    ret: bool,
}

impl ModuleIterator {
    /// Creates new iterator over modules of process with id `process_id`
    pub fn new(process_id: u32) -> crate::Result<Self> {
        unsafe {
            let snap =
                CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id)
                    .map_err(|_| FaitheError::last_error())?;

            let entry = MODULEENTRY32W {
                dwSize: size_of::<MODULEENTRY32W>() as _,
                ..Default::default()
            };

            let mut this = Self {
                snap,
                entry,
                ret: true,
            };

            if Module32FirstW(snap, &mut this.entry) == false {
                Err(FaitheError::last_error())
            } else {
                Ok(this)
            }
        }
    }
}

impl Iterator for ModuleIterator {
    type Item = ModuleEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.ret {
            None
        } else {
            let this = self.entry.into();

            unsafe {
                self.ret = Module32NextW(self.snap, &mut self.entry) == true;
            }

            Some(this)
        }
    }
}
