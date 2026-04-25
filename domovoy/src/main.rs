use clap::Parser;
use domovoy::{Cli, Commands, auth};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Auth => auth::init()?,
        Commands::Reset => auth::reset()?,
    }
    Ok(())
}
