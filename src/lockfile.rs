//! Create (if needed) and acquire lockfile
//! in order to prevent multiple instances to run at the same time

use fs2::FileExt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Error, ErrorKind};

pub fn acquire_lockfile() -> io::Result<File> {
    // Create oniri cachedir (if it doesn't exist)
    let cachedir = dirs::cache_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not determine cache directory"))?
        .join("oniri");
    fs::create_dir_all(&cachedir)?;

    let lockfile_path = cachedir.join("oniri.lock");

    let lockfile = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(lockfile_path)?;

    lockfile.try_lock_exclusive().map_err(|error| {
        if error.kind() == ErrorKind::WouldBlock {
            Error::from(ErrorKind::AlreadyExists)
        } else {
            error
        }
    })?;

    Ok(lockfile)
}
