use clap::Parser;
use domovoy::{Cli, Commands, auth, tui};
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_appender = rolling::never("/tmp", "domovoy.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("debug".parse()?))
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Auth => auth::init()?,
        Commands::Reset => auth::reset()?,
        Commands::Tui => tui::run()?,
    }
    Ok(())
}
