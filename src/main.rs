mod cli;
mod errors;

use clap::Parser;
use cli::Cli;
use tracing::debug;

fn init_tracing() {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "warn,kube_client=error".to_string()),
    );
    tracing_subscriber::fmt::init();
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    init_tracing();
    color_eyre::config::HookBuilder::default()
        .panic_section("consider reporting the bug on github")
        .install()?;

    let cli = Cli::parse();

    if let Some(context) = cli.context {
        debug!("using kubeconfig context: {}", context);
    } else {
        debug!("using default kubeconfig context");
    }
    Ok(())
}
