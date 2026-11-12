use std::time::{SystemTime, UNIX_EPOCH};

const KEY_NAME: &str = "organizations/REDACTED_ORG_ID/apiKeys/REDACTED_KEY_ID";

#[derive(serde::Serialize)]
struct Claims {
    sub: String,
    iss: String,
    nbf: u64,
    exp: u64,
    uri: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.coinbase.com/api/v3/brokerage/orders/historical/batch";

    let home = std::env::var("HOME")?;

    let key_secret = std::fs::read_to_string(format!("{}/.config/coinbase-cli/key.pem", home))?;

    let token = build_jwt(
        KEY_NAME,
        &key_secret,
        "GET",
        "/api/v3/brokerage/orders/historical/batch",
    )?;

    let client = reqwest::blocking::Client::new();

    let response = client.get(url).bearer_auth(token).send()?;

    let status = response.status();
    let body: String = response.text()?;

    if status.is_success() {
        println!("Success! Response body:\n{}", body);
    } else {
        println!("Failure. Response body:\n{}", body);
    }

    Ok(())
}

fn build_jwt(
    key_name: &str,
    key_secret: &str,
    method: &str,
    path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // 1. time
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    // 2. uri: "GET api.coinbase.com/api/v3..."
    let uri = format!("{} api.coinbase.com{}", method, path);

    // 3. Claims
    let claims = Claims {
        sub: key_name.to_string(),
        iss: "cdp".to_string(),
        nbf: now,
        exp: now + 120,
        uri,
    };

    // 4. header
    let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::ES256);
    header.kid = Some(key_name.to_string());

    // 5. key
    let key = jsonwebtoken::EncodingKey::from_ec_pem(key_secret.as_bytes())?;

    // 6. sign
    let token = jsonwebtoken::encode(&header, &claims, &key)?;

    Ok(token)
}
