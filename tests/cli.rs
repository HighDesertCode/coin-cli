use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn help_lists_commands() {
    Command::cargo_bin("coin")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("A Rust CLI for the Coinbase Advanced Trade API"))
        .stdout(contains("orders"))
        .stdout(contains("auth"));
}

#[cfg(unix)]
#[test]
fn auth_saves_key() {
    let home = tempfile::TempDir::new().unwrap();
    let download = home.path().join("cdp_api_key.json");
    let key = r#"{ "id": "test-key-id", "privateKey": "BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBw==" }"#;

    std::fs::write(&download, key).unwrap();

    Command::cargo_bin("coin")
        .unwrap()
        .env("HOME", home.path())
        .env("XDG_CONFIG_HOME", home.path().join("config"))
        .arg("auth")
        .arg(&download)
        .assert()
        .success()
        .stdout(contains("Saved the API key"));

    let saved = home.path().join("config/coin-cli/cdp_api_key.json");
    assert_eq!(std::fs::read_to_string(saved).unwrap(), key);
}

#[cfg(windows)]
#[test]
fn auth_saves_key() {
    let home = tempfile::TempDir::new().unwrap();
    let download = home.path().join("cdp_api_key.json");
    let key = r#"{ "id": "test-key-id", "privateKey": "BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBw==" }"#;

    std::fs::write(&download, key).unwrap();

    Command::cargo_bin("coin")
        .unwrap()
        .env("APPDATA", home.path().join("config"))
        .arg("auth")
        .arg(&download)
        .assert()
        .success()
        .stdout(contains("Saved the API key"));

    let saved = home.path().join("config\\HighDesertCode\\coin-cli\\config\\cdp_api_key.json");
    assert_eq!(std::fs::read_to_string(saved).unwrap(), key);
}