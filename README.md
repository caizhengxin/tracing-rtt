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
fn main() {
    tracing_rtt::init();

    tracing::error!("jankincai");
    tracing::warn!("jankincai");
    tracing::info!("jankincai");
    tracing::debug!("jankincai");
    tracing::trace!("jankincai");
}
```
