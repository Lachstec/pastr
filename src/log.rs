use tracing_subscriber::fmt::format::FmtSpan;

pub fn configure_subscriber() {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::NEW)
        .init();
}
