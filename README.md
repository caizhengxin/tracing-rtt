# tracing-rtt

[![Crates.io](https://img.shields.io/crates/v/tracing-rtt)](https://crates.io/crates/tracing-rtt)
[![Crates.io](https://img.shields.io/crates/d/tracing-rtt)](https://crates.io/crates/tracing-rtt)
[![License](https://img.shields.io/crates/l/tracing-rtt)](LICENSE-MIT)

## Features

- [x] OpenOCD RTT
- [ ] Embedded Time
- [ ] Filter

## Usage

> Cargo.toml

```toml
[dependencies]
tracing = { version = "0.1", default-features = false }
# tracing-rtt = { git = "https://github.com/caizhengxin/tracing-rtt.git", default-features = false, features = ["alloc"] }
tracing-rtt = { git = "https://github.com/caizhengxin/tracing-rtt.git", default-features = false, features = ["heapless"] }
```

or:

```toml
[dependencies]
tracing = { version = "0.1", default-features = false }
tracing-rtt = { version = "0.1.0", default-features = false, features = ["heapless"] }
# tracing-rtt = { version = "0.1.0", default-features = false, features = ["alloc"] }
```

> main.rs

```rust
#![no_std]
#![no_main]


use cortex_m_rt::entry;
use panic_tracing_rtt as _;
use cortex_m::Peripherals;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    rtc::Rtc,
};
use rand_core::RngCore;

// features = ["alloc"]
// use embedded_alloc::Heap;

// #[global_allocator]
// static HEAP: Heap = Heap::empty();


// pub fn embedded_heap_init() {
//     use core::mem::MaybeUninit;
//     const HEAP_SIZE: usize = 1024;
//     static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
//     unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
// }

#[entry]
fn main() -> ! {
    // init heap
    // embedded_heap_init();

    // init RTT
    tracing_rtt::init();

    let mut dp = pac::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr
        .use_hse(8.MHz())
        .require_pll48clk() 
        // .sysclk(48.MHz())
        .freeze();

    let mut delay = cp.SYST.delay(&clocks);

    let mut rng = dp.RNG.constrain(&clocks);

    // let mut rtc = Rtc::new(dp.RTC, &mut dp.PWR);
    let mut rtc = Rtc::new_lsi(dp.RTC, &mut dp.PWR);

    // LED
    let gpiofs = dp.GPIOF.split();
    let mut led0 = gpiofs.pf9.into_push_pull_output();
    let mut led1 = gpiofs.pf10.into_push_pull_output();

    loop {
        tracing::debug!("time: {} random: {}", rtc.get_datetime(), rng.next_u64());

        led0.set_low();
        led1.set_low();
        delay.delay_ms(50);

        led0.set_high();
        led1.set_high();
        delay.delay_ms(50);
    }
}
```

## Example

- [embedded-rtt](./examples/embedded-rtt/)
