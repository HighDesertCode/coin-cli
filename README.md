<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/logo-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="assets/logo-light.svg">
  <img alt="coin-cli" src="assets/logo-light.svg">
</picture>

A Rust CLI tool for interacting with your Coinbase account.

This is an unofficial tool and isn't affiliated with or endorsed by Coinbase. 

[![Crates.io](https://img.shields.io/crates/v/coin-cli?color=dea584&style=flat-square)](https://crates.io/crates/coin-cli)
[![MSRV](https://img.shields.io/crates/msrv/coin-cli?color=dea584&style=flat-square)](https://crates.io/crates/coin-cli)

## Overview
Review the following for install, setup, and usage.

### Install
Requires Rust 1.85 or newer.

From `crates.io`:

```sh
cargo install coin-cli
```

### Setup
You will need to create and download an API key as a `.json` file from Coinbase. This CLI is currently intended for read-only operations.

1. Visit the [API keys](https://portal.cdp.coinbase.com/api-keys/secret) screen.
2. Create a new API key, give it a name, update allowed IP addresses, then click **Advanced Settings**.
3. Under **Signature Algorithm**, make sure that `Ed25519` is selected and choose view-only.
4. Click on **Create**, pass 2FA check, then click on **Download API Key**.
5. `coin` reads the key from the following directories:
    * Linux and macOS on `~/.config/coin-cli/`.
    * Windows on `%APPDATA%\HighDesertCode\coin-cli\config\`.

> [!CAUTION]
> Be mindful to handle your API key with care. Anyone with the file can access your account with the key's permissions.

### Usage
Run `coin --help` in the terminal to get more information on how to use the tool.

#### Commands
* `orders`
    * View historical orders