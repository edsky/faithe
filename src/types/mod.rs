
cfg_if::cfg_if! {
    if #[cfg(not(feature = "no-std"))] {
        mod winapi;
        pub use winapi::*;
    }
}

mod entry;
pub use entry::*;
