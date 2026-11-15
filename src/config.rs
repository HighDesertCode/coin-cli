use anyhow::Context;
use etcetera::{AppStrategy, AppStrategyArgs, choose_app_strategy};
use std::path::PathBuf;

use crate::client;

fn config_path() -> anyhow::Result<PathBuf> {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "io".to_string(),
        author: "HighDesertCode".to_string(),
        app_name: "coin-cli".to_string(),
    })?;

    let config_file = strategy.in_config_dir("key.json");

    Ok(config_file)
}

pub fn load_key() -> anyhow::Result<client::Key> {
    let path = config_path()?;

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    let api_key: client::Key = serde_json::from_str(&content)?;

    Ok(api_key)
}
