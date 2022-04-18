# Faithe
Memory hacking library for windows.

# Instalation
```toml
# Latest version
[dependencies]
faithe = "0.7.0"

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

// Creates a trait that will emulate behavior of virtual functions in C++.
struct CPlayer;
interface! {
    trait IEntity(CPlayer) {
        extern "C" fn get_health() -> i32 = 0;
        extern "C" fn set_health(new: i32) = 1;
    }
}
/*
class CPlayer {
    virtual int get_health() = 0;
    virtual void set_health(int new_value) = 0;
};
*/

// Creates struct with explicitly defined offsets.
xstruct! {
    // STRUCT HAS SIZE OF ZERO.
    struct Foo {
        0x0 @ a: u32,
        0x16 @ b: bool
    }

    // STRUCT HAS SIZE 20.
    struct Bar(20) {
        0x0 @ a: u32,
        0x16 @ b: bool
    }
}

// Creates a function with explicitly defined RVA relative to some module.
function! {
    // Explicitly defined RVA offset relative to `01-hello` module.
    extern FUNC: extern "C" fn(a: i32) = "01-hello.exe"@0x1900;
}
FUNC.call(5);
```
