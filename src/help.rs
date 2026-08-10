//! Print help message

pub fn show_help() {
    println!("oniri - A tool that automatically maximizes the only window of a niri workspace");
    println!();
    println!("Start oniri from your niri config:");
    println!("`spawn-at-startup \"oniri\"`");
    println!("or");
    println!("`spawn-sh-at-startup \"oniri <options>\"`");
    println!();
    println!("OPTIONS:");
    println!(
        "  -F, --first-only                 Only maximize the first opened window, do not act on the last remaining one"
    );
    println!(
        "  -T, --tiling-layout              Unmaximize the first window when opening a second one, like in a tiling compositor"
    );
    println!("  -E, --edges-maximizing           Maximize windows to edges");
    println!(
        "  -R, --reclaim-space              Reclaim empty screen space left by closed windows"
    );
    println!(
        "  -H, --height-tolerance <number>  Set the height size tolerance (in pixels) when comparing the window size to the output size"
    );
    println!(
        "                                   to determine if the window is maximized or not (defaults to 150)"
    ); // https://github.com/Antiz96/oniri/issues/3
    println!(
        "  -W, --width-tolerance <number>   Set the width size tolerance (in pixels) when comparing the window size to the output size"
    );
    println!(
        "                                   to determine if the window is maximized or not (defaults to 150)"
    ); // https://github.com/Antiz96/oniri/issues/3
    println!("  -h, --help                       Display this help message");
    println!("  -V, --version                    Display version information");
    println!();
    println!("For more information, see the oniri(1) man page.");
}
