{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		utils.url = "github:numtide/flake-utils";
	};

	outputs = { nixpkgs, utils, ... }: utils.lib.eachDefaultSystem(system: let
		pkgs = import nixpkgs {
			inherit system;
		};
	in {
		devShell = pkgs.mkShell rec {
			nativeBuildInputs = with pkgs; [ rustup pkg-config ];

			buildInputs = with pkgs; [
				xorg.libX11
				xorg.libXi
				xorg.libXtst
				xorg.libXcursor
				xorg.libXrandr
				vulkan-loader
				libGL
			];

			LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath buildInputs}";

			shellHook = ''
				rustup toolchain install stable
				rustup toolchain install --profile minimal nightly
				rustup default stable
				rustup component add rust-analyzer
				rustup +nightly component add rustfmt
				rustup update
			'';
		};
	});
}
