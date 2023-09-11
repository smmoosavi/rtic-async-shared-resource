use super::inner::AsyncOptionInner;

pub struct WaitForValueFuture<'a, T> {
    inner: &'a AsyncOptionInner<T>,
}

impl<'a, T> WaitForValueFuture<'a, T> {
    pub(super) fn new(inner: &'a AsyncOptionInner<T>) -> Self {
        Self { inner }
    }
}

impl<'a, T: Clone> core::future::Future for WaitForValueFuture<'a, T> {
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
