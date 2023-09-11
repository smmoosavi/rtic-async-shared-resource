use alloc::rc::Rc;
use core::cell::RefCell;

pub struct AsyncOption<T> {
    inner: Rc<RefCell<Option<T>>>,
    waker: Rc<RefCell<Option<core::task::Waker>>>,
}

impl<T> AsyncOption<T> {
    pub fn new(value: Option<T>) -> Self {
        AsyncOption {
            inner: Rc::new(RefCell::new(value)),
            waker: Rc::new(RefCell::new(None)),
        }
    }
    pub fn ready(item: T) -> Self {
        AsyncOption::new(Some(item))
    }
    pub fn pending() -> Self {
        AsyncOption::new(None)
    }
    pub fn set(&self, item: T) {
        *self.inner.borrow_mut() = Some(item);
        if let Some(waker) = self.waker.borrow_mut().take() {
            waker.wake();
        }
    }
}

impl<T> Clone for AsyncOption<T> {
    fn clone(&self) -> Self {
        AsyncOption {
            inner: self.inner.clone(),
            waker: self.waker.clone(),
        }
    }
}

impl<T> core::future::Future for AsyncOption<T> {
    type Output = T;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let inner = self.inner.try_borrow_mut();
        let waker = self.waker.try_borrow_mut();
        if let Ok(mut inner) = inner {
            if let Some(item) = inner.take() {
                return core::task::Poll::Ready(item);
            }
        }
        if let Ok(mut waker) = waker {
            *waker = Some(cx.waker().clone());
        }
        core::task::Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use async_std::prelude::FutureExt;
    use async_std::task::yield_now;
    use rtic_core::Mutex as _;

    use crate::mock_mutex::MockMutex;

    #[async_std::test]
    async fn test_some_should_be_ready() {
        let option = super::AsyncOption::ready(1);
        assert_eq!(option.await, 1);
    }

    #[async_std::test]
    async fn test_none_should_be_pending_until_set() {
        let option = super::AsyncOption::pending();
        let ctrl = async {
            let option = option.clone();
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: set");
            option.set(1);
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: done");
        };
        let run = async {
            let option = option.clone();
            println!("run: wait");
            assert_eq!(option.await, 1);
            println!("run: done");
        };
        ctrl.join(run).await;
        // output:
        // ctrl: release
        // run: wait
        // ctrl: set
        // ctrl: release
        // run: done
        // ctrl: done
    }

    #[async_std::test]
    async fn test_with_mutex() {
        let option = MockMutex::new(super::AsyncOption::pending());
        let ctrl = async {
            let mut option = option.clone();
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: set");
            option.lock(|option| option.set(1));
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: done");
        };

        let run = async {
            let mut option = option.clone();
            println!("run: wait");
            assert_eq!(option.lock(|option| option.clone()).await, 1);
            println!("run: done");
        };

        ctrl.join(run).await;
        // output:
        // ctrl: release
        // run: wait
        // ctrl: set
        // ctrl: release
        // run: done
        // ctrl: done
    }
}
