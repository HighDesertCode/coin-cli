use std::time::{SystemTime, UNIX_EPOCH};

use ::base64::Engine;
use ::ed25519_dalek::pkcs8::EncodePrivateKey;

use anyhow::Context;

#[derive(serde::Serialize)]
struct Claims {
    sub: String,
    iss: String,
    nbf: u64,
    exp: u64,
    uri: String,
}

#[derive(serde::Deserialize)]
struct Key {
    id: String,
    #[serde(rename = "privateKey")]
    private_key: String,
}

fn main() -> anyhow::Result<()> {
    let url = "https://api.coinbase.com/api/v3/brokerage/orders/historical/batch";

    let home = std::env::var("HOME")?;
    let content = std::fs::read_to_string(format!("{}/coinbase/key.json", home))
        .context("failed to read ~/coinbase/key.json")?;

    let api_key: Key = serde_json::from_str(&content)?;

    let token = build_jwt(&api_key, "GET", "/api/v3/brokerage/orders/historical/batch")
        .context("key.json is not a valid CDP key file")?;

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

fn build_jwt(api_key: &Key, method: &str, path: &str) -> anyhow::Result<String> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let uri = format!("{} api.coinbase.com{}", method, path);

    let claims = Claims {
        sub: api_key.id.to_string(),
        iss: "cdp".to_string(),
        nbf: now,
        exp: now + 120,
        uri,
    };

    // Set the header and key then sign the token
    let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA);
    header.kid = Some(api_key.id.to_string());

    let bytes = base64::engine::general_purpose::STANDARD.decode(&api_key.private_key)?;
    anyhow::ensure!(
        bytes.len() == 64,
        "expected 64-byte key, got {}",
        bytes.len()
    );

    let seed: [u8; 32] = bytes[..32].try_into()?;

    let signing_key = ed25519_dalek::SigningKey::from_bytes(&seed);
    let der = signing_key.to_pkcs8_der()?;
    let key = jsonwebtoken::EncodingKey::from_ed_der(der.as_bytes());

    let token = jsonwebtoken::encode(&header, &claims, &key)?;

    Ok(token)
}
