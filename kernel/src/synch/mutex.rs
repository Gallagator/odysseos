use core::ops::{Deref, DerefMut};

use spin;

pub struct Mutex<T> {
    inner: spin::Mutex<T>,
}

pub struct MutexGuard<'a, T> {
    inner: spin::MutexGuard<'a, T>,
}

impl<T> Mutex<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: spin::Mutex::new(val),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        MutexGuard {
            inner: self.inner.lock(),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.inner.is_locked()
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.inner.try_lock().map(|inner| MutexGuard { inner })
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.inner // SO BEAUTIFUL!
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.inner
    }
}
