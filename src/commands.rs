use std::io::{self, Write};

use clap::Subcommand;
use rpassword::{prompt_password, read_password};
use serde::de::{self, value};

use crate::{
    crypto,
    storage::{self, load_vault, save_vault},
};

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
    if dev_mode {
        println!("(Dev mode) The master password is: {}", password);
    }
    Ok(password)
}

pub fn list_entries(dev_mode: bool) -> anyhow::Result<()> {
    let vault = load_vault(dev_mode)?;
    let services: Vec<&str> = vault.entries.iter().map(|e| e.service.as_str()).collect();
    println!("Stored services:");
    for service in services {
        println!("- {}", service);
    }
    Ok(())
}

pub fn add_entry(
    service: &str,
    username: &str,
    password: &str,
    dev_mode: bool,
) -> anyhow::Result<()> {
    let mut vault = load_vault(dev_mode)?;
    let master_password = get_master_password(dev_mode)?;
    let derived_key = crypto::derive_key(&master_password, &vault.salt)?;
    let is_correct = crypto::test_master_key(&derived_key, &vault.check)?;
    if !is_correct {
        anyhow::bail!("Incorrect master password");
    }
    let encrypted_data = crypto::encrypt(&derived_key, password);

    vault.entries.push(crate::models::Entry {
        service: service.to_string(),
        username: Some(username.to_string()),
        nonce: encrypted_data.nonce,
        ciphertext: encrypted_data.ciphertext,
    });
    save_vault(&vault, dev_mode)?;
    println!("Added entry for service: {}", service);
    Ok(())
}
pub fn remove_entry(service: &str, dev_mode: bool) -> anyhow::Result<()> {
    let mut vault = load_vault(dev_mode)?;
    let master_password = get_master_password(dev_mode)?;
    let derived_key = crypto::derive_key(&master_password, &vault.salt)?;
    let is_correct = crypto::test_master_key(&derived_key, &vault.check)?;
    if !is_correct {
        anyhow::bail!("Incorrect master password");
    }

    vault.entries.retain(|e| e.service.to_lowercase() != service.to_lowercase());
    save_vault(&vault, dev_mode)?;
    println!("Removed entry for service: {}", service);
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
    let vault = crate::models::Vault {
        version: 1,
        salt,
        check: encrypted_check,
        entries: Vec::new(),
    };
    save_vault(&vault, dev_mode)?;
    println!("Initialized vault successfully.");
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

fn check_master_password(
    master_password: &str,
    vault: &crate::models::Vault,
) -> anyhow::Result<bool> {
    let derived_key = crypto::derive_key(master_password, &vault.salt)?;
    let is_correct = crypto::test_master_key(&derived_key, &vault.check)?;
    Ok(is_correct)
}
