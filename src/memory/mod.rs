use windows::Win32::System::Memory::{
    MEMORY_BASIC_INFORMATION, PAGE_PROTECTION_FLAGS, PAGE_TYPE, VIRTUAL_ALLOCATION_TYPE,
};

/// Basic information about memory region.
#[derive(Debug, Clone)]
pub struct MemoryBasicInformation {
    /// Base address of region.
    pub base_address: usize,
    /// Base address of allocated memory.
    pub alloc_base: usize,
    /// Initial protection of allocated pages.
    pub alloc_protection: PAGE_PROTECTION_FLAGS,
    /// Size of region in bytes.
    pub region_size: usize,
    /// Current state of memory region.
    pub state: VIRTUAL_ALLOCATION_TYPE,
    /// Current protection of memory region.
    pub protection: PAGE_PROTECTION_FLAGS,
    /// Type of allocated memory.
    pub memory_type: PAGE_TYPE,
}

impl From<MEMORY_BASIC_INFORMATION> for MemoryBasicInformation {
    fn from(v: MEMORY_BASIC_INFORMATION) -> Self {
        Self {
            base_address: v.BaseAddress as _,
            alloc_base: v.AllocationBase as _,
            alloc_protection: v.AllocationProtect as _,
            region_size: v.RegionSize,
            state: v.State,
            protection: v.Protect,
            memory_type: v.Type,
        }
    }
}

/// Resolves multilevel pointer.
/// # Behavior
/// It begins from adding to base first offset and reading a value on this address, assigns to
/// base readed value and so on.
pub unsafe fn follow_pointer_path<const I: usize, T>(
    mut base: *const u8,
    offsets: [usize; I],
) -> *const T {
    for offset in &offsets {
        base = *((base as usize + *offset) as *const usize) as _;
    }
    base as _
}
