use core::task::Waker;
use portable_atomic_util::Arc;
use rtic_sync::arbiter::Arbiter;

pub struct ValueContainerInner<V> {
    pub(super) value: Arc<Arbiter<V>>,
    pub(super) waker: Arc<Arbiter<Option<Waker>>>,
}

impl<T> ValueContainerInner<T> {
    pub(super) fn new(value: T) -> Self {
        Self {
            value: Arc::new(Arbiter::new(value)),
            waker: Arc::new(Arbiter::new(None)),
        }
    }

    pub(super) fn wake(&self) {
        if let Some(inner_waker) = self.waker.try_access() {
            if let Some(waker) = inner_waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    pub(super) fn set_waker(&mut self, waker: Waker) {
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
