use crate::RadonError;
use std::mem::size_of;
use windows::{
    Win32::{
        Foundation::{HANDLE, HINSTANCE},
        System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W,
            TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
        },
    },
};

/// Represents a single module in a running process.
#[derive(Debug, Clone)]
pub struct ModuleEntry {
    /// Id of the process.
    pub process_id: u32,
    /// Base address of the module.
    pub mod_base_addr: usize,
    /// Size of the module in bytes.
    pub mod_base_size: usize,
    /// Handle to the module.
    pub h_module: HINSTANCE,
    /// Name of the module.
    pub sz_module: String,
    /// Full path to the module.
    pub sz_exe_path: String,
}

impl From<MODULEENTRY32W> for ModuleEntry {
    fn from(me: MODULEENTRY32W) -> Self {
        Self {
            process_id: me.th32ProcessID,
            mod_base_addr: me.modBaseAddr as _,
            mod_base_size: me.modBaseSize as _,
            h_module: me.hModule,
            sz_module: String::from_utf16_lossy(
                &me.szModule[..me.szModule.iter().position(|b| *b == 0).unwrap_or(0)],
            ),
            sz_exe_path: String::from_utf16_lossy(
                &me.szExePath[..me.szExePath.iter().position(|b| *b == 0).unwrap_or(0)],
            ),
        }
    }
}

/// Iterator over process's loaded modules.
pub struct Modules {
    h_snap: HANDLE,
    entry: MODULEENTRY32W,
    ret: bool,
}

impl Modules {
    /// Creates new iterator over modules of process with id `process_id`
    pub fn new(process_id: u32) -> crate::Result<Self> {
        unsafe {
            let h_snap =
                CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id);

            if h_snap.is_invalid() {
                return Err(RadonError::last_error());
            }

            let entry = MODULEENTRY32W {
                dwSize: size_of::<MODULEENTRY32W>() as _,
                ..Default::default()
            };

            let mut this = Self {
                h_snap,
                entry,
                ret: true,
            };

            if Module32FirstW(h_snap, &mut this.entry) == false {
                Err(RadonError::last_error())
            } else {
                Ok(this)
            }
        }
    }
}

impl Iterator for Modules {
    type Item = ModuleEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.ret {
            None
        } else {
            let this = self.entry.into();

            unsafe {
                self.ret = Module32NextW(self.h_snap, &mut self.entry) == true;
            }

            Some(this)
        }
    }
}
