use super::super::{Lazy, Result};
use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use subprocess::Exec;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder().user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36").build().unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    version: u8,
    limit: u16,
    valid: bool,
    results: Vec<Results>,
    num_pages: u16,
    page: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct Results {
    pkgname: String,
    pkgbase: String,
    repo: String,
    arch: String,
    pkgver: String,
    pkgrel: String,
    epoch: u16,
    pkgdesc: String,
    url: String,
    filename: String,
    compressed_size: u64,
    installed_size: u64,
    build_date: String,
    last_update: String,
    flag_date: Option<String>,
    maintainers: Vec<String>,
    packager: String,
    groups: Vec<String>,
    licenses: Vec<String>,
    conflicts: Vec<String>,
    provides: Vec<String>,
    replaces: Vec<String>,
    depends: Vec<String>,
    optdepends: Vec<String>,
    makedepends: Vec<String>,
    checkdepends: Vec<String>,
}

struct PackageInfo {
    file_name: String,
    repo: String,
}

async fn fetch_package_list(package_name: &str) -> Result<PackageInfo> {
    let uri = format!("https://archlinux.org/packages/search/json/?name={package_name}");
    let resp = CLIENT.get(&uri).send().await?.json::<Response>().await?;
    if resp.results.len() == 0 {
        return Err(anyhow!("Failed to get package {}", package_name));
    }
    let Some(package_result) = resp.results.into_iter().nth(0) else {
        return Err(anyhow!("Failed to get result nth `0` from package {package_name}"));
    };
    Ok(PackageInfo {
        file_name: package_result.filename,
        repo: package_result.repo,
    })
}

pub async fn add(
    package_name: &str,
    server_path: &str,
    save_path: &str,
    server_name: &str,
) -> Result<()> {
    let package_info = fetch_package_list(package_name).await?;
    let file_name = package_info.file_name;
    let repo = package_info.repo;
    let pkg_uri = format!("{server_path}/{repo}/os/x86_64/{file_name}");
    println!("Download url: {pkg_uri}");
    let sig_uri = format!("{pkg_uri}.sig");
    let pkg_bytes = CLIENT.get(&pkg_uri).send().await?.bytes().await?;
    let sig_bytes = CLIENT.get(&sig_uri).send().await?.bytes().await?;
    let pkg_save_path = format!("{save_path}/{file_name}");
    let sig_save_path = format!("{pkg_save_path}.sig");
    let mut pkg = File::create(&pkg_save_path)?;
    let mut sig = File::create(&sig_save_path)?;
    pkg.write_all(&pkg_bytes)?;
    pkg.flush()?;
    sig.write_all(&sig_bytes)?;
    sig.flush()?;
    (Exec::shell(&format!("repo-add {server_name}.db.tar.gz {pkg_save_path}")))
        .cwd(&save_path)
        .join()?;

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
