use std::{fs::copy, path::Path};

use super::super::Result;
use anyhow::anyhow;
use subprocess::Exec;

pub fn add(package_path: &Path, save_path: &str, server_name: &str) -> Result<()> {
    (Exec::shell(format!(
        "repo-add {server_name}.db.tar.gz {}",
        package_path.to_string_lossy()
    )))
    .cwd(save_path)
    .join()?;
    if let Some(package_path) = package_path.file_name() {
        let target = format!("{save_path}/{}", package_path.to_string_lossy());
        copy(package_path, target)?;
    } else {
        return Err(anyhow!("Failed to get package_name"));
    }
    Ok(())
}

pub fn remove(package_name: &str, save_path: &str, server_name: &str) -> Result<()> {
    (Exec::shell(format!(
        "repo-remove {server_name}.db.tar.gz {package_name}"
    )))
    .cwd(save_path)
    .join()?;
    Ok(())
}
