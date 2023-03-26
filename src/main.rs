use clap::Parser;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, env, default_value = "local")]
    environment: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("tasks".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("setting default subscriber failed");
    let args = Args::parse();

    // TODO: Change crate name
    let settings = axum_template::config::get_config(&args.environment)?;
    axum_template::run(settings).await?;
    Ok(())
}
