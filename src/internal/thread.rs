use super::Peb;
use std::ptr::null_mut;
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

type ThreadInit<T> = unsafe extern "system" fn(Option<Box<T>>) -> u32;

/// Creates new thread with default parameters.
/// # Panics
/// If failed to create a new thread.
pub fn create_thread<T>(init: ThreadInit<T>, param: Option<T>) -> u32 {
    unsafe {
        let mut t_id = 0;
        CreateThread(
            null_mut(),
            0,
            Some(std::mem::transmute(init)),
            param
                .map(|p| Box::into_raw(Box::new(p)))
                .unwrap_or(null_mut()) as _,
            THREAD_CREATION_FLAGS::default(),
            &mut t_id,
        ).unwrap();
        t_id
    }
}

/// Thread Environmental Block.
#[repr(C)]
pub struct Teb {
    _pad: [u8; 0x60],
    /// Reference to process environmental block.
    pub process_environmental_block: &'static Peb,
}

/// Returns an address of TEB(Thread Environmental Block).
#[cfg(feature = "nightly")]
#[inline(always)]
pub fn get_teb<'a>() -> &'a Teb {
    unsafe {
        let mut teb: usize;
        std::arch::asm! {
            "mov {}, GS:[30h]",
            out(reg) teb,
        };
        crate::to_ref(teb as _)
    }
}
