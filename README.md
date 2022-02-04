# Faithe
Memory hacking library for windows.

# Instalation
```toml
# Latest version
[dependencies]
faithe = "0.3.0"

# Development version
[dependencies.faithe]
git = "https://github.com/sy1ntexx/faithe"
```

# Opening processes
```rust
use faithe::types::access_rights::PROCESS_ALL_ACCESS;
use faithe::process as ps;

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
use faithe::types::protection_flags::{PAGE_EXECUTE_READWRITE, PAGE_READONLY};
use faithe::types::allocation_types::{MEM_COMMIT, MEM_RESERVE};
use faithe::types::free_types::MEM_RELEASE;

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
use faithe::pattern::Pattern;

let process = get_process();
let address = process.find_pattern(
    "Something.exe",
    // Available styles: IDA, Code, PiDB
    Pattern::from_ida_style("48 89 85 F0 00 00 00 4C 8B ? ? ? ? ? 48 8D")
)?;
```

# Macros
```rust
use faithe::{interface, xstruct};

struct CEntity;

// Creates a trait that will emulate behavior of virtual functions in C++.
interface! {
    trait IEntity {
        0 @ fn get_health() -> u32;
        1 @ fn set_health(new_value: u32);
    }
    impl for CEntity;
    /*
    class IEntity {
        virtual int get_health() = 0;
        virtual void set_health(int new_value) = 0;
    };
    */
}

// Creates struct with explicitly defined offsets.
xstruct! {
    struct CPlayer {
        // health will be availble at offset 0x100
        0x100 @ health: u32,
        // stamina will be availble at offset 0x100
        0x250 @ stamina: f32
    }
}
```
