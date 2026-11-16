use anyhow::Context;
use base64::Engine;
use ed25519_dalek::pkcs8::EncodePrivateKey;
use reqwest::blocking::Response;

use std::time::{SystemTime, UNIX_EPOCH};

const HOST: &str = "api.coinbase.com";

#[derive(serde::Serialize)]
struct Claims {
    sub: String,
    iss: String,
    nbf: u64,
    exp: u64,
    uri: String,
}

#[derive(serde::Deserialize)]
pub struct Key {
    id: String,
    #[serde(rename = "privateKey")]
    private_key: String,
}

fn build_jwt(api_key: &Key, method: &str, path: &str) -> anyhow::Result<String> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let uri = format!("{method} {HOST}{path}");

    let claims = Claims {
        sub: api_key.id.clone(),
        iss: "cdp".to_string(),
        nbf: now,
        exp: now + 120,
        uri,
    };

    // Set the header and key then sign the token
    let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA);
    header.kid = Some(api_key.id.clone());

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

pub fn send_request(api_key: &Key, path: &str) -> anyhow::Result<Response> {
    let token = build_jwt(api_key, "GET", path).context("key.json is not a valid CDP key file")?;

    let url = format!("https://{HOST}{path}");

    let http_client = reqwest::blocking::Client::new();

    let response = http_client.get(url).bearer_auth(token).send()?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};

    /// Fake secret for tests: 16 bytes of 7, base64-encoded. Never a real key.
    fn test_key(private_key: &str) -> Key {
        Key {
            id: "test-key-id".to_string(),
            private_key: private_key.to_string(),
        }
    }

    fn valid_private_key() -> String {
        STANDARD.encode([7u8; 64])
    }

    #[test]
    fn rejects_invalid_base64() {
        let key = test_key("not base64!");

        let result = build_jwt(&key, "GET", "/batch");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_wrong_length() {
        // valid base64, wrong length
        let key = test_key(&STANDARD.encode([7u8; 8]));

        let err = build_jwt(&key, "GET", "/batch").unwrap_err();

        assert!(err.to_string().contains("expected 64-byte key, got"));
    }

    #[test]
    fn route_claim_has_method_host_and_path() {
        let key = test_key(&valid_private_key());

        let token = build_jwt(&key, "GET", "/batch").unwrap();

        // take the middle part, decode it, and read the JSON
        let payload = token.split('.').nth(1).unwrap();
        let bytes = URL_SAFE_NO_PAD.decode(payload).unwrap();
        let claims: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(claims["uri"], "GET api.coinbase.com/batch");
    }
}
