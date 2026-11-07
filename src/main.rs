use clap::Parser;
use palworld_cli::{Cli, dispatch};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Err(err) = dispatch(cli).await {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
