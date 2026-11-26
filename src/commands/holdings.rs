use anyhow::{Context, Result, bail};
use comfy_table::Table;
use serde::Deserialize;

use crate::client::{Key, send_request};
use crate::keystore::load_key;

const ENDPOINT: &str = "/api/v3/brokerage/portfolios";

#[derive(Deserialize)]
struct BreakdownResponse {
    breakdown: Breakdown,
}

#[derive(Deserialize)]
struct Breakdown {
    equity_positions: Vec<EquityPosition>,
    spot_positions: Vec<SpotPosition>,
}

#[derive(Deserialize)]
struct PortfoliosResponse {
    portfolios: Vec<Portfolio>,
}

#[derive(Deserialize)]
struct Portfolio {
    uuid: String,
    deleted: bool,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Deserialize)]
struct EquityPosition {
    cbrn: String,
    account_uuid: String,
    total_balance_equity: Amount,
    total_balance_fiat: Amount,
}

#[derive(Deserialize)]
struct SpotPosition {
    account_uuid: String,
    average_entry_price: Option<Amount>,
}

#[derive(Deserialize)]
struct Amount {
    value: String,
}

pub fn run() -> Result<()> {
    let api_key: Key = load_key()?;
    let portfolio_body: String = fetch(&api_key, ENDPOINT, &[])?;

    let parsed = parse_portfolios(&portfolio_body)?;

    let portfolio = find_portfolio(&parsed.portfolios)?;

    let breakdown_path = format!("{ENDPOINT}/{}", portfolio.uuid);

    let breakdown_body = fetch(&api_key, &breakdown_path, &[])?;

    let breakdown = parse_breakdown(&breakdown_body)?;

    println!("{}", breakdown.breakdown.equity_positions.len());
    println!("{}", breakdown.breakdown.spot_positions.len());

    Ok(())
}

fn fetch(api_key: &Key, path: &str, params: &[(&str, &str)]) -> Result<String> {
    let response = send_request(api_key, path, params)?;

    let status = response.status();
    let body = response.text()?;

    if status.is_success() {
        Ok(body)
    } else {
        bail!("GET {path} failed with {status}\nresponse body: {body}")
    }
}

fn parse_breakdown(body: &str) -> Result<BreakdownResponse> {
    let response: BreakdownResponse =
        serde_json::from_str(body).context("failed to parse the breakdown response")?;

    Ok(response)
}

fn parse_portfolios(body: &str) -> Result<PortfoliosResponse> {
    let response: PortfoliosResponse =
        serde_json::from_str(body).context("failed to parse the portfolios response")?;

    Ok(response)
}

fn find_portfolio(portfolios: &[Portfolio]) -> Result<&Portfolio> {
    portfolios
        .iter()
        .find(|p| p.kind == "DEFAULT" && !p.deleted)
        .context("no default portfolio found")
}
