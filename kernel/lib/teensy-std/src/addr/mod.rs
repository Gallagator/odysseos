use core::{num::NonZeroUsize, ptr::NonNull};

#[derive(Debug)]
pub struct Addr {
    inner: Option<NonZeroUsize>,
}

impl From<Option<NonZeroUsize>> for Addr {
    fn from(value: Option<NonZeroUsize>) -> Self {
        Self { inner: value }
    }
}

impl<T> From<Option<NonNull<T>>> for Addr {
    fn from(value: Option<NonNull<T>>) -> Self {
        // safety: pointer is NonNull so value is nonzero
        Self {
            inner: value.map(|ptr| unsafe { NonZeroUsize::new_unchecked(ptr.as_ptr() as usize) }),
        }
    }
}

impl Addr {
    pub fn new(value: Option<NonZeroUsize>) -> Self {
        value.into()
    }

    pub fn as_ptr<T>(self) -> Option<NonNull<T>> {
        // Safety: it is always safe to cast a NonZeroUsize to a ptr::NonNull
        self.inner
            .map(|ptr| unsafe { NonNull::new_unchecked(ptr.get() as *mut T) })
    }

    /// Returns the address as a usize
    pub fn as_usize(&self) -> usize {
        match self.inner {
            None => 0,
            Some(ptr) => ptr.get(),
        }
    }

    pub fn map_addr(self, f: impl FnOnce(NonZeroUsize) -> NonZeroUsize) -> Self {
        Self {
            inner: self.inner.map(|addr| f(addr)),
        }
    }
}
