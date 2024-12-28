{
  description = "Translate markdown into JIRA flavor";
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
		# Crate building utils 
		crane.url = "github:ipetkov/crane";
		crane.inputs.nixpkgs.follows = "nixpkgs";
		# Flake utils 
		flake-utils.url = "github:numtide/flake-utils";
		# Newer toolchain overlay
		rust-overlay.url = "github:oxalica/rust-overlay";
	};

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ...}: 
		flake-utils.lib.eachDefaultSystem (system: 
			let 
				overlays = [ (import rust-overlay) ];
				pkgs = import nixpkgs {inherit system overlays;};
				crn = crane.mkLib pkgs;
				common = {
					src = crn.cleanCargoSource ./.;
					# comes from oxalica/rust-overlay
					buildInputs = with pkgs; [rust-bin.stable.latest.default];
				}; 

				deps = crn.buildDepsOnly common;

				crate = crn.buildPackage (common // {
					inherit deps;
				});

				debug = crn.buildPackage (common // {
					inherit deps;
					cargoBuildCommand = "cargo build";
				});
			
			in {
				packages.default = crate;
				packages.debug = debug;

				devShells.default = pkgs.mkShell {
					buildInputs = common.buildInputs;
					shellHook = ''
						alias ll='ls -lah';
					'';
				};
				checks = {
					inherit crate;
				};
		});
}
