use rtic_sync::arbiter::Arbiter;

pub(super) struct AsyncOptionInner<T> {
    value: Arbiter<Option<T>>,
    waker: Arbiter<Option<core::task::Waker>>,
}

impl<T> AsyncOptionInner<T> {
    pub(super) fn new(value: Option<T>) -> Self {
        AsyncOptionInner {
            value: Arbiter::new(value),
            waker: Arbiter::new(None),
        }
    }

    pub(super) fn try_get(&self) -> Option<T>
    where
        T: Clone,
    {
        let inner = self.value.try_access()?;
        inner.clone()
    }
    pub(super) fn try_set(&self, item: T) -> Result<(), T> {
        let Some(mut inner) = self.value.try_access() else {
            return Err(item);
        };
        *inner = Some(item);
        self.try_wake();
        Ok(())
    }

    pub(super) fn try_wake(&self) {
        if let Some(waker) = self.waker.try_access() {
            if let Some(waker) = waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    pub(super) fn set_waker(&self, new_waker: core::task::Waker) {
        if let Some(mut waker) = self.waker.try_access() {
            *waker = Some(new_waker);
        }
    }
}
