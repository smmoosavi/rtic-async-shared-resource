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
        if let Some(item) = self.inner.try_get() {
            return core::task::Poll::Ready(item);
        }
        self.inner.set_waker(cx.waker().clone());
        core::task::Poll::Pending
    }
}
