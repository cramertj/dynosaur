use core::mem::ManuallyDrop;

pub struct Ref<T> {
    t: ManuallyDrop<T>,
}

impl<T> Ref<T> {
    pub unsafe fn new(t: T) -> Self {
        Self {
            t: ManuallyDrop::new(t),
        }
    }
}

impl<T> std::ops::Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.t
    }
}

/// Newtype that permits borrowed (`&mut T`) or shared (`&T`) access,
/// but nothing else.
pub struct RefMut<T> {
    t: ManuallyDrop<T>,
}

impl<T> RefMut<T> {
    pub fn new(t: T) -> Self {
        Self {
            t: ManuallyDrop::new(t),
        }
    }
}

impl<T> std::ops::Deref for RefMut<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.t
    }
}

impl<T> std::ops::DerefMut for RefMut<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.t
    }
}
