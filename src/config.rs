use std::fs;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

use anyhow::{Context, Result, bail};
use etcetera::{AppStrategy, AppStrategyArgs, choose_app_strategy};

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
    let mut options = OpenOptions::new();
    options.write(true);

    if force {
        options.create(true).truncate(true);
    } else {
        options.create_new(true);
    }

    if let Some(folder) = dest.parent() {
        fs::create_dir_all(folder)
            .with_context(|| format!("failed to create {}", folder.display()))?;

        #[cfg(unix)]
        fs::set_permissions(folder, fs::Permissions::from_mode(0o700))
            .with_context(|| format!("failed to apply 700 permissions on {}", folder.display()))?;
    }

    // a newly created file starts out owner-only
    #[cfg(unix)]
    options.mode(0o600);

    let mut file = match options.open(dest) {
        Ok(file) => file,
        Err(err) if err.kind() == ErrorKind::AlreadyExists => {
            let msg: String = format!(
                "an API key is already saved at: {}\nrun again with --force to replace it",
                dest.display()
            );

            bail!(msg)
        }
        Err(err) => {
            return Err(err).with_context(|| format!("failed to write to {}", dest.display()));
        }
    };

    file.write_all(content.as_bytes())
        .with_context(|| format!("failed to write to {}", dest.display()))?;

    #[cfg(unix)]
    fs::set_permissions(dest, fs::Permissions::from_mode(0o600))
        .with_context(|| format!("failed to apply 600 permissions on {}", dest.display()))?;

    Ok(())
}

pub fn load_key() -> Result<Key> {
    let path = config_path()?;

    let content = fs::read_to_string(&path).with_context(|| {
        format!(
            "failed to read {}\nif no API key has been saved yet, run: coin auth <PATH>",
            path.display()
        )
    })?;

    let api_key: Key = parse_key(&content)?;

    Ok(api_key)
}

pub fn save_key(content: &str, force: bool) -> Result<PathBuf> {
    let dest = config_path()?;

    write_key(&dest, content, force)?;

    Ok(dest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn refuses_to_overwrite_without_force() {
        let dir = TempDir::new().unwrap();
        let dest = dir.path().join("coin-cli/cdp_api_key.json");

        write_key(&dest, "first", false).unwrap();
        let err = write_key(&dest, "second", false).err().unwrap();

        assert!(err.to_string().contains("--force"));
        assert_eq!(fs::read_to_string(&dest).unwrap(), "first");
    }

    #[test]
    fn force_replaces_existing_key() {
        let dir = TempDir::new().unwrap();
        let dest = dir.path().join("coin-cli/cdp_api_key.json");

        write_key(&dest, "first", false).unwrap();
        write_key(&dest, "second", true).unwrap();

        assert_eq!(fs::read_to_string(&dest).unwrap(), "second");
    }

    #[cfg(unix)]
    #[test]
    fn key_and_folder_are_owner_only() {
        let dir = TempDir::new().unwrap();
        let dest = dir.path().join("coin-cli/cdp_api_key.json");

        write_key(&dest, "secret", false).unwrap();

        let file = fs::metadata(&dest).unwrap().permissions().mode() & 0o777;
        let folder = fs::metadata(dir.path().join("coin-cli"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777;

        assert_eq!(file, 0o600);
        assert_eq!(folder, 0o700);
    }

    #[cfg(unix)]
    #[test]
    fn force_tightens_loose_permissions() {
        let dir = TempDir::new().unwrap();
        let dest = dir.path().join("coin-cli/cdp_api_key.json");

        write_key(&dest, "first", false).unwrap();
        fs::set_permissions(&dest, fs::Permissions::from_mode(0o644)).unwrap();

        write_key(&dest, "second", true).unwrap();

        let file = fs::metadata(&dest).unwrap().permissions().mode() & 0o777;
        assert_eq!(file, 0o600);
    }
}
