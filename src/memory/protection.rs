bitflags::bitflags! {
    #[doc = "Specifies memory page protection on different platforms."]
    pub struct MemoryProtection: u32 {
        #[doc = "No access."]
        const NONE = 0b000;

        #[doc = "Read access."]
        const READ = 0b001;

        #[doc = "Read write access."]
        const READ_WRITE = 0b011;

        #[doc = "Read execute access."]
        const READ_EXECUTE = 0b101;

        #[doc = "Write access."]
        const WRITE = 0b010;

        #[doc = "Execute access."]
        const EXECUTE = 0b100;

        #[doc = "Read write execute access."]
        const READ_WRITE_EXECUTE = 0b111;
    }
}

impl MemoryProtection {
    /// Converts memory protection to the corresponding os dependent value.
    /// `Windows` convertion works as follows:
    /// - `---` => PAGE_NOACCESS,
    /// - `--x` => PAGE_EXECUTE,
    /// - `-w-` => PAGE_WRITECOPY,
    /// - `-wx` => PAGE_EXECUTE_WRITECOPY,
    /// - `r--` => PAGE_READONLY,
    /// - `r-x` => PAGE_EXECUTE_READ,
    /// - `rw-` => PAGE_READWRITE,
    /// - `rwx` => PAGE_EXECUTE_READWRITE,
    #[cfg(windows)]
    #[cfg(not(feature = "no-std"))]
    pub fn to_os(&self) -> windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS {
        use windows::Win32::System::Memory::{
            PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY,
            PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE, PAGE_WRITECOPY,
        };
        // Damn I hate windows
        match (
            self.contains(Self::READ),
            self.contains(Self::WRITE),
            self.contains(Self::EXECUTE),
        ) {
            (false, false, false) => PAGE_NOACCESS,
            (false, false, true) => PAGE_EXECUTE,
            (false, true, false) => PAGE_WRITECOPY,
            (false, true, true) => PAGE_EXECUTE_WRITECOPY,
            (true, false, false) => PAGE_READONLY,
            (true, false, true) => PAGE_EXECUTE_READ,
            (true, true, false) => PAGE_READWRITE,
            (true, true, true) => PAGE_EXECUTE_READWRITE,
        }
    }
    /// Converts memory protection to the corresponding os dependent value.
    /// `Unix` convertion works as follows:
    /// `Self::READ` = PROT_READ.
    /// `Self::WRITE` = PROT_WRITE.
    /// `Self::EXECUTE` = PROT_EXEC.
    #[cfg(unix)]
    pub fn to_os(&self) -> i32 {
        self.bits as i32
    }

    /// Converts PAGE_PROTECTION_FLAGS value to [`MemoryProtection`].
    /// For conversions see [`Self::to_os`].
    /// For any other protection `None` is returned.
    #[cfg(windows)]
    #[cfg(not(feature = "no-std"))]
    pub fn from_os(prot: windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS) -> Option<Self> {
        use windows::Win32::System::Memory::{
            PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY,
            PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE, PAGE_WRITECOPY,
        };

        match prot {
            PAGE_NOACCESS => Some(Self::NONE),
            PAGE_EXECUTE => Some(Self::EXECUTE),
            PAGE_WRITECOPY => Some(Self::WRITE),
            PAGE_EXECUTE_WRITECOPY => Some(Self::WRITE | Self::EXECUTE),
            PAGE_READONLY => Some(Self::READ),
            PAGE_EXECUTE_READ => Some(Self::READ_EXECUTE),
            PAGE_READWRITE => Some(Self::READ_WRITE),
            PAGE_EXECUTE_READWRITE => Some(Self::READ_WRITE_EXECUTE),
            _ => None,
        }
    }

    /// Converts unix protection to [`MemoryProtection`].
    /// For conversions see [`Self::to_os`].
    /// For any other protection `None` is returned.
    #[cfg(unix)]
    pub fn from_os(prot: i32) -> Option<Self> {
        Self::from_bits(prot)
    }
}
