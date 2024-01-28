{
  description = "A primary/seconday tiler for River WM.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rusttoolchain =
          pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
        sharedDeps = with pkgs; [ rusttoolchain pkg-config ];
      in
      rec {
        # `nix build`
        packages = {
          filtile = pkgs.rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            buildInputs = with pkgs;
              [ ];
            nativeBuildInputs = with pkgs;
              sharedDeps ++ [ ];
          };
          default = packages.filtile;
        };

        # `nix develop`
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs;
            sharedDeps ++ [ bacon openssl ];
        };

      });
}
