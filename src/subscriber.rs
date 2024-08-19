use tracing_core::{span, Event, Metadata, Subscriber, field, Level};
use tracing_core::span::{Attributes, Record};
#[cfg(not(feature = "std"))]
use rtt_target::{rprint as print, rprintln as println};
#[cfg(feature = "std")]
use std::{
    fmt,
    fmt::Write,
    ops::{Deref, DerefMut}
};
#[cfg(not(feature = "std"))]
use core::{
    fmt,
    fmt::Write,
    ops::{Deref, DerefMut}
};


#[inline]
fn debug_info<T: AsRef<[u8]> + fmt::Display>(metadata: &Metadata<'_>, msg: T) {
    let level = *metadata.level();

    let level_str = match level {
        Level::ERROR => "\x1b[31m[ERROR]\x1b[0m",
        Level::INFO => "\x1b[32m[INFO]\x1b[0m",
        Level::WARN => "\x1b[33m[WARN]\x1b[0m",
        Level::DEBUG => "\x1b[34m[DEBUG]\x1b[0m",
        Level::TRACE => "[TRACE]",
    };

    let file = metadata.file().unwrap_or_default();
    let line = metadata.line().unwrap_or_default();

    println!("{level_str} [{file}:{line}] {}", msg);
}


#[derive(Debug, Clone, Copy)]
pub struct RttSubscriber {
}


impl RttSubscriber {
    pub fn new() -> Self {
        Self {  }
    }
}


impl Subscriber for RttSubscriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        let metadata = span.metadata();

        let mut writer = DebugVisitor::new();
        span.record(&mut writer);
        debug_info(metadata, metadata.name());

        let id = span::Id::from_u64(1 as u64);

        id
    }

    fn record(&self, _span: &span::Id, _values: &Record<'_>) {
        // 记录 span 的信息
        println!("record: {:?}", _span)
    }

    fn record_follows_from(&self, _span: &span::Id, _follows: &span::Id) {
        // 记录关联的 span
        println!("record_follows_from: {:?}", _span)
    }

    fn event(&self, event: &Event<'_>) {
        let metadata = event.metadata();

        let mut writer = DebugVisitor::new();
        event.record(&mut writer);

        debug_info(metadata, &*writer);
    }

    fn enter(&self, _span: &span::Id) {
        // 处理进入 span 的逻辑
        println!("enter: {:?}", _span)
    }

    fn exit(&self, _span: &span::Id) {
        // 处理退出 span 的逻辑
        println!("exit: {:?}", _span)
    }
}


pub struct DebugVisitor {
    buffer: String,
}


impl DebugVisitor {
    pub fn new() -> Self {
        Self { buffer: String::new() }
    }
}


impl Deref for DebugVisitor {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}


impl DerefMut for DebugVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}


impl field::Visit for DebugVisitor {
    fn record_debug(&mut self, _field: &field::Field, value: &dyn core::fmt::Debug) {
        let _ = write!(self.buffer, "{:?}", value);
    }
}
