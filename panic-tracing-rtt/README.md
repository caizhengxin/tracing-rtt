# panic-tracing-rtt

[![Crates.io](https://img.shields.io/crates/v/tracing-rtt)](https://crates.io/crates/tracing-rtt)
[![Crates.io](https://img.shields.io/crates/d/tracing-rtt)](https://crates.io/crates/tracing-rtt)
[![License](https://img.shields.io/crates/l/tracing-rtt)](LICENSE-MIT)

## Cargo.toml

```toml
[dependencies]
panic-tracing-rtt = "0"
```

## Usage

```rust
#![no_std]

use panic_tracing_rtt as _;

fn main() {
    tracing_rtt::init();

    panic!("jankincai");
}
```