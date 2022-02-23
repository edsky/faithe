
cfg_if::cfg_if! {
    if #[cfg(not(feature = "no-std"))] {
        mod winapi;
        pub use winapi::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(any(not(feature = "no-std"), feature = "alloc"))] {
        mod string;
        pub use string::UnicodeString;
    }
}

mod entry;
pub use entry::*;
