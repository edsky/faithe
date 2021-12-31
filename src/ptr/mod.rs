use crate::size_of;
use std::{
    fmt::{Debug, LowerHex, UpperHex},
    marker::PhantomData,
    ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign},
};

/// Pointer type.
/// # Safety
/// This is pure unsafety, don't recomended to use if you don't want to shoot yourself in the foot.
/// Why does it exists? Because.
#[derive(Clone, Copy)]
pub struct Ptr<T> {
    addr: usize,
    _ph: PhantomData<T>,
}

impl<T> Debug for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X}", self.addr)
    }
}

impl<T> LowerHex for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.addr)
    }
}

impl<T> UpperHex for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X}", self.addr)
    }
}

impl<T> Ptr<T> {
    /// Create pointer that pointes to some address.
    #[inline]
    pub fn new(addr: usize) -> Self {
        Self {
            addr,
            _ph: PhantomData::default(),
        }
    }

    /// Returns address the pointer is pointing to.
    #[inline]
    pub fn address(&self) -> usize {
        self.addr
    }

    #[inline]
    /// Returns if internal pointer address equals to 0.
    pub fn is_null(&self) -> bool {
        self.addr == 0
    }

    /// Casts a pointer to other type.
    #[inline]
    pub fn cast<U>(self) -> Ptr<U> {
        Ptr::<U> {
            addr: self.addr,
            _ph: PhantomData::default(),
        }
    }

    /// Increments pointer address by `1`.
    #[inline]
    pub fn inc(&mut self) {
        self.addr += 1;
    }

    /// Increments pointer address by `offset`.
    #[inline]
    pub fn inc_by(&mut self, offset: usize) {
        self.addr += offset;
    }

    /// Decrements pointer address by `1`.
    #[inline]
    pub fn dec(&mut self) {
        self.addr -= 1;
    }

    /// Decrements pointer address by `offset`.
    #[inline]
    pub fn dec_by(&mut self, offset: usize) {
        self.addr -= offset;
    }

    /// Offsets pointer by `offset`
    /// ```
    /// # use radon::ptr::Ptr;
    /// let mut p = Ptr::<u32>::new(0xFF);
    /// p.offset(5);
    /// assert_eq!(p.address(), 0xFF + 5);
    /// p.offset(-10);
    /// assert_eq!(p.address(), 0xFF - 5);
    /// ```
    #[inline]
    pub fn offset(&mut self, offset: isize) {
        self.addr = self.addr.wrapping_add(offset as usize);
    }

    /// Reads the value pointer is pointing to.
    #[inline]
    pub fn read(&self) -> T {
        unsafe { std::ptr::read(self.addr as _) }
    }

    /// Writes value by pointer.
    #[inline]
    pub fn write(&self, val: T) {
        unsafe { *(self.addr as *mut T) = val }
    }

    /// Returns offset from other pointer.
    #[inline]
    pub fn offset_from(&self, other: Ptr<T>) -> isize {
        (other.addr - self.addr) as isize
    }

    /// Swaps values at two pointers.
    #[inline]
    pub fn swap(&self, other: Ptr<T>) {
        unsafe { std::ptr::swap(self.addr as *mut T, other.addr as *mut T) }
    }
}

impl<T> Add<usize> for Ptr<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.addr + rhs * size_of!(T))
    }
}

impl<T> AddAssign<usize> for Ptr<T> {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.addr += rhs * size_of!(T);
    }
}

impl<T> Sub<usize> for Ptr<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(self.addr - rhs * size_of!(T))
    }
}

impl<T> SubAssign<usize> for Ptr<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.addr -= rhs * size_of!(T);
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { crate::to_ref(self.addr as _) }
    }
}

impl<T> DerefMut for Ptr<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { crate::to_mut_ref(self.addr as _) }
    }
}
