use clap::{Parser, Subcommand};

/// a command line interface for your Coinbase account
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// lists all orders from the account
    Orders,
}
