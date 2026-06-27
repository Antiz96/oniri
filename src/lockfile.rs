//! Create (if needed) and acquire lockfile
//! in order to prevent multiple instances to run at the same time

use fs2::FileExt;
use std::fs::{self, File, OpenOptions};

pub fn acquire_lockfile() -> std::io::Result<File> {
    // Create oniri cachedir (if it doesn't exist)
    let cachedir = dirs::cache_dir()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "could not determine cache directory",
            )
        })?
        .join("oniri");
    fs::create_dir_all(&cachedir)?;

    let lockfile_path = cachedir.join("oniri.lock");

    let lockfile = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(lockfile_path)?;

    lockfile.try_lock_exclusive().map_err(|err| {
        if err.kind() == std::io::ErrorKind::WouldBlock {
            std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "oniri is already running",
            )
        } else {
            err
        }
    })?;

    Ok(lockfile)
}
