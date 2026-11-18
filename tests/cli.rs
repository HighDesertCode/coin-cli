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
