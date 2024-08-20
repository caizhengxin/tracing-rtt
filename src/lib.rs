#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;


mod errors;
mod fmt;
mod std;

pub use fmt::init;
