{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        {
          self,
          config,
          system,
          ...
        }:
        let
          overlays = [ inputs.rust-overlay.overlays.default ];
          pkgs = import inputs.nixpkgs {
            inherit system overlays;
          };
          rustToolchain = pkgs.rust-bin.stable."1.87.0".default.override {
            targets = [ "x86_64-pc-windows-msvc" ];
          };

          rust-toolchain = pkgs.symlinkJoin {
            name = "rust-toolchain";
            paths = [
              rustToolchain
              pkgs.cargo-watch
              pkgs.rust-analyzer
              pkgs.cargo-dist
              pkgs.cargo-tarpaulin
              pkgs.cargo-insta
              pkgs.cargo-machete
              pkgs.cargo-edit
            ];
          };
        in
        {
          # Rust dev environment
          devShells.default = pkgs.mkShell {
            inputsFrom = [
              config.treefmt.build.devShell
            ];
            RUST_BACKTRACE = "full";
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

            packages = [
              rust-toolchain
              pkgs.clippy
            ];
          };

          # Add your auto-formatters here.
          # cf. https://numtide.github.io/treefmt/
          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
              toml-sort.enable = true;
            };
          };
        };
    };
}
