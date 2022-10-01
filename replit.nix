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
        pkgs.hexdump
        pkgs.imagemagick6_light
	];
    env = {
        CARGO_HOME = "/home/runner/imtool/.cargo";
        RUST_BACKTRACE = 1;
    };
}