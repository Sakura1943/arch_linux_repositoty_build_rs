pub mod cli;
pub use anyhow::{anyhow, Result};
pub mod commands;
use once_cell::sync::Lazy;
use std::env;

pub static CUSTOM_PATH: Lazy<String> = Lazy::new(|| {
    let home_path = env::var("HOME").unwrap();
    format!("{home_path}/.local/share/arch_linux_repository_build")
});

pub fn basic_config(server_name: &str, server_url: &str) -> String {
    format!(
        "[basic]
server_name = \"{server_name}\"
save_path = \"{}\"
mirror_server = \"{server_url}\"",
        format!("{}/repository", format!("{}", CUSTOM_PATH.to_owned()))
    )
}
