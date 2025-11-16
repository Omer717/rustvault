use std::path::Path;

use clap::Parser;

mod cli;
mod commands;

fn main() -> anyhow::Result<()> {
    let dev_mode = Path::new("Cargo.toml").exists();

    let cli = cli::Cli::parse();

    match cli.command() {
        commands::Commands::List => {
            commands::list_entries(dev_mode)?;
        }
        commands::Commands::Add {
            service,
            username,
            password,
        } => {
            commands::add_entry(service, username, password, dev_mode)?;
        }
        commands::Commands::Remove { service } => {
            commands::remove_entry(service, dev_mode)?;
        }
        commands::Commands::Get { service } => {
            commands::get_entry(service, dev_mode)?;
        }
        commands::Commands::Init => {
            commands::initialize_vault(dev_mode)?;
        }
        commands::Commands::Edit {
            service,
            username,
            password,
        } => {
            commands::edit_entry(service, username.as_deref(), password.as_deref(), dev_mode)?;
        }
        commands::Commands::Export { filepath } => {
            commands::export_vault(filepath, dev_mode)?;
        }
    }

    Ok(())
}
