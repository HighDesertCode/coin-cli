use anyhow::{Result, bail};

use crate::client::send_request;
use crate::keystore::load_key;

const ENDPOINT: &str = "/api/v3/brokerage/orders/historical/batch";

pub fn run() -> Result<()> {
    let api_key = load_key()?;
    let response = send_request(&api_key, ENDPOINT)?;

    let status = response.status();
    let body = response.text()?;

    if status.is_success() {
        println!("Success! Response body:\n{body}");
    } else {
        bail!("GET {ENDPOINT} failed with {status}\nresponse body: {body}");
    }

    Ok(())
}
