mod cli;
mod client;
mod commands;
mod keystore;

fn main() -> anyhow::Result<()> {
    let cli = <cli::Cli as clap::Parser>::parse();
    match cli.command {
        cli::Command::Auth { path, force } => commands::auth::run(&path, force),
        cli::Command::Orders {
            limit,
            side,
            ticker,
        } => commands::orders::run(limit, side, ticker),
    }
}
