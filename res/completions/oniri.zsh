#compdef oniri

local -a opts
opts=(
    {-F,--first-only}'[Only maximize the first window opened, do no act on the last remaining one]'
    {-T,--tiling-layout}'[Unmaximize the first window when opening a second one, like in a tiling compositor]'
    {-E,--edges-maximizing}'[Maximize windows to edges]'
    {-H,--height-tolerance}'[Set the height size tolerance (in pixels) when comparing the window size to the output size to determine if the window is maximized or not]'
    {-W,--width-tolerance}'[Set the width size tolerance (in pixels) when comparing the window size to the output size to determine if the window is maximized or not]'
    {-h,--help}'[Display the help message]'
    {-V,--version}'[Display version information]'
)

_arguments $opts
