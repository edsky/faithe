use super::Process;
use windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS;

#[derive(Debug)]
/// Allocated memory page
pub struct MemoryRegion {
    /// Start of the page
    pub start: usize,
    /// End of the region
    pub end: usize,
    /// Size of the region
    pub size: usize,
    /// Protection of the region
    pub protection: PAGE_PROTECTION_FLAGS,
    /// Initial protection of the region
    pub initial: PAGE_PROTECTION_FLAGS,
}

/// Iterator over process's memory regions
pub struct MemoryRegionIter<'a> {
    proc: &'a Process,
    current: usize,
}

impl<'a> MemoryRegionIter<'a> {
    /// Creates new iterator over process's memory regions.
    pub fn new(proc: &'a Process) -> Self {
        Self { current: 0, proc }
    }
}

impl<'a> Iterator for MemoryRegionIter<'a> {
    type Item = MemoryRegion;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = self.proc.query_memory(self.current).ok()?;
        while chunk.state.0 == 0x10000 {
            self.current = chunk.base_address + chunk.region_size;
            chunk = self.proc.query_memory(self.current).ok()?;
        }
        let region = MemoryRegion {
            start: chunk.base_address,
            end: chunk.base_address + chunk.region_size,
            size: chunk.region_size,
            protection: chunk.protection,
            initial: chunk.alloc_protection,
        };
        self.current = chunk.base_address + chunk.region_size;
        Some(region)
    }
}
