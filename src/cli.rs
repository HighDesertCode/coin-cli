use clap::{Parser, Subcommand, ValueEnum};

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
    Orders {
        /// maximum number of orders to show
        #[arg(short = 'l', long, default_value_t = 20)]
        limit: u32,

        /// filter equities by buy or sell transactions
        #[arg(long, value_enum)]
        side: Option<Side>,

        /// Only show orders for this ticker, among the orders fetched
        #[arg(short, long, value_name = "TICKER")]
        ticker: Option<String>,
    },
}

// extra set up for flags
#[derive(Clone, Copy, ValueEnum)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    /// the two different side options to filter orders
    pub fn side_filter(self) -> &'static str {
        match self {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        }
    }
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

        assert!(matches!(cli.command, Command::Orders { .. }));
    }

    #[test]
    fn rejects_unknown_command() {
        assert!(Cli::try_parse_from(["coin", "delete"]).is_err());
    }

    #[test]
    fn orders_limit_defaults_to_20() {
        let cli = Cli::try_parse_from(["coin", "orders"]).unwrap();

        assert!(matches!(cli.command, Command::Orders { limit: 20, .. }));
    }

    #[test]
    fn orders_accepts_limit() {
        let cli = Cli::try_parse_from(["coin", "orders", "--limit", "5"]).unwrap();

        assert!(matches!(cli.command, Command::Orders { limit: 5, .. }));
    }

    #[test]
    fn orders_rejects_a_limit_that_is_not_a_number() {
        assert!(Cli::try_parse_from(["coin", "orders", "--limit", "abc"]).is_err());
    }

    #[test]
    fn orders_side_defaults_to_none() {
        let cli = Cli::try_parse_from(["coin", "orders"]).unwrap();

        assert!(matches!(cli.command, Command::Orders { side: None, .. }));
    }

    #[test]
    fn orders_accepts_side() {
        let cli = Cli::try_parse_from(["coin", "orders", "--side", "sell"]).unwrap();

        assert!(matches!(
            cli.command,
            Command::Orders {
                side: Some(Side::Sell),
                ..
            }
        ));
    }

    #[test]
    fn orders_rejects_an_unknown_side() {
        assert!(Cli::try_parse_from(["coin", "orders", "--side", "hold"]).is_err());
    }

    #[test]
    fn side_uses_the_spelling_coinbase_expects() {
        assert_eq!(Side::Buy.side_filter(), "BUY");
        assert_eq!(Side::Sell.side_filter(), "SELL");
    }

    #[test]
    fn orders_accepts_ticker() {
        let cli = Cli::try_parse_from(["coin", "orders", "--ticker", "NVDA"]).unwrap();

        assert!(matches!(cli.command, Command::Orders { ticker: Some(ref t), .. } if t == "NVDA"));
    }

    #[test]
    fn orders_ticker_defaults_to_none() {
        let cli = Cli::try_parse_from(["coin", "orders"]).unwrap();

        assert!(matches!(cli.command, Command::Orders { ticker: None, .. }));
    }
}
