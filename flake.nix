{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		utils.url = "github:numtide/flake-utils";
	};

	outputs = { nixpkgs, utils, ... }: utils.lib.eachDefaultSystem(system: let
		cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));

		pkgs = import nixpkgs {
			inherit system;
		};
		
		deps = with pkgs; [
			xorg.libX11
			xorg.libXi
			xorg.libXtst
			xorg.libXcursor
			xorg.libXrandr
			vulkan-loader
			libGL
		];

		LD_PATH = "${nixpkgs.lib.makeLibraryPath deps}";

		keyscreen = pkgs.rustPlatform.buildRustPackage rec {
			inherit (cargoToml.package) name version;
			src = ./.;
			cargoLock = {
				lockFile = ./Cargo.lock;
				outputHashes = {
					"catppuccin-egui-3.1.0" = "sha256-oG2eFabEMEcxIFUOny6DrQbELpq72X7PQSrbq0Iwrqo=";
				};
			};

			desktopItem = pkgs.makeDesktopItem {
				name = "keyscreen";
				desktopName = "KeyScreen";
				exec = "keyscreen";
				categories = [ "Application" ];
			};

			nativeBuildInputs = with pkgs; [ pkg-config makeWrapper ];
			buildInputs = deps;
			postInstall = ''
				# Wrap binary
				mv $out/bin/keyscreen $out/bin/keyscreen-unwrapped
				makeWrapper $out/bin/keyscreen-unwrapped $out/bin/keyscreen \
					--set LD_LIBRARY_PATH ${LD_PATH}

				# Create desktop item
				mkdir -p $out/share/applications
				cp ${desktopItem}/share/applications/* $out/share/applications
			'';
		};
	in {
		packages.default = keyscreen;
		apps.default = utils.lib.mkApp {
			drv = keyscreen;
		};

		devShell = pkgs.mkShell rec {
			nativeBuildInputs = with pkgs; [ rustup pkg-config ];
			buildInputs = deps;
			LD_LIBRARY_PATH = LD_PATH;

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
