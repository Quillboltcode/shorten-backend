use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, Registry};

pub fn init_tracing() {
    let subscriber = Registry::default()
        .with(fmt::layer().compact())
        .with(EnvFilter::from_default_env());

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up tracing");
}
