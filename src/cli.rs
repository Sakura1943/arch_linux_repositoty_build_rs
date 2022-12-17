use clap::{Args, Command, CommandFactory, Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub sub_commands: Option<SubCommands>,
    #[arg(short, long, help = "Configuration file path")]
    pub config_file: Option<String>,
    #[arg(short, long, help = "Show all packages")]
    pub show_all: bool
}

#[derive(Subcommand, Clone)]
pub enum SubCommands {
    #[command(about = "Aur package operations")]
    Aur(Aur),
    #[command(about = "Official package operations")]
    Official(Official),
}

#[derive(Clone, Args)]
pub struct Aur {
    #[arg(help = "Aur package name")]
    pub package_name: String,
    #[arg(short, long, help = "Add aur package to repository")]
    pub add: bool,
    #[arg(short, long, help = "Remove package from repository")]
    pub remove: bool
}

#[derive(Clone, Args)]
pub struct Official {
    #[arg(help = "Official package name")]
    pub package_name: String,
    #[arg(short, long, help = "Add official package to repository")]
    pub add: bool,
    #[arg(short, long, help = "Remove package from repository")]
    pub remove: bool
}

#[allow(dead_code)]
impl Cli {
    pub fn build() -> Self {
        Self::parse()
    }

    pub fn cmds() -> Command {
        Self::command()
    }
}
