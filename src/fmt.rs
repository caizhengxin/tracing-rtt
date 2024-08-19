use tracing_core::LevelFilter;


#[derive(Debug)]
pub struct SubscriberBuilder<F = LevelFilter> {
    filter: F,
}


impl Default for SubscriberBuilder {
    fn default() -> Self {
        Self {
            filter: LevelFilter::INFO
        }
    }
}


pub fn fmt() -> SubscriberBuilder {
    SubscriberBuilder::default()
}


pub fn init() {
    let subscriber = crate::subscriber::RttSubscriber::new();
    let dispatcher = tracing_core::dispatcher::Dispatch::new(subscriber);
    tracing_core::dispatcher::set_global_default(dispatcher).unwrap();

    // tracing::subscriber::set_global_default(subscriber).unwrap();
}