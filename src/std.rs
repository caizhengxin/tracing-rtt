#[cfg(feature = "std")]
pub use std::{
    fmt,
    fmt::Write,
    ops::{Deref, DerefMut}
};
#[cfg(not(feature = "std"))]
pub use core::{
    fmt,
    fmt::Write,
    ops::{Deref, DerefMut},
};
#[cfg(all(feature = "heapless", not(feature = "alloc")))]
pub use heapless::String;
#[cfg(all(feature = "alloc", not(feature = "heapless")))]
pub use alloc::{
    string::String,
};
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub use rtt_target::{rprint as print, rprintln as println};