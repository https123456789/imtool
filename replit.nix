{ pkgs }: {
	deps = [
        pkgs.neovim
        pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
        pkgs.tokei
        pkgs.libheif
	];
}