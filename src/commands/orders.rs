use anyhow::Context;

use crate::client;

pub fn run() -> anyhow::Result<()> {
    let url = "https://api.coinbase.com/api/v3/brokerage/orders/historical/batch";

    let home = std::env::var("HOME")?;
    let content = std::fs::read_to_string(format!("{home}/coinbase/key.json"))
        .context("failed to read ~/coinbase/key.json")?;

    let api_key: client::Key = serde_json::from_str(&content)?;

    let token = client::build_jwt(&api_key, "GET", "/api/v3/brokerage/orders/historical/batch")
        .context("key.json is not a valid CDP key file")?;

    let http_client = reqwest::blocking::Client::new();

    let response = http_client.get(url).bearer_auth(token).send()?;

    let status = response.status();
    let body: String = response.text()?;

    if status.is_success() {
        println!("Success! Response body:\n{body}");
    } else {
        println!("Failure. Response body:\n{body}");
    }

    Ok(())
}
