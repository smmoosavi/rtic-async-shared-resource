#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod async_option;
pub use async_option::AsyncOption;

#[cfg(test)]
mod mock_mutex;
