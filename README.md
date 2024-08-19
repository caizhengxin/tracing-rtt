# tracing-rtt

[![Crates.io](https://img.shields.io/crates/v/tracing-rtt)](https://crates.io/crates/tracing-rtt)
[![Crates.io](https://img.shields.io/crates/d/tracing-rtt)](https://crates.io/crates/tracing-rtt)
[![License](https://img.shields.io/crates/l/tracing-rtt)](LICENSE-MIT)

## Features

- [x] xxxx
- [ ] xxxx

## Usage

> Cargo.toml

```toml
[dependencies]
tracing-rtt = { git = "https://github.com/caizhengxin/tracing-rtt.git" }
```

or:

```toml
[dependencies]
tracing-rtt = "0.1.0"
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
