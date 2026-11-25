use anyhow::{Context, Result, bail};
use comfy_table::Table;
use serde::Deserialize;

use crate::client::send_request;
use crate::keystore::load_key;

const ENDPOINT: &str = "/api/v3/brokerage/orders/historical/batch";
const QUERY: (&str, &str) = ("product_type", "EQUITY");

#[derive(Deserialize)]
struct OrdersResponse {
    has_next: bool,
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

pub fn run(limit: u32) -> Result<()> {
    let mut params: Vec<(&str, &str)> = Vec::new();

    params.push(QUERY);

    // limit flag
    let limit = limit.to_string();
    params.push(("limit", &limit));

    let api_key = load_key()?;
    let response = send_request(&api_key, ENDPOINT, &params)?;

    let status = response.status();
    let body = response.text()?;

    if status.is_success() {
        let response = parse_orders(&body)?;

        if response.orders.is_empty() {
            println!("No orders to show");
        } else {
            build_table(&response.orders);

            if response.has_next {
                println!("More orders exist than are shown.");
            }
        }
    } else {
        bail!("GET {ENDPOINT} failed with {status}\nresponse body: {body}");
    }

    Ok(())
}

fn parse_orders(body: &str) -> Result<OrdersResponse> {
    let response: OrdersResponse =
        serde_json::from_str(body).context("failed to parse the orders response")?;

    Ok(response)
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

        let response = parse_orders(body).unwrap();

        assert_eq!(response.orders.len(), 2);
        assert_eq!(response.orders[0].side, "BUY");
        assert_eq!(response.orders[0].filled_size, "2");
        assert_eq!(
            response.orders[1].product_details.equity_details.ticker,
            "GLOBX"
        );
        assert!(!response.has_next);
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
        let response = parse_orders(r#"{ "orders": [], "has_next": false }"#).unwrap();

        assert!(response.orders.is_empty());
    }

    #[test]
    fn reads_has_next_when_more_orders_exist() {
        let response = parse_orders(r#"{ "orders": [], "has_next": true }"#).unwrap();

        assert!(response.has_next);
    }
}
