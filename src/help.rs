// Import external modules
use std::env;

// Print help message if the -h / --help arg is passed
pub fn show_help() -> bool {
    if env::args().any(|arg| arg == "-h" || arg == "--help") {
        println!("oniri - A tool that automatically maximizes the only window of a niri workspace");
        println!();
        println!("USAGE:");
        println!("  Just start oniri from your niri config: spawn-at-startup \"oniri\"");
        println!();
        println!("See also the list of options that can be set (or passed from the CLI) below.");
        println!();
        println!("OPTIONS:");
        println!("  -F, --first-only        Only act on the first window");
        println!(
            "  -H, --height-tolerance  Set the height size tolerance (in pixels) when comparing the window size to the output size to determine if the window is maximized or not"
        );
        println!(
            "  -W, --width-tolerance   Set the width size tolerance (in pixels) when comparing the window size to the output size to determine if the window is maximized or not"
        );
        println!("  -h, --help              Display this help message");
        println!("  -V, --version           Display version information");
        println!();
        println!("For more information, see the oniri(1) man page.");
        return true;
    }
    false
}
