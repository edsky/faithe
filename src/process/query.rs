use windows::Win32::System::Memory::{PAGE_READONLY, PAGE_READWRITE, PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_READ, PAGE_EXECUTE_WRITECOPY, PAGE_WRITECOPY, PAGE_EXECUTE, PAGE_PROTECTION_FLAGS, PAGE_NOACCESS};

use super::Process;

/// Allows to easily query process memory.
pub struct Query<'a>(pub(crate) &'a Process);
impl<'a> Query<'a> {
    /// Checks if it's possible to read memory at the address.
    #[inline]
    pub fn read_at(&self, addr: usize) -> bool {
        if let Ok(mem) = self.0.query_memory(addr) {
            return 
                (mem.protection.0 & PAGE_READONLY.0 != 0) ||
                (mem.protection.0 & PAGE_READWRITE.0 != 0) ||
                (mem.protection.0 & PAGE_EXECUTE_READ.0 != 0) ||
                (mem.protection.0 & PAGE_EXECUTE_READWRITE.0 != 0) ||
                (mem.protection.0 & PAGE_EXECUTE_WRITECOPY.0 != 0)
        }
        false
    }

    /// Checks if it's possible to write memory to the address.
    #[inline]
    pub fn write_at(&self, addr: usize) -> bool {
        if let Ok(mem) = self.0.query_memory(addr) {
            return 
                (mem.protection.0 & PAGE_WRITECOPY.0 != 0) ||
                (mem.protection.0 & PAGE_READWRITE.0 != 0) ||
                (mem.protection.0 & PAGE_EXECUTE_READWRITE.0 != 0) ||
                (mem.protection.0 & PAGE_EXECUTE_WRITECOPY.0 != 0)
        }
        false
    }

    /// Checks if it's possible to execute memory at the address.
    #[inline]
    pub fn execute_at(&self, addr: usize) -> bool {
        if let Ok(mem) = self.0.query_memory(addr) {
            return 
                (mem.protection.0 & PAGE_EXECUTE.0 != 0) ||
                (mem.protection.0 & PAGE_EXECUTE_READ.0 != 0) ||
                (mem.protection.0 & PAGE_EXECUTE_READWRITE.0 != 0) ||
                (mem.protection.0 & PAGE_EXECUTE_WRITECOPY.0 != 0)
        }
        false
    }

    /// Returns the start of the next allocated chunk
    #[inline]
    pub fn boundary(&self, addr: usize) -> usize {
        if let Ok(mem) = self.0.query_memory(addr) {
            return mem.alloc_base + mem.region_size;
        }
        0
    }

    /// Returns the protection of the memory
    #[inline]
    pub fn access(&self, addr: usize) -> PAGE_PROTECTION_FLAGS {
        self.0.query_memory(addr).map(|m| m.protection).unwrap_or(PAGE_NOACCESS)
    }
}