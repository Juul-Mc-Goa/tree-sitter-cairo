{
  description = "A devShell flake for nightly rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in with pkgs; {
        devShells.default = mkShell {
          buildInputs = [ eza pkg-config rust-bin.nightly.latest.default ];

          shellHook = ''
            alias ls=eza
            alias find=fd
            export PATH=$PATH:./node_modules/.bin
          '';
        };
      });
}
