use crate::std::*;
use tracing_core::LevelFilter;
use tracing_core::{span, Event, Metadata, field, Level};


#[cfg(feature = "heapless")]
const BUFFER_SIZE: usize = 200;


#[derive(Debug)]
pub struct SubscriberBuilder<F = LevelFilter> {
    #[allow(dead_code)]
    filter: F,
}


impl Default for SubscriberBuilder {
    fn default() -> Self {
        Self {
            filter: LevelFilter::INFO
        }
    }
}


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

    println!("{} [{}:{}] {}", level_str, file, line, msg);
}


#[derive(Debug, Clone, Copy)]
pub struct Subscriber {
}


impl Subscriber {
    pub fn new() -> Self {
        Self {  }
    }
}


impl tracing_core::Subscriber for Subscriber {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        // metadata.level() <= self.filter
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

    fn record(&self, _span: &span::Id, _values: &span::Record<'_>) {
    }

    fn record_follows_from(&self, _span: &span::Id, _follows: &span::Id) {
    }

    fn event(&self, event: &Event<'_>) {
        let metadata = event.metadata();

        let mut writer = DebugVisitor::new();
        event.record(&mut writer);

        debug_info(metadata, &*writer);
    }

    fn enter(&self, _span: &span::Id) {
    }

    fn exit(&self, _span: &span::Id) {
    }
}


pub struct DebugVisitor {
    #[cfg(feature = "heapless")]
    buffer: String<BUFFER_SIZE>,
    #[cfg(any(feature = "alloc", feature = "std"))]
    buffer: String,
}


impl DebugVisitor {
    pub fn new() -> Self {
        Self { buffer: String::new() }
    }
}


impl Deref for DebugVisitor {
    #[cfg(feature = "heapless")]
    type Target = String<BUFFER_SIZE>;
    #[cfg(any(feature = "alloc", feature = "std"))]
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


#[allow(dead_code)]
pub fn fmt() -> SubscriberBuilder {
    SubscriberBuilder::default()
}


pub fn init() {
    let subscriber: Subscriber = Subscriber::new();
    let dispatcher = tracing_core::dispatcher::Dispatch::new(subscriber);
    tracing_core::dispatcher::set_global_default(dispatcher).unwrap();

    // tracing::subscriber::set_global_default(subscriber).unwrap();
}