use core::future::Future;
use core::pin::Pin;
use core::task::Context;
use core::task::Poll;
use pin_project::pin_project;

use super::inner::ValueContainerInner;

#[must_use = "futures do nothing unless you `.await` or poll them"]
#[pin_project]
pub struct ChangeFuture<T> {
    initial_value: Option<T>,
    inner: ValueContainerInner<T>,
}

impl<T> ChangeFuture<T>
where
    T: Clone,
{
    pub fn new(initial_value: Option<T>, inner: ValueContainerInner<T>) -> Self {
        Self {
            initial_value,
            inner,
        }
    }
}

impl<T> Future for ChangeFuture<T>
where
    T: Clone + PartialEq<T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if this.initial_value.is_none() {
            *this.initial_value = this.inner.try_get_value();
            return Poll::Pending;
        }

        match get_changed(this.initial_value.as_ref(), this.inner) {
            Some(value) => Poll::Ready(value),
            _ => {
                this.inner.set_waker(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

fn get_changed<T>(initial_value: Option<&T>, inner: &ValueContainerInner<T>) -> Option<T>
where
    T: Clone + PartialEq<T>,
{
    let initial_value = initial_value?;
    let inner_value = inner.value.try_borrow().ok()?;
    if *inner_value != *initial_value {
        Some(inner_value.clone())
    } else {
        None
    }
}
