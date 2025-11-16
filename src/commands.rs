use std::io::{self, Write};

use clap::Subcommand;
use rpassword::{prompt_password, read_password};

use crate::crypto;

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
    let master_pass = get_master_password(dev_mode)?;
    let salt = crypto::get_salt();
    let master_key = crypto::derive_key(&master_pass, &salt)?;
    let encrypted_check = crypto::encrypt_check(&master_key);
    let mut vault = crate::models::Vault {
        version: 1,
        salt,
        check: encrypted_check,
        entries: Vec::new(),
    };

    let x = serde_json::to_string_pretty(&vault)?;
    println!("Initialized vault:\n{}", x);

    let test_pass = get_master_password(dev_mode)?;
    let test_key = crypto::derive_key(&test_pass, &vault.salt)?;
    let is_valid = crypto::test_master_key(&test_key, &vault.check)?;
    println!("Master password valid: {}", is_valid);
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
