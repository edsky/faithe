mod interface;
mod sizeof;
mod strings;
mod xstruct;

cfg_if::cfg_if! {
    if #[cfg(not(feature = "no-std"))] {
        mod function;
        pub use function::*;
    }
}
