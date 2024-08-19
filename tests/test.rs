#[test]
fn test_tracing() {
    tracing_rtt::init();
    // tracing_subscriber::fmt().init();

    tracing::error!("jankincai");
    tracing::warn!("jankincai");
    tracing::info!("jankincai");
    tracing::debug!("jankincai");
    tracing::trace!("jankincai");

    tracing::info_span!("jankincai");
}