use anyhow::{Context, Result, bail};
use etcetera::{AppStrategy, AppStrategyArgs, choose_app_strategy};
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use std::path::{Path, PathBuf};

use crate::client::{Key, parse_key};

fn config_path() -> Result<PathBuf> {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "io".to_string(),
        author: "HighDesertCode".to_string(),
        app_name: "coin-cli".to_string(),
    })?;

    let config_file = strategy.in_config_dir("cdp_api_key.json");

    Ok(config_file)
}

fn write_key(dest: &Path, content: &str, force: bool) -> Result<()> {
    if dest.exists() && !force {
        let msg: String = format!(
            "an API key is already saved at: {}\nrun again with --force to replace it",
            dest.display()
        );

        bail!(msg)
    }

    if let Some(folder) = dest.parent() {
        fs::create_dir_all(folder)
            .with_context(|| format!("failed to create {}", folder.display()))?;

        #[cfg(unix)]
        fs::set_permissions(folder, fs::Permissions::from_mode(0o700))
            .with_context(|| format!("failed to apply 700 permissions on {}", folder.display()))?;
    }

    fs::write(dest, content).with_context(|| format!("failed to write to {}", dest.display()))?;

    #[cfg(unix)]
    fs::set_permissions(dest, fs::Permissions::from_mode(0o600))
        .with_context(|| format!("failed to apply 600 permissions on {}", dest.display()))?;

    Ok(())
}

pub fn load_key() -> Result<Key> {
    let path = config_path()?;

    let content =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;

    let api_key: Key = parse_key(&content)?;

    Ok(api_key)
}

pub fn save_key(content: &str, force: bool) -> Result<PathBuf> {
    let dest = config_path()?;

    write_key(&dest, content, force)?;

    Ok(dest)
}
