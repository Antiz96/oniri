// Import external modules
use std::env;

// Check if the -F / --first-only arg is passed
// Used later to determine if we only act on the first window or not
pub fn is_first_only() -> bool {
    env::args().any(|arg| arg == "-F" || arg == "--first-only")
}
