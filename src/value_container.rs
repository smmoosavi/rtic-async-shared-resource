mod change_future;
mod inner;
mod set_value_future;

pub struct ValueContainer<V> {
    value: V,
    inner: inner::ValueContainerInner<V>,
}

impl<V> ValueContainer<V>
where
    V: Clone,
{
    pub fn new(value: V) -> Self {
        let inner = inner::ValueContainerInner::new(value.clone());
        Self { value, inner }
    }

    pub fn get_value(&self) -> V {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: V) -> set_value_future::SetValueFuture<V> {
        self.value = value;
        set_value_future::SetValueFuture::new(self.value.clone(), &self.inner)
    }
    pub fn wait_for_change(&self) -> change_future::ChangeFuture<V>
    where
        V: PartialEq<V>,
    {
        change_future::ChangeFuture::new(Some(self.value.clone()), &self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_mutex::MockMutex;
    use async_std::prelude::FutureExt;
    use async_std::task::yield_now;
    use rtic_core::Mutex;

    #[test]
    fn test_have_initial_value() {
        let value = ValueContainer::new(0);
        assert_eq!(value.get_value(), 0);
    }

    #[test]
    fn test_set_value() {
        let mut value = ValueContainer::new(0);
        value.set_value(1);
        assert_eq!(value.get_value(), 1);
    }

    #[async_std::test]
    async fn test_work_with_mutex() {
        let mtx = MockMutex::new(ValueContainer::new(0));
        let ctrl = async {
            let mut mtx = mtx.clone();
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: set");
            mtx.lock(|value| value.set_value(1));
            println!("ctrl: release");
        };
        let run = async {
            let mut mtx = mtx.clone();
            println!("run: wait");
            let change = mtx.lock(|value| value.wait_for_change()).await;
            println!("run: changed");
            assert_eq!(change, 1);
        };
        ctrl.join(run).await;

        // ctrl: release
        // run: wait
        // ctrl: set
        // ctrl: release
        // run: changed
    }

    #[async_std::test]
    async fn test_wait_miss_if_inner_is_taken() {
        let mtx = MockMutex::new(ValueContainer::new(0));
        let ctrl = async {
            let mut mtx = mtx.clone();
            println!("ctrl: take inner");
            let inner_ref = mtx.lock(|value| value.inner.value.try_access().unwrap());
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: set 1");
            mtx.lock(|value| value.set_value(1));
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: drop inner");
            drop(inner_ref);
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: set 2");
            mtx.lock(|value| value.set_value(2));
            println!("ctrl: release");
            yield_now().await;
        };
        let run = async {
            let mut mtx = mtx.clone();
            println!("run: wait");
            let change = mtx.lock(|value| value.wait_for_change()).await;
            println!("run: changed {}", change);
            assert_eq!(change, 2);
        };

        ctrl.join(run).await;
        // ctrl: take inner
        // ctrl: release
        // run: wait
        // ctrl: set 1
        // ctrl: release
        // ctrl: drop inner
        // ctrl: release
        // ctrl: set 2
        // ctrl: release
        // run: changed 2
    }

    #[async_std::test]
    async fn test_set_should_wait_until_inner_is_available() {
        let mtx = MockMutex::new(ValueContainer::new(0));
        let ctrl = async {
            let mut mtx = mtx.clone();
            println!("ctrl: take inner");
            let inner_ref = mtx.lock(|value| value.inner.value.try_access().unwrap());
            println!("ctrl: release");
            yield_now().await;
            println!("ctrl: drop inner");
            drop(inner_ref);
            println!("ctrl: release");
            yield_now().await;
        };

        let run = async {
            let mut mtx = mtx.clone();
            println!("run: wait");
            let change = mtx.lock(|value| value.wait_for_change()).await;
            println!("run: changed");
            assert_eq!(change, 1);
        };
        let set = async {
            let mut mtx = mtx.clone();
            println!("set: wait");
            mtx.lock(|value| value.set_value(1)).await;
            println!("set: done");
        };
        ctrl.join(run).join(set).await;
        // ctrl: take inner
        // ctrl: release
        // run: wait
        // set: wait
        // ctrl: drop inner
        // ctrl: release
        // set: done
        // run: changed
    }
}
