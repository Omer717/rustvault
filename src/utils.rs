use directories::ProjectDirs;
use std::{fs, path::PathBuf};

pub fn get_file(file_name: &str, dev_mode: bool) -> anyhow::Result<std::path::PathBuf> {
    if dev_mode {
        // Stay in project folder for development
        Ok(PathBuf::from(file_name))
    } else {
        // Use OS-appropriate application data folder
        if let Some(proj_dirs) = ProjectDirs::from("com", "OmerOrg", "Rustask") {
            let data_dir = proj_dirs.data_dir();
            fs::create_dir_all(data_dir)?;
            Ok(data_dir.join(file_name))
        } else {
            anyhow::bail!("Could not determine data directory for your OS");
        }
    }
}
