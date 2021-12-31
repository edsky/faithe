use std::ptr::null_mut;
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

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
