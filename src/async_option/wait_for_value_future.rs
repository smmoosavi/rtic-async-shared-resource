use super::inner::AsyncOptionInner;

pub struct WaitForValueFuture<T> {
    inner: AsyncOptionInner<T>,
}

impl<T> WaitForValueFuture<T> {
    pub(super) fn new(inner: AsyncOptionInner<T>) -> Self {
        Self { inner }
    }
}

impl<T: Clone> core::future::Future for WaitForValueFuture<T> {
    type Output = T;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let inner = self.inner.value.try_borrow_mut();
        let waker = self.inner.waker.try_borrow_mut();
        if let Ok(inner) = inner {
            if let Some(item) = inner.clone() {
                return core::task::Poll::Ready(item);
            }
        }
        if let Ok(mut waker) = waker {
            *waker = Some(cx.waker().clone());
        }
        core::task::Poll::Pending
    }
}
