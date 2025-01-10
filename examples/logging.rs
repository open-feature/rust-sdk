use open_feature::{provider::NoOpProvider, EvaluationOptions, OpenFeature};

#[tokio::main]
async fn main() {
    init_logger();

    let mut api = OpenFeature::singleton_mut().await;
    api.set_provider(NoOpProvider::default()).await;
    drop(api);

    let client = OpenFeature::singleton()
        .await
        .create_client()
        .with_logging_hook(true); // Add a client-level hook

    let eval = EvaluationOptions::default();
    let _ = client
        .get_bool_details("my_feature", None, Some(&eval))
        .await;
}

#[cfg(not(feature = "structured-logging"))]
fn init_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
}

#[cfg(feature = "structured-logging")]
fn init_logger() {
    structured_logger::Builder::with_level("debug").init();
}
