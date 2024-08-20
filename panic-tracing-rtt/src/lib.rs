//! Set the panicking behavior to a J-LINK debugger and loop.
//! 
//! This crate contains a panic handler that emits the reason to an
//! in-memory ring buffer that an attached J-LINK device can print,
//! and then loops forever.
//! 
//! # Example
//! 
//! ```ignore
//! #![no_std]
//! 
//! use panic_tracing_rtt as _;
//! 
//! fn main() {
//!     panic!("jankincai");
//! }
//! ```
#![no_std]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::interrupt;


#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();

    tracing::error!("{info}");

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}