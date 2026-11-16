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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parse_orders() {
        let cli = Cli::try_parse_from(["coin", "orders"]).unwrap();

        assert!(matches!(cli.command, Command::Orders));
    }

    #[test]
    fn rejects_unknown_command() {
        assert!(Cli::try_parse_from(["coin", "delete"]).is_err());
    }
}
