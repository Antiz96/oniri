// Import the env module to access environment variables and arguments
use std::env;

// Define NAME & VERSION constant variables, fetched from Cargo metadata
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Gather the argument passed from the command line
    let args: Vec<String> = env::args().collect();

    // Print name and version if the -V / --version arg is passed
    if args.iter().any(|a| a == "-V" || a == "--version") {
        println!("{} {}", NAME, VERSION);
    }
}
