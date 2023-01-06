use arb::{
    anyhow, basic_config,
    cli::{Cli, SubCommands},
    commands::{aur, custom, official},
    Result, CUSTOM_PATH,
};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input};
use flate2::read::GzDecoder;
use serde::Deserialize;
use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::PathBuf,
};
use tar::Archive;

#[derive(Deserialize)]
struct Config {
    basic: Basic,
}

#[derive(Deserialize)]
struct Basic {
    save_path: String,
    server_name: String,
    mirror_server: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::build();
    let mut cmd = Cli::cmds();
    let custom_config_path = if let Some(config_path) = cli.config_file {
        PathBuf::from(&config_path)
    } else {
        PathBuf::from(&CUSTOM_PATH.to_owned()).join("config/basic.toml")
    };

    if let Some(parent) = custom_config_path.clone().parent() {
        create_dir_all(parent)?;
    }

    if !custom_config_path.clone().exists() {
        let server_name = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Server name?")
            .interact_on(&Term::stderr())?;
        let server = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Server url?")
            .default("https://mirrors.bfsu.edu.cn/archlinux".to_owned())
            .interact_on(&Term::stderr())?;
        let mut file = File::create(&custom_config_path)?;
        file.write_all(basic_config(&server_name, &server).as_bytes())?;
    }

    let config_content = read_to_string(&custom_config_path)?;

    let config = toml::from_str::<Config>(&config_content)?;

    let package_save_path = config.basic.save_path;
    let server_name = config.basic.server_name;
    let mirror_server = config.basic.mirror_server;

    if !PathBuf::from(&package_save_path).exists() {
        create_dir_all(&package_save_path)?;
    }

    if let Some(subcommands) = cli.sub_commands {
        match subcommands {
            SubCommands::Aur(_aur) => match (_aur.add, _aur.remove) {
                (true, false) => {
                    for pkg_name in _aur.package_name {
                        aur::add(&pkg_name, &package_save_path, &server_name).await?;
                    }
                }
                (false, true) => {
                    for pkg_name in _aur.package_name {
                        aur::remove(&pkg_name, &package_save_path, &server_name)?;
                    }
                }
                _ => {
                    cmd.print_help()?;
                    return Ok(());
                }
            },
            SubCommands::Official(_official) => match (_official.add, _official.remove) {
                (true, false) => {
                    for pkg_name in _official.package_name {
                        official::add(&pkg_name, &mirror_server, &package_save_path, &server_name)
                            .await?;
                    }
                }
                (false, true) => {
                    for pkg_name in _official.package_name {
                        official::remove(&pkg_name, &package_save_path, &server_name)?;
                    }
                }
                _ => {
                    cmd.print_help()?;
                    return Ok(());
                }
            },
            SubCommands::Custom(_custom) => match (_custom.add, _custom.remove) {
                (true, false) => {
                    if let Some(package_path) = _custom.package_path {
                        for pkg_path in package_path {
                            custom::add(&pkg_path, &package_save_path, &server_name)?;
                        }
                    } else if let Some(package_url) = _custom.package_url {
                        for pkg_url in package_url {
                            custom::add_with_url(&pkg_url, &package_save_path, &server_name).await?;
                        }
                    } else {
                        return Err(anyhow!(
                            "Failed to add package, cause `package_path` or `package_url` is not provided."
                        ));
                    }
                }
                (false, true) => {
                    if let Some(package_name) = _custom.package_name {
                        for pkg_name in package_name {
                            custom::remove(&pkg_name, &package_save_path, &server_name)?;
                        }
                    } else {
                        return Err(anyhow!(
                            "Failed to remove package, cause `package_name` is not provided."
                        ));
                    }
                }
                _ => {
                    cmd.print_help()?;
                    return Ok(());
                }
            },
        }
    }
    if cli.show_all {
        let tar_gz = File::open(format!("{package_save_path}/{server_name}.db.tar.gz")).unwrap();
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        for file in archive.entries()? {
            let file = file?;
            println!("{:#?}", file.header().path()?);
        }
    }
    Ok(())
}
