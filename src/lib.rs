#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;


mod errors;
mod rprint;
mod fmt;
mod std;

pub use fmt::init;
