use crate::async_option::inner::AsyncOptionInner;
use crate::async_option::wait_for_value_future::WaitForValueFuture;

mod inner;
mod wait_for_value_future;

pub struct AsyncOption<T> {
    inner: AsyncOptionInner<T>,
}

impl<T> AsyncOption<T> {
    pub fn new(value: Option<T>) -> Self {
        AsyncOption {
            inner: AsyncOptionInner::new(value),
        }
    }
    pub fn ready(item: T) -> Self {
        AsyncOption::new(Some(item))
    }
    pub fn pending() -> Self {
        AsyncOption::new(None)
    }
    pub fn try_set(&self, item: T) -> Result<(), T> {
        self.inner.try_set(item)
    }

    pub fn wait_for_value(&self) -> WaitForValueFuture<T>
    where
        T: Clone,
    {
        WaitForValueFuture::new(self.inner.clone())
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
        assert_eq!(option.wait_for_value().await, 1);
    }

    #[async_std::test]
    async fn test_with_mutex() {
        let mtx = MockMutex::new(super::AsyncOption::pending());
        let ctrl = async {
            let mut mtx = mtx.clone();
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: set");
            mtx.lock(|option| option.try_set(1).ok());
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: done");
        };

        let run = async {
            let mut mtx = mtx.clone();
            println!("run: wait");
            assert_eq!(mtx.lock(|option| option.wait_for_value()).await, 1);
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
    async fn test_can_be_waited_multiple_time() {
        let mtx = MockMutex::new(super::AsyncOption::pending());
        let ctrl = async {
            let mut mtx = mtx.clone();
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: set");
            mtx.lock(|option| option.try_set(1).ok());
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: done");
        };
        let run1 = async {
            let mut mtx = mtx.clone();
            println!("run1: wait");
            assert_eq!(mtx.lock(|option| option.wait_for_value()).await, 1);
            println!("run1: done");
        };
        let run2 = async {
            let mut mtx = mtx.clone();
            println!("run2: wait");
            assert_eq!(mtx.lock(|option| option.wait_for_value()).await, 1);
            println!("run2: done");
        };
        ctrl.join(run1).join(run2).await;

        // output:
        // ctrl: release
        // run1: wait
        // run2: wait
        // ctrl: set
        // ctrl: release
        // run1: done
        // run2: done
        // ctrl: done
    }

    // AsyncOption is not Send, because it contains Rc.
    // #[test]
    // fn test_type_is_send() {
    //     fn is_send<T: Send>() {}
    //     is_send::<super::AsyncOption<u32>>();
    // }
}
