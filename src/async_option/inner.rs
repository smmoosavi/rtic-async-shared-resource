use alloc::rc::Rc;
use core::cell::RefCell;

pub(super) struct AsyncOptionInner<T> {
    value: Rc<RefCell<Option<T>>>,
    waker: Rc<RefCell<Option<core::task::Waker>>>,
}

impl<T> AsyncOptionInner<T> {
    pub(super) fn new(value: Option<T>) -> Self {
        AsyncOptionInner {
            value: Rc::new(RefCell::new(value)),
            waker: Rc::new(RefCell::new(None)),
        }
    }

    pub(super) fn try_get(&self) -> Option<T>
    where
        T: Clone,
    {
        let inner = self.value.try_borrow().ok()?;
        inner.clone()
    }
    pub(super) fn try_set(&self, item: T) -> Result<(), T> {
        let Ok(mut inner) = self.value.try_borrow_mut() else {
            return Err(item);
        };
        *inner = Some(item);
        self.try_wake();
        Ok(())
    }

    pub(super) fn try_wake(&self) {
        if let Ok(waker) = self.waker.try_borrow_mut() {
            if let Some(waker) = waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    pub(super) fn set_waker(&self, new_waker: core::task::Waker) {
        if let Ok(mut waker) = self.waker.try_borrow_mut() {
            *waker = Some(new_waker);
        }
    }
}

impl<T> Clone for AsyncOptionInner<T> {
    fn clone(&self) -> Self {
        AsyncOptionInner {
            value: self.value.clone(),
            waker: self.waker.clone(),
        }
    }
}
