{ pkgs }: {
	deps = [
		pkgs.gcc
		pkgs.ccls
		pkgs.gdb
        pkgs.cmake
        pkgs.neovim
	];
}