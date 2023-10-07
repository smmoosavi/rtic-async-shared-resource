use alloc::rc::Rc;
use core::cell::RefCell;
use core::task::Waker;

pub struct ValueContainerInner<V> {
    pub(super) value: Rc<RefCell<V>>,
    pub(super) waker: Rc<RefCell<Option<Waker>>>,
}

impl<T> ValueContainerInner<T> {
    pub(super) fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            waker: Rc::new(RefCell::new(None)),
        }
    }

    pub(super) fn wake(&self) {
        if let Ok(inner_waker) = self.waker.try_borrow() {
            if let Some(waker) = inner_waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    pub(super) fn set_waker(&mut self, waker: Waker) {
        if let Ok(mut inner_waker) = self.waker.try_borrow_mut() {
            *inner_waker = Some(waker);
        }
    }

    pub(super) fn try_set_value(&self, value: T) -> Result<(), T> {
        if let Ok(mut inner_value) = self.value.try_borrow_mut() {
            *inner_value = value;
            self.wake();
            Ok(())
        } else {
            Err(value)
        }
    }

    pub(super) fn try_get_value(&self) -> Option<T>
    where
        T: Clone,
    {
        if let Ok(inner_value) = self.value.try_borrow() {
            Some(inner_value.clone())
        } else {
            None
        }
    }
}

impl<T> Clone for ValueContainerInner<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            waker: self.waker.clone(),
        }
    }
}
