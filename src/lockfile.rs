//! Create (if needed) and acquire lockfile
//! in order to prevent multiple instances to run at the same time

use anyhow::{Context, anyhow};
use fs2::FileExt;
use std::fs::{self, File, OpenOptions};
use std::io::ErrorKind;

pub fn acquire_lockfile() -> anyhow::Result<File> {
    // Create oniri cachedir (if it doesn't exist)
    let cachedir = dirs::cache_dir()
        .context("Failed to determine the cache directory")?
        .join("oniri");

    fs::create_dir_all(&cachedir).with_context(|| {
        format!(
            "Failed to create the {} cache directory",
            cachedir.display()
        )
    })?;

    let lockfile_path = cachedir.join("oniri.lock");

    let lockfile = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&lockfile_path)
        .with_context(|| format!("Failed to open the {} lockfile", lockfile_path.display()))?;

    lockfile.try_lock_exclusive().map_err(|error| {
        if error.kind() == ErrorKind::WouldBlock {
            anyhow!(error).context("Another instance of Oniri is already running")
        } else {
            anyhow!(error).context(format!(
                "Failed to acquire lock on the {} lockfile",
                lockfile_path.display()
            ))
        }
    })?;

    Ok(lockfile)
}
