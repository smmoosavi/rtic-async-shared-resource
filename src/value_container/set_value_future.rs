use super::inner::ValueContainerInner;
use core::future::Future;
use core::task::{Context, Poll};
use pin_project::pin_project;

#[pin_project]
pub struct SetValueFuture<'a, T> {
    inner: &'a ValueContainerInner<T>,
    value: Option<T>,
}

impl<'a, T> SetValueFuture<'a, T> {
    pub(super) fn new(value: T, inner: &'a ValueContainerInner<T>) -> Self {
        let res = inner.try_set_value(value);
        match res {
            Ok(_) => Self { inner, value: None },
            Err(value) => Self {
                inner,
                value: Some(value),
            },
        }
    }
}

impl<'a, T> Future for SetValueFuture<'a, T>
where
    T: Clone,
{
    type Output = ();

    fn poll(self: core::pin::Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let value = this.value.take();
        match value {
            Some(value) => match this.inner.try_set_value(value) {
                Ok(_) => Poll::Ready(()),
                Err(value) => {
                    *this.value = Some(value);
                    Poll::Pending
                }
            },
            None => Poll::Ready(()),
        }
    }
}
