// Import modules
// std::env to access environment variables and cli arguments
use std::env;

// Define NAME & VERSION constants, fetched from Cargo metadata
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Print name and version if the -V / --version arg is passed
pub fn show_version() -> bool {
    if env::args().any(|arg| arg == "-V" || arg == "--version") {
        println!("{} {}", NAME, VERSION);
        return true;
    }
    false
}
