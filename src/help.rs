//! Print help message

pub fn show_help() {
    println!("oniri - A tool that automatically maximizes the only window of a niri workspace");
    println!();
    println!("USAGE:");
    println!("  Just start oniri from your niri config: spawn-at-startup \"oniri\"");
    println!();
    println!("See also the list of options that can be set (or passed from the CLI) below.");
    println!();
    println!("OPTIONS:");
    println!(
        "  -F, --first-only        Only maximize the first opened window, do not act on the last remaining one"
    );
    println!(
        "  -H, --height-tolerance  Set the height size tolerance (in pixels) when comparing the window size to the output size to determine if the window is maximized or not" // https://github.com/Antiz96/oniri/issues/3
    );
    println!(
        "  -W, --width-tolerance   Set the width size tolerance (in pixels) when comparing the window size to the output size to determine if the window is maximized or not" // https://github.com/Antiz96/oniri/issues/3
    );
    println!("  -h, --help              Display this help message");
    println!("  -V, --version           Display version information");
    println!();
    println!("For more information, see the oniri(1) man page.");
}
