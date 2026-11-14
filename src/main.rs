mod cli;
mod client;
mod commands;

fn main() -> anyhow::Result<()> {
    let cli = <cli::Cli as clap::Parser>::parse();
    match cli.command {
        cli::Command::Orders => commands::orders::run(),
    }
}
