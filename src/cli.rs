use crate::commands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rustvault")]
#[command(about = "A rust safe password vault CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

impl Cli {
    pub fn command(&self) -> &commands::Commands {
        &self.command
    }
}
