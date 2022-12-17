use super::super::{anyhow, Result};
use git2::Repository;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::{
    fs::{copy, read_dir},
    path::PathBuf,
};
use subprocess::Exec;

static CLIENT: Lazy<Client> = Lazy::new(|| Client::builder().build().unwrap());

async fn fetch_pkgbuld(name: &str) -> Result<String> {
    let uri = format!("https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h={name}");
    let resp = CLIENT.get(uri).send().await?;
    let status = resp.status().as_u16();
    if status == 200 {
        Ok(resp.text().await?)
    } else {
        Ok("unknown".to_owned())
    }
}

pub async fn add(package_name: &str, package_save_path: &str, server_name: &str) -> Result<()> {
    let result = fetch_pkgbuld(&package_name).await?;
    if result == "unknown".to_owned() {
        return Err(anyhow!("Unknown package"));
    }
    let package_dir = PathBuf::from(format!("{package_save_path}/aur/pkgbuild/{package_name}"));
    let url = format!("https://aur.archlinux.org/{package_name}.git");
    Repository::clone(&url, &package_dir)?;
    (Exec::shell("makepkg PKGBUILD -f"))
        .cwd(package_dir.to_string_lossy().to_string())
        .join()?;
    for file in read_dir(&package_dir)? {
        let file = file?;
        if file
            .file_name()
            .to_string_lossy()
            .to_string()
            .ends_with("pkg.tar.zst")
        {
            println!("{}", file.path().to_string_lossy());
            (Exec::shell(&format!(
                "repo-add {server_name}.db.tar.gz {}",
                file.path().to_string_lossy()
            )))
            .cwd(&package_save_path)
            .join()?;
            copy(
                file.path(),
                format!(
                    "{}/{}",
                    &package_save_path,
                    file.file_name().to_string_lossy()
                ),
            )?;
        }
    }
    Ok(())
}

pub fn remove(package_name: &str, package_save_path: &str, server_name: &str) -> Result<()> {
    (Exec::shell(format!(
        "repo-remove {server_name}.db.tar.gz {package_name}"
    )))
    .cwd(&package_save_path)
    .join()?;
    Ok(())
}
