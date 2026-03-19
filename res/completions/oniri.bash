_oniri() {
	local arg="${2}"
	local -a opts
	opts=('-F --first-only
	       -H --height-tolerance
	       -W --width-tolerance
	       -h --help
	       -V --version')

	COMPREPLY=( $(compgen -W "${opts[*]}" -- "${arg}") )
}

complete -F _oniri oniri
