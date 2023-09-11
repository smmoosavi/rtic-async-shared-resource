# RTIC Async Shared Resource

This is an experiment to use async shared resources in RTIC. In many cases, we
need a shared value that we can mutate in one task and read in another task.
`AsyncOption` is an example struct that uses shared resources internally.
multiple implementation of shared resource is tested in this repo.

## `AsyncOption`

`AsyncOption` is a struct that can be waited until it has some value.

```rust
impl AsyncOption<T> {
  fn new(value: Option<T>) -> Self {}
  fn ready(item: T) -> Self {}
  fn pending() -> Self {}

  fn try_set(&self, item: T) -> Result<(), T> {}
  fn wait_for_value(&self) -> impl Future<Output = T> where T: Clone {}
}
```

example usage:

```rust
#[task(shared = [option], priority = 1)]
async fn task1(mut ctx: task1::Context) {
  let option = ctx.shared.option;
  assert_eq!(option.lock(|option| option.wait_for_value()).await, 1);
}

#[task(shared = [option], priority = 1)]
async fn task2(mut ctx: task2::Context) {
  let option = ctx.shared.option;
  option.lock(|option| option.try_set(1)).unwrap();
}
```

to implement `AsyncOption` we need a shared resource that can be mutated in one
task and be read in another task. we have some options:

### `Rc<RefCell>`

`Rc<RefCell>` is a type that allows us to have multiple owners of data where
only one of them can mutate the data at any given time. But it's not `Send`.
[shared][rtic-share] resource in RTIC should be `Send`.

> Types of `#[shared]` resources have to be `Send`.

```rust
//  `std::rc::Rc<std::cell::RefCell<std::option::Option<u32>>>` cannot be sent between threads safely
```

[code][code-rc-refcell]

### `Arc<Arbiter>`

The rtic-sync crate provides the [`Arbiter`][arbiter] struct for exclusive
access to shared resources. But it's not `Clone`. The `Arc<Arbiter>` type allows
us to have multiple owned values that can have exclusive access to shared
resources. And it's `Send`. But `Arc` does not exist in the `thumbv6m-none-eabi`
target.

```rust
use alloc::sync::Arc;
//         ^^^^ could not find `sync` in `alloc`
```

[code][code-arbiter-arc]

### `Arbiter` ref

If we have a reference to `Arbiter`, we can mutate its value without a mutable
reference. However, having a reference to `Arbiter` means our struct should have
a lifetime, which will cause lifetime errors when used with Mutex

```rust
assert_eq!(mtx.lock(|option| option.wait_for_value()).await, 1);
//                   ------- ^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
//                   |     |
//                   |     return type of closure is async_option::wait_for_value_future::WaitForValueFuture<'2, i32>
//                   has type `&'1 mut async_option::AsyncOption<i32>`

```

[code][code-arbiter-ref]

[rtic-share]:
  https://rtic.rs/2/book/en/by-example/resources.html#shared-resources-and-lock
[arbiter]: https://docs.rs/rtic-sync/latest/rtic_sync/arbiter/index.html
[code-rc-refcell]:
  https://github.com/smmoosavi/rtic-async-shared-resource/tree/rc-refcell/src
[code-arbiter-arc]:
  https://github.com/smmoosavi/rtic-async-shared-resource/tree/arbiter-arc/src
[code-arbiter-ref]:
  https://github.com/smmoosavi/rtic-async-shared-resource/tree/arbiter-ref/src
