use anyhow::{Context, Result};

use std::fs;
use std::path::{Path, PathBuf};

use crate::client::parse_key;
use crate::config::save_key;

pub fn run(path: &Path, force: bool) -> Result<()> {
    let content: String =
        fs::read_to_string(path).with_context(|| format!("unable to read {}", path.display()))?;

    parse_key(&content)?;

    let saved_location: PathBuf = save_key(&content, force)?;

    println!(
        "Saved the API key to {}\nThe downloaded file still contains the secret. Delete once verified working.",
        saved_location.display()
    );

    Ok(())
}
