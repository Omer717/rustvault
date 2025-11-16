use crate::{models::Vault, utils::get_file};
use std::fs;

pub fn load_vault(dev_mode: bool) -> anyhow::Result<Vault> {
    let path = get_file("vault.json", dev_mode)?;
    let data = fs::read_to_string(&path)?;
    let vault = serde_json::from_str(&data)?;
    Ok(vault)
}

pub fn save_vault(vault: &Vault, dev_mode: bool) -> anyhow::Result<()> {
    let path = get_file("vault.json", dev_mode)?;
    let json_vault: String = serde_json::to_string_pretty(vault)?;
    std::fs::write(&path, json_vault)?;
    Ok(())
}
