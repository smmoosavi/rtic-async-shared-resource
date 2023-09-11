use alloc::rc::Rc;
use std::sync::Mutex;

pub struct MockMutex<T> {
    value: Rc<Mutex<T>>,
}

impl<T> Clone for MockMutex<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<T> MockMutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(Mutex::new(value)),
        }
    }
}

impl<T> rtic_core::Mutex for MockMutex<T> {
    type T = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R {
        let mut value = self.value.lock().unwrap();
        f(&mut value)
    }
}
