# Radon
Memory hacking library for windows.

# Instalation
```toml
[dependencies.radon]
git = "https://github.com/sy1ntexx/radon
```

# Opening processes
```rust
use radon::types::access_rights::PROCESS_ALL_ACCESS;
use radon::process as ps;

let process = ps::Processes::new()?
    .find(|p| p.sz_exe_file == "Process name.exe")
    .unwrap()
    .open(false, PROCESS_ALL_ACCESS)?;
```

# Modules iterating
```rust
let process = get_process();
process
    .modules()?
    .for_each(|m| dbg!(m));
```

# Reading / Writing memory
```rust
let process = get_process();
let mut value = process.read_process_memory::<u32>(0xFF)?;
value += 100;

process.write_process_memory(0xFF, value)?;
```

# Allocating / Freeing / Protecting / Querying memory
```rust
use radon::types::protection_flags::{PAGE_EXECUTE_READWRITE, PAGE_READONLY};
use radon::types::allocation_types::{MEM_COMMIT, MEM_RESERVE};
use radon::types::free_types::MEM_RELEASE;

let process = get_process();
let mut chunk = process.virtual_allocate(
    0,
    1000,
    MEM_COMMIT | MEM_RESERVE,
    PAGE_EXECUTE_READWRITE
)?;
let info = process.virtual_query(chunk)?;

process.virtual_protect(chunk, 1000, PAGE_READONLY)?;
process.virtual_free(chunk, 0, MEM_RELEASE)?;
```

# Searching for patterns
```rust
use radon::pattern::Pattern;

let process = get_process();
let address = process.find_pattern(
    "Something.exe",
    Pattern::from_ida_style("48 89 85 F0 00 00 00 4C 8B ? ? ? ? ? 48 8D")
)?;
```

# Macros
```rust
use radon::{interface, xstruct};

struct CEntity;

interface! {
    trait IEntity {
        0 @ fn get_health() -> u32;
        1 @ fn set_health(new_value: u32);
    }
    impl for CEntity;
}

xstruct! {
    struct CPlayer {
        0x100 @ health: u32,
        0x250 @ stamina: f32
    }
}
```