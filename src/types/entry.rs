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
