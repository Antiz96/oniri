complete -c oniri -f

complete -c oniri -s F -l first-only -d 'Only maximize the first window opened, do no act on the last remaining one'
complete -c oniri -s T -l tiling-layout -d 'Unmaximize the first window when opening a second one, like in a tiling compositor'
complete -c oniri -s H -l height-tolerance -d 'Set the height size tolerance (in pixels) when comparing the window size to the output size to determine if the window is maximized or not'
complete -c oniri -s W -l width-tolerance -d 'Set the width size tolerance (in pixels) when comparing the window size to the output size to determine if the window is maximized or not'
complete -c oniri -s h -l help -d 'Display the help message'
complete -c oniri -s V -l version -d 'Display version information'
