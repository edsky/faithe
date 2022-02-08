use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};
use std::ptr::null_mut;
use super::PEB;

type ThreadInit<T> = unsafe extern "system" fn(Option<Box<T>>) -> u32;

/// Creates new thread with default parameters.
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
        );
        t_id
    }
}

/// Thread Environmental Block.
#[repr(C)]
pub struct TEB {
    _pad: [u8; 0x60],
    /// Reference to process environmental block.
    pub process_environmental_block: &'static PEB
}

/// Returns an address of TEB(Thread Environmental Block).
#[cfg(feature = "nightly")]
#[inline(always)]
pub fn get_teb<'a>() -> &'a TEB {
    unsafe {
        let mut teb: usize;
        std::arch::asm! {
            "mov {}, GS:[30h]",
            out(reg) teb,
        };
        crate::to_ref(teb as _)
    }
}