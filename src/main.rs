use std::path::Path;

use clap::Parser;

mod cli;
mod commands;

fn main() -> anyhow::Result<()> {
    let dev_mode = Path::new("Cargo.toml").exists();

    let cli = cli::Cli::parse();

    Ok(())
}
