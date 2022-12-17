use super::super::Result;
use anyhow::anyhow;
use std::{fs::{copy, File}, path::Path, io::Write};
use subprocess::Exec;
use super::CLIENT;

pub fn add(package_path: &Path, save_path: &str, server_name: &str) -> Result<()> {
    if !package_path.to_string_lossy().ends_with(".pkg.tar.zst") {
        return Err(anyhow!(
            "Only files with the .pkg.tar.zst extension are allowed to operate."
        ));
    }
    (Exec::shell(format!(
        "repo-add {server_name}.db.tar.gz {}",
        package_path.to_string_lossy()
    )))
    .cwd(save_path)
    .join()?;
    if let Some(pkg_path) = package_path.file_name() {
        let target = format!("{save_path}/{}", pkg_path.to_string_lossy());
        copy(package_path, target)?;
    } else {
        return Err(anyhow!("Failed to get package_name"));
    }
    Ok(())
}

pub async fn add_with_url(package_url: &str, save_path: &str, server_name: &str) -> Result<()> {
    let bytes = CLIENT.get(package_url)
        .send()
        .await?
        .bytes()
        .await?;
    let Some(pkg_name) = package_url.split("/").last() else {
        return Err(anyhow!("Failed to get package_name"));
    };
    let pkg_path = format!("{save_path}/{pkg_name}");
    let mut file = File::create(&pkg_path)?;
    file.write_all(&bytes)?;
    file.flush()?;
    (Exec::shell(&format!("repo-add {server_name}.db.tar.gz {pkg_path}"))).cwd(save_path).join()?;
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
