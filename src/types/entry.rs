// @TODO: Make an iterator over ListEntry.

/// Link of Doubly-linked list.
#[derive(Debug)]
#[repr(C)]
pub struct ListEntry<T> {
    /// Next
    pub flink: *mut T,
    /// Previous
    pub blink: *mut T,
}

impl<T> Clone for ListEntry<T> {
    fn clone(&self) -> Self {
        Self {
            flink: self.flink,
            blink: self.blink,
        }
    }
}

impl<T> Copy for ListEntry<T> {}

/// Resolves next link in doubly-linked list.
#[macro_export]
macro_rules! containing_record {
    ($next:expr, $type:ty, $field:tt) => {
        $next.flink.cast::<u8>().sub(memoffset::offset_of!($type, $field)).cast::<$type>()
    };
}

/// Creates an iterator over doubly-linked list.
#[macro_export]
macro_rules! list_iter {
    ($head:expr, $type:ty, $field:tt) => {
        {
            let __first = $crate::containing_record!($head, $type, $field);
            let mut __next = __first;
            let mut __started = false;
            core::iter::from_fn(move || {
                if __first == __next && __started {
                    None
                } else {
                    __started = true;
                    
                    let val = __next.read();
                    __next = $crate::containing_record!((*__next).$field, $type, $field);
                    Some(val)
                }
            })
        }
    };
}