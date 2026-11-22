use anyhow::{Context, Result, bail};
use comfy_table::Table;
use serde::Deserialize;

use crate::client::send_request;
use crate::keystore::load_key;

const ENDPOINT: &str = "/api/v3/brokerage/orders/historical/batch";
const QUERY: (&str, &str) = ("product_type", "EQUITY");

#[derive(Deserialize)]
struct Response {
    orders: Vec<Order>,
}

#[derive(Deserialize)]
struct Order {
    side: String,
    filled_value: String,
    filled_size: String,
    created_time: String,
    #[serde(rename = "order_type")]
    typing: String,
    product_details: ProductDetails,
}

#[derive(Deserialize)]
struct ProductDetails {
    equity_details: EquityDetails,
}

#[derive(Deserialize)]
struct EquityDetails {
    ticker: String,
}

pub fn run() -> Result<()> {
    let api_key = load_key()?;
    let response = send_request(&api_key, ENDPOINT, &[QUERY])?;

    let status = response.status();
    let body = response.text()?;

    if status.is_success() {
        let orders = parse_orders(&body)?;
        build_table(&orders);
    } else {
        bail!("GET {ENDPOINT} failed with {status}\nresponse body: {body}");
    }

    Ok(())
}

fn parse_orders(body: &str) -> Result<Vec<Order>> {
    let response: Response =
        serde_json::from_str(body).context("failed to parse the orders response")?;

    Ok(response.orders)
}

fn build_table(orders: &[Order]) {
    let mut table = Table::new();
    table.set_header(vec![
        "Activity", "Ticker", "Shares", "Amount", "Type", "DateTime",
    ]);

    for order in orders {
        let ticker = order.product_details.equity_details.ticker.as_str();

        table.add_row(vec![
            order.side.as_str(),
            ticker,
            order.filled_size.as_str(),
            order.filled_value.as_str(),
            order.typing.as_str(),
            order.created_time.as_str(),
        ]);
    }

    println!("{table}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_orders_response() {
        let body = include_str!("../../tests/fixtures/orders.json");

        let orders = parse_orders(body).unwrap();

        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0].side, "BUY");
        assert_eq!(orders[0].filled_size, "2");
        assert_eq!(orders[1].product_details.equity_details.ticker, "GLOBX");
    }

    #[test]
    fn rejects_invalid_response() {
        let err = parse_orders("this is invalid json").err().unwrap();

        assert!(
            err.to_string()
                .contains("failed to parse the orders response")
        );
    }

    #[test]
    fn parses_an_empty_order_list() {
        let orders = parse_orders(r#"{ "orders": [] }"#).unwrap();

        assert!(orders.is_empty());
    }
}
