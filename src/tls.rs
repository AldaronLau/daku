//! Task local storage

use core::cell::UnsafeCell;

/// Similar to [`LocalKey`](https://doc.rust-lang.org/std/thread/struct.LocalKey.html)
/// but wrapping a [cell](https://doc.rust-lang.org/std/cell/index.html).
#[derive(Debug)]
pub struct Local<T> {
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Local<T> {}

impl<T> Local<T> {
    /// Create new task local storage
    pub const fn new(data: T) -> Self {
        let data = UnsafeCell::new(data);

        Self { data }
    }

    /// Gets a mutable pointer to the wrapped value.
    pub const fn as_ptr(&self) -> *mut T {
        self.data.get()
    }

    /// Similar to
    /// [`LocalKey::with_borrow_mut()`](https://doc.rust-lang.org/std/thread/struct.LocalKey.html#method.with_borrow_mut)
    ///
    /// # Safety
    /// Value must not be currently borrowed.
    #[inline(always)]
    pub unsafe fn with<R>(&'static self, f: impl FnOnce(&mut T) -> R) -> R {
        let data = unsafe { &mut *self.as_ptr() };

        f(data)
    }
}
