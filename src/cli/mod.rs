use clap::{Parser, Subcommand};
mod commands;

#[derive(Debug, Parser)]
#[command(name = "auto-public-ip-update", bin_name = "apiu", author="Apollo-Roboto", version, about="Automatically update public ip address from cloudflare dns records", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command()]
    Current(commands::CurrentArguments),
    #[command()]
    Monitor(commands::MonitorArguments),
}

impl Commands {
    pub async fn run(&self) -> i32 {
        match self {
            Commands::Monitor(args) => commands::monitor_command(args).await,
            Commands::Current(args) => commands::current_command(args).await,
        }
    }
}

pub async fn run() -> i32 {
    let parsed_cli = Cli::parse();

    parsed_cli.command.run().await
}
