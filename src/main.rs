mod cli;
mod client;
mod commands;
mod config;

fn main() -> anyhow::Result<()> {
    let cli = <cli::Cli as clap::Parser>::parse();
    match cli.command {
        cli::Command::Auth { path, force }=> commands::auth::run(&path, force),
        cli::Command::Orders => commands::orders::run(),
    }
}
