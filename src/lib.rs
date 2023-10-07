#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod async_option;
mod value_container;

pub use async_option::AsyncOption;
pub use value_container::ValueContainer;

#[cfg(test)]
mod mock_mutex;
