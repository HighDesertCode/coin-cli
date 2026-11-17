use clap::{Parser, Subcommand};

use std::path::PathBuf;

/// a Rust CLI for the Coinbase Advanced Trade API
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// sets up `coin` with the API key file downloaded from Coinbase
    Auth {
        /// path to the API key file downloaded from Coinbase
        path: PathBuf,

        /// force replace the saved key if one already exists
        #[arg(short, long)]
        force: bool,
    },

    /// lists all orders from the account
    Orders,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    
    use std::path::Path;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    // ------ tests for the auth command ------
    #[test]
    fn rejects_auth_without_path() {
        assert!(Cli::try_parse_from(["coin", "auth"]).is_err());
    }

    #[test]
    fn parse_auth_with_path() {
        let cli = Cli::try_parse_from(["coin", "auth", "cdp_api_key.json"]).unwrap();

        assert!(matches!(
            cli.command,
            Command::Auth { ref path, force: false } if path == Path::new("cdp_api_key.json")
        ));
    }

    #[test]
    fn parse_auth_with_force() {
        for flag in ["--force", "-f"] {
            let cli = Cli::try_parse_from(["coin", "auth", "cdp_api_key.json", flag]).unwrap();

            assert!(matches!(cli.command, Command::Auth { force: true, .. }));
        }
    }

    // ------ tests for the orders command ------
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
