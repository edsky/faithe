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

/// Macro for interal use. Provides functionality to hide panic messages if needed.
#[doc(hidden)]
#[macro_export]
macro_rules! __expect {
    ($var:expr, $msg:expr) => {
        if cfg!(feature = "no-msgs") {
            $var.unwrap()
        } else {
            $var.expect($msg)
        }
    };
}