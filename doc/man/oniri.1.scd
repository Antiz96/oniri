oniri(1) ["Oniri 1.0.4" ["Oniri Manual"]]

# NAME

*Oniri* - A tool that automatically maximizes the only window of a niri workspace.

# DESCRIPTION

Oniri is a tool that automatically maximizes the *on*ly window of a *niri* workspace, whether it's the first one opened or the last remaining one after all the other windows got closed (this behavior is configurable, see *OPTIONS* below).

# SYNOPSIS

*oniri* [OPTIONS]

# OPTIONS

*-F, --first-only*
	Only maximize the first window opened, do no act on the last remaining one.

*-H, --height-tolerance* <number>
	Set the height size tolerance (in pixels) when comparing the window size to the output size to determine if the window is maximized or not (https://github.com/Antiz96/oniri/issues/3).

*-W, --width-tolerance* <number>
	Set the width size tolerance (in pixels) when comparing the window size to the output size to determine if the window is maximized or not (https://github.com/Antiz96/oniri/issues/3).

*-h, --help*
	Display the help message.

*-V, --version*
	Display version information.

# SEE ALSO

https://niri-wm.github.io/niri

https://crates.io/crates/niri-ipc

# BUGS

Please report bugs to the GitHub page: https://github.com/Antiz96/oniri

# AUTHOR

Robin Candau <robincandau@protonmail.com>
