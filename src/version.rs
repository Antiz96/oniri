// Import modules
// std::env to access environment variables and cli arguments
// std::process to interact with processes
use std::{env, process};

// Define NAME & VERSION constants, fetched from Cargo metadata
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Print name and version if the -V / --version arg is passed
pub fn show_version() {
    if env::args().any(|arg| arg == "-V" || arg == "--version") {
        println!("{} {}", NAME, VERSION);
        process::exit(0);
    }
}
