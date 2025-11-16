use std::io::{self, Write};

use clap::Subcommand;
use rpassword::{prompt_password, read_password};

#[derive(Subcommand)]
pub enum Commands {
    List,
    Add {
        service: String,
        username: String,
        password: String,
    },
    Remove {
        service: String,
    },
    Get {
        service: String,
    },
    Init,
    Edit {
        service: String,
        username: Option<String>,
        password: Option<String>,
    },
    Export {
        filepath: String,
    },
}

fn get_master_password(dev_mode: bool) -> anyhow::Result<String> {
    let password = prompt_password("Enter Master Password:").unwrap();
    println!("The master password is: {}", password);
    Ok(password)
}

pub fn list_entries(dev_mode: bool) -> anyhow::Result<()> {
    // Implementation here
    Ok(())
}
pub fn add_entry(
    service: &str,
    username: &str,
    password: &str,
    dev_mode: bool,
) -> anyhow::Result<()> {
    get_master_password(dev_mode)?;
    // Implementation here
    Ok(())
}
pub fn remove_entry(service: &str, dev_mode: bool) -> anyhow::Result<()> {
    get_master_password(dev_mode)?;
    // Implementation here
    Ok(())
}
pub fn get_entry(service: &str, dev_mode: bool) -> anyhow::Result<()> {
    get_master_password(dev_mode)?;
    // Implementation here
    Ok(())
}
pub fn initialize_vault(dev_mode: bool) -> anyhow::Result<()> {
    get_master_password(dev_mode)?;
    // Implementation here
    Ok(())
}
pub fn edit_entry(
    service: &str,
    username: Option<&str>,
    password: Option<&str>,
    dev_mode: bool,
) -> anyhow::Result<()> {
    get_master_password(dev_mode)?;
    // Implementation here
    Ok(())
}
pub fn export_vault(filepath: &str, dev_mode: bool) -> anyhow::Result<()> {
    get_master_password(dev_mode)?;
    // Implementation here
    Ok(())
}
