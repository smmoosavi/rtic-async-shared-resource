use core::task::Waker;
use rtic_sync::arbiter::Arbiter;

pub struct ValueContainerInner<V> {
    pub(super) value: Arbiter<V>,
    pub(super) waker: Arbiter<Option<Waker>>,
}

impl<T> ValueContainerInner<T> {
    pub(super) fn new(value: T) -> Self {
        Self {
            value: Arbiter::new(value),
            waker: Arbiter::new(None),
        }
    }

    pub(super) fn wake(&self) {
        if let Some(inner_waker) = self.waker.try_access() {
            if let Some(waker) = inner_waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    pub(super) fn set_waker(&self, waker: Waker) {
        if let Some(mut inner_waker) = self.waker.try_access() {
            *inner_waker = Some(waker);
        }
    }

    pub(super) fn try_set_value(&self, value: T) -> Result<(), T> {
        if let Some(mut inner_value) = self.value.try_access() {
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
        self.value
            .try_access()
            .map(|inner_value| inner_value.clone())
    }
}
