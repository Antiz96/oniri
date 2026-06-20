//! Show version

use std::env;

pub fn show_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}
