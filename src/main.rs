use arb::{
    cli::{Cli, SubCommands},
    commands::{aur, official, custom},
    Result, BASIC_CONFIG, CUSTOM_PATH, anyhow,
};
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
        let mut file = File::create(&custom_config_path)?;
        file.write_all(BASIC_CONFIG.as_bytes())?;
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
                    aur::add(&_aur.package_name, &package_save_path, &server_name).await?;
                }
                (false, true) => {
                    aur::remove(&_aur.package_name, &package_save_path, &server_name)?;
                }
                (true, true) => {
                    cmd.print_help()?;
                    return Ok(());
                }
                (false, false) => {
                    cmd.print_help()?;
                    return Ok(());
                }
            },
            SubCommands::Official(_official) => match (_official.add, _official.remove) {
                (true, false) => {
                    official::add(
                        &_official.package_name,
                        &mirror_server,
                        &package_save_path,
                        &server_name,
                    )
                    .await?;
                }
                (false, true) => {
                    official::remove(&_official.package_name, &package_save_path, &server_name)?;
                }
                (true, true) => {
                    cmd.print_help()?;
                    return Ok(());
                }
                (false, false) => {
                    cmd.print_help()?;
                    return Ok(());
                }
            },
            SubCommands::Custom(_custom) => {
                match(_custom.add, _custom.remove) {
                    (true, false) => {
                        custom::add(&_custom.package_path, &package_save_path, &server_name)?;
                    },
                    (false, true) => {
                        if let Some(package_name) = _custom.package_name {
                            custom::remove(&package_name, &package_save_path, &server_name)?;
                        } else {
                            return Err(anyhow!("Failed to remove package, cause `pacakge_name` is no t provided."))
                        }
                    },
                    (true, true) => {
                        cmd.print_help()?;
                        return Ok(());
                    },
                    (false, false) => {
                        cmd.print_help()?;
                        return Ok(());
                    }
                }
            }
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
