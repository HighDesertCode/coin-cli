use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn help_mentions_orders() {
    Command::cargo_bin("coin")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains(
            "A Rust CLI tool for interacting with your Coinbase account",
        ));
}
