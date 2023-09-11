use alloc::rc::Rc;
use core::cell::RefCell;

pub(super) struct AsyncOptionInner<T> {
    pub(super) value: Rc<RefCell<Option<T>>>,
    pub(super) waker: Rc<RefCell<Option<core::task::Waker>>>,
}

impl<T> AsyncOptionInner<T> {
    pub(super) fn new(value: Option<T>) -> Self {
        AsyncOptionInner {
            value: Rc::new(RefCell::new(value)),
            waker: Rc::new(RefCell::new(None)),
        }
    }
    pub(super) fn try_set(&self, item: T) -> Result<(), T> {
        let Ok(mut inner) = self.value.try_borrow_mut() else {
        return Err(item);
    };
        *inner = Some(item);
        if let Some(waker) = self.waker.try_borrow_mut().ok().take() {
            if let Some(waker) = waker.as_ref() {
                waker.wake_by_ref();
            }
        }
        Ok(())
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
